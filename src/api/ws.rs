use std::{
    num::NonZeroU32,
    sync::{atomic::Ordering, Arc},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use futures_util::{SinkExt, StreamExt};
use governor::{Quota, RateLimiter};
use uuid::Uuid;

use crate::{
    app_state::AppState,
    config::RATELIMIT_PER_MINUTE,
    messages::{parse_ws_message, to_ws_message, InternalMessages, Messages},
    utils::{sanitize::sanitize_message, validate_session},
    Result,
};

pub async fn ws_handler(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    ws.on_upgrade(|socket| async move {
        if let Err(e) = handle_socket(socket, state).await {
            tracing::error!("Error handling socket: {:?}", e);
        };
    })
}

async fn handle_socket(stream: WebSocket, state: Arc<AppState>) -> Result<()> {
    let _start = Instant::now();
    let (mut sender, mut receiver) = stream.split();
    let mut uuid: Option<Uuid> = None;
    let mut name: Option<String> = None;
    let lim = RateLimiter::direct(*RATELIMIT_PER_MINUTE);
    let irclim =
        RateLimiter::direct(Quota::per_minute(NonZeroU32::new(4).unwrap()).allow_burst(NonZeroU32::new(8).unwrap()));

    while let Some(Ok(message)) = receiver.next().await {
        if let Message::Text(txt) = message {
            tracing::info!("{:?}", parse_ws_message(&txt));
            if let Some(Messages::Connect { server_id, username }) = parse_ws_message(&txt) {
                let data = validate_session(server_id, username).await?;
                uuid = Some(data.id);
                name = Some(data.name);
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
    let _name = match name {
        Some(name) => name,
        None => return Ok(()),
    };
    // Subscribe before sending joined message.
    let mut rx = state.tx.subscribe();

    // This task will receive broadcast messages and send text message to our client.
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            match msg {
                InternalMessages::UserInvalidJson { requester_id, error } => {
                    if requester_id == uuid {
                        let _ = sender.send(to_ws_message(Messages::Error { error, nonce: None })).await;
                    }
                }
                InternalMessages::UserError {
                    requester_id,
                    error,
                    nonce,
                } => {
                    if requester_id == uuid {
                        let _ = sender.send(to_ws_message(Messages::Error { error, nonce })).await;
                    }
                }
                InternalMessages::CosmeticsUpdate {
                    requester_id,
                    cosmetic_id,
                    nonce,
                } => {
                    if requester_id == uuid {
                        let _ = sender
                            .send(to_ws_message(Messages::CosmeticsUpdated { cosmetic_id, nonce }))
                            .await;
                    } else {
                        let _ = sender.send(to_ws_message(Messages::CosmeticAck)).await;
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
                InternalMessages::IrcCreate {
                    message,
                    sender: user,
                    date,
                } => {
                    let msg = Messages::IrcCreated {
                        message,
                        date,
                        sender: user,
                    };
                    let _ = sender.send(to_ws_message(msg)).await;
                }
                _ => {}
            }
        }
    });

    // Clone things we want to pass to the receiving task.
    let tx = state.tx.clone();

    // This task will receive messages from client and send them to broadcast subscribers.
    let state_clone = state.clone();
    let mut recv_task = tokio::spawn(async move {
        let state = state_clone;

        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            tracing::debug!("Add message/s: {}", state.messages_sec.fetch_add(1, Ordering::SeqCst));
            let state_clone = state.clone();
            tokio::spawn(async move {
                tokio::time::sleep(Duration::from_secs(1)).await;
                tracing::debug!(
                    "Removing value prev: {}",
                    state_clone.messages_sec.fetch_sub(1, Ordering::SeqCst)
                )
            });

            if let Err(e) = lim.check() {
                tracing::error!("Rate limit exceeded: {}", e);
                continue;
            }
            let msg = parse_ws_message(&text);
            tracing::debug!("{uuid} {}", text);
            match msg {
                Some(Messages::Connect { .. }) => {
                    let _ = tx.send(InternalMessages::UserInvalidJson {
                        requester_id: uuid,
                        error: "Already connected".to_owned(),
                    });
                }
                Some(Messages::Error { error, .. }) => {
                    let _ = tx.send(InternalMessages::UserInvalidJson {
                        requester_id: uuid,
                        error,
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
                Some(Messages::CosmeticsUpdate { cosmetic_id, nonce }) => {
                    let mut users = state.users.lock();
                    let user = users.get(&uuid);
                    let mut user = match user {
                        Some(user) => user.clone(),
                        None => {
                            let _ = tx.send(InternalMessages::UserError {
                                requester_id: uuid,
                                error: "You dont have any cosmetcs".to_owned(),
                                nonce,
                            });
                            continue;
                        }
                    };
                    user.enabled_prefix = if let Some(cosmetic_id) = cosmetic_id {
                        let cosmetics = state.cosmetics.lock();
                        let cosmetic = cosmetics.iter().find(|c| c.id == cosmetic_id);
                        let cosmetic = match cosmetic {
                            Some(cosmetic) => cosmetic,
                            None => {
                                let _ = tx.send(InternalMessages::UserError {
                                    requester_id: uuid,
                                    error: "Cosmetic not found".to_owned(),
                                    nonce,
                                });
                                continue;
                            }
                        };
                        if !user.flags.contains(cosmetic.required_flags) {
                            let _ = tx.send(InternalMessages::UserError {
                                requester_id: uuid,
                                error: "You dont have this cosmetics".to_owned(),
                                nonce,
                            });
                            continue;
                        }
                        Some(cosmetic_id)
                    } else {
                        None
                    };
                    users.insert(uuid, user);

                    let _ = tx.send(InternalMessages::CosmeticsUpdate {
                        cosmetic_id,
                        nonce,
                        requester_id: uuid,
                    });
                }
                Some(Messages::IrcCreate { message }) => {
                    if state.irc_blacklist.lock().contains(&uuid) {
                        continue;
                    }

                    if let Err(e) = irclim.check() {
                        tracing::error!("Rate limit exceeded: {}", e);
                        continue;
                    }

                    let _ = tx.send(InternalMessages::IrcCreate {
                        message: sanitize_message(&message),
                        sender: uuid,
                        date: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis(),
                    });
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
    tracing::info!("TOTAL: {}", state.user_set.lock().len());
    Ok(())
}
