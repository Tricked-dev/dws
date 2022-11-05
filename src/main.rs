//! Example websocket server.
//!
//! Run with
//!
//! ```not_rust
//! cd examples && cargo run -p example-websockets
//! ```

use axum::extract::ws::{Message, WebSocket};
use axum::extract::{State, WebSocketUpgrade};
use axum::Router;
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, get_service},
};
use futures::{sink::SinkExt, stream::StreamExt};
use futures_util::future::join;
use once_cell::sync::Lazy;
use std::sync::Mutex;
// use parking_lot::Mutex;
use crate::messages::{InternalMessages, Messages, Responses};
use std::{
    collections::{HashMap, HashSet},
    net::SocketAddr,
    path::PathBuf,
    sync::Arc,
    time::Instant,
};
use tokio::join;
use tokio::sync::broadcast;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

pub mod messages;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "dws=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");

    let user_set = Mutex::new(HashSet::new());
    let (tx, mut rx) = broadcast::channel::<InternalMessages>(100);
    let app_state = Arc::new(AppState {
        user_set,
        tx: tx.clone(),
    });
    let st = app_state.clone();
    // build our application with some routes
    let app = Router::with_state(app_state).route("/ws", get(ws_handler));

    // run it with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    join(
        axum::Server::bind(&addr).serve(app.into_make_service()),
        async {
            while let Ok(msg) = rx.recv().await {
                match msg {
                    InternalMessages::RequestUser {
                        user_id,
                        requester_id,
                    } => {
                        let mut user_set = st.user_set.lock().unwrap();
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
        },
    )
    .await;
}

struct AppState {
    user_set: Mutex<HashSet<Uuid>>,
    tx: broadcast::Sender<InternalMessages>,
}

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

fn parse_ws_message(msg: &str) -> Option<Messages> {
    let msg = serde_json::from_str::<Messages>(msg);
    match msg {
        Ok(msg) => Some(msg),
        Err(e) => {
            tracing::error!("Error parsing message: {}", e);
            None
        }
    }
}

fn parse_internal_message(msg: &str) -> Option<InternalMessages> {
    let msg = serde_json::from_str::<InternalMessages>(msg);
    match msg {
        Ok(msg) => Some(msg),
        Err(e) => {
            tracing::error!("Error parsing message: {}", e);
            None
        }
    }
}

fn to_internal_message(msg: &InternalMessages) -> String {
    serde_json::to_string(msg).unwrap()
}

fn to_ws_message(msg: Responses) -> Message {
    let msg = serde_json::to_string(&msg);
    match msg {
        Ok(msg) => Message::Text(msg),
        Err(e) => {
            tracing::error!("Error parsing message: {}", e);
            Message::Text(String::new())
        }
    }
}

async fn handle_socket(stream: WebSocket, state: Arc<AppState>) {
    let start = Instant::now();
    let (mut sender, mut receiver) = stream.split();
    let mut uuid: Option<Uuid> = None;
    while let Some(Ok(message)) = receiver.next().await {
        if let Message::Text(txt) = message {
            println!(
                "{}",
                serde_json::to_string_pretty(&Messages::Connect(Uuid::new_v4())).unwrap()
            );
            println!("{:?}", parse_ws_message(&txt));
            if let Some(Messages::Connect(uid)) = parse_ws_message(&txt) {
                let mut user_set = state.user_set.lock().unwrap();
                user_set.insert(uid);
                uuid = Some(uid);
                break;
            }
        }
    }

    sender
        .send(to_ws_message(Responses::Connected(true)))
        .await
        .unwrap();
    let uuid = match uuid {
        Some(uuid) => uuid,
        None => return,
    };

    // Subscribe before sending joined message.
    let mut rx = state.tx.subscribe();

    // Send joined message to all subscribers.
    let msg = format!("{} joined.", uuid);

    // This task will receive broadcast messages and send text message to our client.
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            match msg {
                InternalMessages::UserRequestResponse {
                    is_online,
                    requester_id,
                    user_id,
                } => {
                    let msg = Responses::IsOnline {
                        is_online,
                        uuid: user_id,
                    };
                    let _ = sender.send(to_ws_message(msg)).await;
                }
                _ => {}
            }
            // if sender.send(Message::Text(msg)).await.is_err() {
            //     tracing::debug!("websocket send error");
            //     break;
            // }
        }
        println!("HI!")
    });

    // Clone things we want to pass to the receiving task.
    let tx = state.tx.clone();
    let uuid = uuid.clone();

    // This task will receive messages from client and send them to broadcast subscribers.
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            let msg = parse_ws_message(&text);
            tracing::debug!("{}", text);
            match msg {
                Some(Messages::IsOnline(user_uuid)) => {
                    // let user_set = state.user_set.lock().unwrap();
                    // let is_online = user_set.contains(&uuid);
                    let _ = tx.send(InternalMessages::RequestUser {
                        user_id: user_uuid,
                        requester_id: uuid,
                    });
                }
                Some(Messages::Disconnect(reason)) => {
                    let msg = format!("{} disconnected.", uuid);
                    tracing::debug!("{}", msg);
                    // let _ = tx.send(msg);
                    break;
                }
                _ => {}
            }
            // let msg = format!("{}: {}", uuid, text);
            // Add username before message.
            // let _ = tx.send(format!("{}: {}", name, text));
        }
        println!("websocket connection closed");
    });

    // If any one of the tasks exit, abort the other.
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };

    // Send user left message.
    let msg = format!("{} left.", uuid);
    tracing::debug!("{}", msg);
    // let _ = state.tx.send(msg);
    // Remove username from map so new clients can take it.
    state.user_set.lock().unwrap().remove(&uuid);
}

