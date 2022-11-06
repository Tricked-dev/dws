use std::{sync::Arc, time::Instant};

use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use futures_util::{SinkExt, StreamExt};
use uuid::Uuid;

use crate::{
    app_state::AppState,
    messages::{parse_ws_message, to_ws_message, InternalMessages, Messages},
    Result,
};

pub async fn ws_handler(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    ws.on_upgrade(|socket| async move {
        if let Err(e) = handle_socket(socket, state).await {
            tracing::error!("Error handling socket: {}", e);
        };
    })
}

async fn handle_socket(stream: WebSocket, state: Arc<AppState>) -> Result<()> {
    let _start = Instant::now();
    let (mut sender, mut receiver) = stream.split();
    let mut uuid: Option<Uuid> = None;
    while let Some(Ok(message)) = receiver.next().await {
        if let Message::Text(txt) = message {
            tracing::info!("{:?}", parse_ws_message(&txt));
            if let Some(Messages::Connect(uid)) = parse_ws_message(&txt) {
                let mut user_set = state.user_set.lock();
                user_set.insert(uid);
                uuid = Some(uid);
                break;
            }
        }
    }

    if let Err(e) = sender.send(to_ws_message(Messages::ConnectedResponse(true))).await {
        tracing::error!("Error sending message: {}", e);
        return Ok(());
    }
    let uuid = match uuid {
        Some(uuid) => uuid,
        None => return Ok(()),
    };
    // Subscribe before sending joined message.
    let mut rx = state.tx.subscribe();

    // Send joined message to all subscribers.
    let _msg = format!("{} joined.", uuid);

    // This task will receive broadcast messages and send text message to our client.
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            match msg {
                InternalMessages::UserInvalidJson { requester_id, error } => {
                    if requester_id == uuid {
                        let _ = sender.send(to_ws_message(Messages::Error(error))).await;
                    }
                }
                InternalMessages::UserRequestResponse {
                    is_online,
                    requester_id,
                    user_id,
                    nonce,
                } => {
                    if requester_id != uuid {
                        continue;
                    }
                    let msg = Messages::IsOnlineResponse {
                        is_online,
                        uuid: user_id,
                        nonce,
                    };
                    let _ = sender.send(to_ws_message(msg)).await;
                }
                InternalMessages::UserRequestBulkResponse {
                    requester_id,
                    users,
                    nonce,
                } => {
                    if requester_id != uuid {
                        continue;
                    }
                    let msg = Messages::IsOnlineBulkResponse { users, nonce };
                    let _ = sender.send(to_ws_message(msg)).await;
                }
                InternalMessages::BroadCastMessage { message, to } => {
                    if to.contains(&uuid) || to.is_empty() {
                        let msg = Messages::Broadcast(message);
                        let _ = sender.send(to_ws_message(msg)).await;
                    }
                }
                InternalMessages::Pong {
                    nonce,
                    uuid: requester_id,
                } => {
                    if requester_id != uuid {
                        continue;
                    }
                    let _ = sender.send(to_ws_message(Messages::Pong(nonce))).await;
                }
                _ => {}
            }
        }
        println!("HI!")
    });

    // Clone things we want to pass to the receiving task.
    let tx = state.tx.clone();

    // This task will receive messages from client and send them to broadcast subscribers.
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            let msg = parse_ws_message(&text);
            tracing::debug!("{}", text);
            match msg {
                Some(Messages::Connect(_)) => {
                    let _ = tx.send(InternalMessages::UserInvalidJson {
                        requester_id: uuid,
                        error: "Already connected".to_owned(),
                    });
                }
                Some(Messages::Error(e)) => {
                    let _ = tx.send(InternalMessages::UserInvalidJson {
                        requester_id: uuid,
                        error: e,
                    });
                }
                Some(Messages::IsOnline { uuid: user_id, nonce }) => {
                    let _ = tx.send(InternalMessages::RequestUser {
                        user_id,
                        requester_id: uuid,
                        nonce,
                    });
                }
                Some(Messages::IsOnlineBulk { uuids, nonce }) => {
                    let _ = tx.send(InternalMessages::RequestUsersBulk {
                        user_ids: uuids,
                        requester_id: uuid,
                        nonce,
                    });
                }
                Some(Messages::Ping(nonce)) => {
                    let _ = tx.send(InternalMessages::Pong { nonce, uuid });
                }

                _ => {}
            }
        }
    });

    // If any one of the tasks exit, abort the other.
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };

    tracing::debug!("{} disconnected from the website", uuid,);
    state.user_set.lock().remove(&uuid);
    println!("{:?}", state.user_set.lock());
    Ok(())
}
