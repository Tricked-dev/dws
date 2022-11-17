#![allow(clippy::single_match)]

use std::{
    env,
    sync::{atomic::AtomicU16, Arc},
};

use axum::{
    routing::{get, post},
    Router,
};
use futures_util::future::join;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use serenity::builder::CreateMessage;
use tokio::sync::broadcast as tokio_broadcast;
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
        set_ctrlc, uuid_to_username,
    },
};

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
    /* Initlized the CONFIG */
    Lazy::force(&CONFIG);
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "dws=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
    register().await?;
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

    let addr = format!("{host}:{port}", host = CONFIG.host, port = CONFIG.port)
        .parse()
        .unwrap();
    tracing::debug!("listening on {}", addr);
    let (r, _) = join(axum::Server::bind(&addr).serve(app.into_make_service()), async {
        while let Ok(msg) = rx.recv().await {
            match msg {
                InternalMessages::RequestUser {
                    user_id,
                    requester_id,
                    nonce,
                } => {
                    let is_online = app_state
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
                        let username = if let Ok(r) = uuid_to_username(sender).await {
                            r.name
                        } else {
                            continue;
                        };
                        channel
                            .send_message(&*REST, CreateMessage::new().content(format!("{username}: {}", message)))
                            .await
                            .unwrap();
                    }
                    None => {}
                },
                InternalMessages::RequestUsersBulk {
                    user_ids,
                    requester_id,
                    nonce,
                } => {
                    let users = app_state.users.lock();
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
            }
        }
    })
    .await;
    r?;
    Ok(())
}