// async fn handle_socket(mut socket: WebSocket<Messages, Responses>) {
//     let mut uuid: Option<Uuid> = None;
//     if let Some(msg) = socket.recv().await {
//         if let Ok(msg) = msg {
//             match msg {
//                 Message::Text(t) => {
//                     let msg = serde_json::from_str::<Messages>(&t);
//                     match msg {
//                         Ok(msg) => {
//                             let mut users = SESSIONS.users.lock();
//                             match msg {
//                                 Messages::Connect(uid) => {
//                                     if !uuid.is_none() {
//                                         let response = serde_json::to_string(&Responses::Error(
//                                             "Already connected".to_string(),
//                                         ))
//                                         .unwrap();
//                                         socket.send(Message::Text(response)).await.unwrap();
//                                     } else if users.contains(&uid) {
//                                         socket
//                                             .send(Message::Text(
//                                                 serde_json::to_string(&Responses::Error(
//                                                     "Already connected".to_string(),
//                                                 ))
//                                                 .unwrap(),
//                                             ))
//                                             .await
//                                             .unwrap();
//                                     } else {
//                                         uuid = Some(uid.clone());
//                                         users.push(uid);
//                                         socket
//                                             .send(Message::Text(
//                                                 serde_json::to_string(&Responses::Connected(true))
//                                                     .unwrap(),
//                                             ))
//                                             .await
//                                             .unwrap();
//                                     }
//                                 }
//                                 Messages::IsOnline(uuid) => {
//                                     if users.contains(&uuid) {
//                                         let response =
//                                             serde_json::to_string(&Responses::GetUser(true))
//                                                 .unwrap();
//                                         socket.send(Message::Text(response)).await.unwrap();
//                                     } else {
//                                         let response =
//                                             serde_json::to_string(&Responses::GetUser(false))
//                                                 .unwrap();
//                                         socket.send(Message::Text(response)).await.unwrap();
//                                     }
//                                 }
//                             }
//                             println!("Received message: {:?}", msg);
//                             socket.send(Message::Text(t)).await.unwrap();
//                         }
//                         Err(e) => {
//                             println!("Error: {:?}", e);
//                         }
//                     }
//                 }
//                 Message::Binary(_) => {
//                     println!("client sent binary data");
//                 }
//                 Message::Ping(_) => {
//                     println!("socket ping");
//                 }
//                 Message::Pong(_) => {
//                     println!("socket pong");
//                 }
//                 Message::Close(_) => {
//                     println!("client disconnected");
//                     return;
//                 }
//             }
//         } else {
//             println!("client disconnected");
//             return;
//         }
//     }

//     loop {
//         if socket
//             .send(Message::Text(String::from("Hi!")))
//             .await
//             .is_err()
//         {
//             println!("client disconnected");
//             return;
//         }
//         tokio::time::sleep(std::time::Duration::from_secs(3)).await;
//     }
// }
