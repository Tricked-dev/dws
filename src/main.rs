#![allow(clippy::single_match)]

use std::{collections::HashSet, net::SocketAddr, sync::Arc};

use axum::{routing::get, Router};
use futures_util::future::join;
use parking_lot::Mutex;
use tokio::sync::broadcast;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{app_state::AppState, messages::InternalMessages, ws::ws_handler};

pub mod app_state;
pub mod error;
pub mod messages;
pub mod ws;

mod metrics;

pub use error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "dws=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let user_set = Mutex::new(HashSet::new());
    let (tx, mut rx) = broadcast::channel::<InternalMessages>(100);

    let app_state = Arc::new(AppState {
        user_set,
        tx: tx.clone(),
    });
    // build our application with some routes
    let app = Router::with_state(app_state.clone())
        .route("/metrics", get(metrics::metrics))
        .route("/ws", get(ws_handler));

    // run it with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    let (r, _) = join(axum::Server::bind(&addr).serve(app.into_make_service()), async {
        while let Ok(msg) = rx.recv().await {
            match msg {
                InternalMessages::RequestUser { user_id, requester_id } => {
                    let user_set = app_state.user_set.lock();
                    let is_online = user_set.contains(&user_id);
                    let msg = InternalMessages::UserRequestResponse {
                        is_online,
                        requester_id,
                        user_id,
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
