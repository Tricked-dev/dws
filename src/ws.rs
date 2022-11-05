use std::{sync::Arc, time::Instant};

use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt};
use uuid::Uuid;

use crate::{
    app_state::AppState,
    messages::{parse_ws_message, to_ws_message, InternalMessages, Messages, Responses},
};

pub async fn ws_handler(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(stream: WebSocket, state: Arc<AppState>) {
    let _start = Instant::now();
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
                let mut user_set = state.user_set.lock();
                user_set.insert(uid);
                uuid = Some(uid);
                break;
            }
        }
    }

    sender.send(to_ws_message(Responses::Connected(true))).await.unwrap();
    let uuid = match uuid {
        Some(uuid) => uuid,
        None => return,
    };

    // Subscribe before sending joined message.
    let mut rx = state.tx.subscribe();

    // Send joined message to all subscribers.
    let _msg = format!("{} joined.", uuid);

    // This task will receive broadcast messages and send text message to our client.
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            match msg {
                InternalMessages::UserRequestResponse {
                    is_online,
                    requester_id: _,
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
                Some(Messages::Disconnect(_reason)) => {
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
    state.user_set.lock().remove(&uuid);
    println!("{:?}", state.user_set.lock());
}
