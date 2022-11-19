#![allow(clippy::single_match)]

use std::{
    env,
    sync::{atomic::AtomicU16, Arc},
    time::Duration,
};

use axum::{
    routing::{delete, get, post},
    Router,
};
use futures_util::future::join3;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use serenity::builder::CreateMessage;
use tokio::{
    sync::broadcast::{self as tokio_broadcast, Sender},
    time::sleep,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    api::*,
    app_state::AppState,
    commands::{register, REST},
    config::CONFIG,
    error::Result,
    messages::InternalMessages,
    utils::{
        retrieve_cosmetics::{retrieve_cosmetics, CosmeticFile},
        set_ctrlc, uuid_to_username, Influx,
    },
};

pub mod admin;
pub mod app_state;
pub mod bitflags;
pub mod cli;
pub mod commands;
pub mod config;
pub mod error;
pub mod messages;
pub mod utils;

mod api;
mod source;

#[tokio::main]
async fn main() -> Result<()> {
    if env::var("WRITE_SOURCES").is_ok() {
        source::write_sources();
    }

    // Load config
    Lazy::force(&CONFIG);

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "dws=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // registers the commands
    tokio::spawn(async {
        // Register the commands after 60 seconds so to not spam the api when developing
        sleep(Duration::from_secs(60)).await;
        if let Err(e) = register().await {
            tracing::error!("Error registering commands: {:?}", e);
        }
    });

    let (tx, mut rx) = tokio_broadcast::channel::<InternalMessages>(100);

    let cosmetics = retrieve_cosmetics().await;

    let app_state = Arc::new(AppState {
        tx: tx.clone(),
        broadcast_secret: CONFIG.api_secret.clone(),
        cosmetics: Mutex::new(cosmetics.cosmetics),
        users: Mutex::new(cosmetics.users),
        messages_sec: AtomicU16::new(0),
    });

    set_ctrlc(app_state.clone())?;

    let app_state_clone = app_state.clone();

    // saves the cosmetics every 5 minutes
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(300));

        loop {
            interval.tick().await;

            let file = CosmeticFile {
                cosmetics: app_state_clone.cosmetics.lock().clone(),
                users: app_state_clone.users.lock().clone(),
            };
            tokio::fs::write(&CONFIG.cosmetics_file, serde_json::to_string_pretty(&file).unwrap())
                .await
                .expect("Failed to write cosmetics file");
        }
    });

    // build our application with some routes
    let app = Router::with_state(app_state.clone())
        .route("/metrics", get(metrics::metrics))
        .route("/broadcast", post(broadcast::broadcast))
        .route("/cosmetics", get(cosmetics::cosmetics))
        .route("/cosmetics", post(cosmetics::force_update))
        .route("/discord", post(discord::handle_request))
        .route("/ws", get(ws::ws_handler));

    let admin = if CONFIG.admin_dash {
        Router::with_state(app_state.clone())
            .route("/", get(admin::load_admin))
            .route("/users", get(admin::users::get_users))
            .route("/users", post(admin::users::add_user))
            .route("/users", delete(admin::users::remove_user))
            .route("/cosmetics", get(admin::cosmetics::get_cosmetics))
            .route("/cosmetics", post(admin::cosmetics::add_cosmetic))
            .route("/cosmetics", delete(admin::cosmetics::remove_cosmetic))
    } else {
        Router::with_state(app_state.clone())
    };

    let addr = format!("{host}:{port}", host = CONFIG.host, port = CONFIG.port).parse()?;
    let admin_addr = format!(
        "{host}:{port}",
        host = CONFIG.host,
        port = CONFIG.admin_port.unwrap_or(CONFIG.port + 1)
    )
    .parse()?;
    tracing::debug!("listening on http://{}", addr);
    tracing::debug!("admin listening on http://{}", admin_addr);
    let (r, r2, _) = join3(
        axum::Server::bind(&addr).serve(app.into_make_service()),
        axum::Server::bind(&admin_addr).serve(admin.into_make_service()),
        async {
            while let Ok(msg) = rx.recv().await {
                if let Err(e) = handle_internal(msg, &app_state, &tx).await {
                    tracing::error!("Error handling internal message: {:?}", e);
                }
            }
        },
    )
    .await;
    r?;
    r2?;
    Ok(())
}

async fn handle_internal(msg: InternalMessages, state: &Arc<AppState>, tx: &Sender<InternalMessages>) -> Result<()> {
    match msg {
        InternalMessages::RequestUser {
            user_id,
            requester_id,
            nonce,
        } => {
            tokio::spawn(
                Influx::new("request_online")
                    .label("uuid", &requester_id.to_string())
                    .value("user", &user_id.to_string())
                    .send(),
            );
            let is_online = state
                .users
                .lock()
                .get(&user_id)
                .map(|x| x.connected)
                .unwrap_or_default();

            let msg = InternalMessages::UserRequestResponse {
                is_online,
                requester_id,
                user_id,
                nonce,
            };
            let _ = tx.send(msg);
        }
        InternalMessages::IrcCreate {
            message,
            sender,
            date: _,
        } => match CONFIG.discord_irc_channel {
            Some(channel) => {
                let data = uuid_to_username(sender).await?;
                channel
                    .send_message(
                        &*REST,
                        CreateMessage::new().content(format!("{}: {}", data.name, message,)),
                    )
                    .await
                    .unwrap();
                Influx::new("irc_message").label("uuid", &sender.to_string()).await?;
            }
            None => {}
        },
        InternalMessages::RequestUsersBulk {
            user_ids,
            requester_id,
            nonce,
        } => {
            tokio::spawn(
                Influx::new("bulk_request_online")
                    .label("uuid", &requester_id.to_string())
                    .send(),
            );
            let users = state.users.lock();
            let list = user_ids
                .into_iter()
                .map(|user_id| {
                    let is_online = users.get(&user_id).map(|x| x.connected).unwrap_or_default();
                    (user_id, is_online)
                })
                .collect();
            let msg = InternalMessages::UserRequestBulkResponse {
                users: list,
                requester_id,
                nonce,
            };
            let _ = tx.send(msg);
        }
        _ => {}
    };
    Ok(())
}
