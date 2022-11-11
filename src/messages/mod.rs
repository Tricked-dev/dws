use axum::extract::ws::Message;

pub use internal_messages::InternalMessages;
pub use websocket_messages::Messages;

mod internal_messages;
mod websocket_messages;

pub fn parse_ws_message(msg: &str) -> Option<Messages> {
    let msg = serde_json::from_str::<Messages>(msg);
    match msg {
        Ok(msg) => Some(msg),
        Err(e) => {
            tracing::error!("Error parsing message: {}", e);
            Some(Messages::Error {
                error: e.to_string(),
                nonce: None,
            })
        }
    }
}

pub fn to_ws_message(msg: Messages) -> Message {
    let msg = serde_json::to_string(&msg);
    match msg {
        Ok(msg) => Message::Text(msg),
        Err(e) => {
            tracing::error!("Error parsing message: {}", e);
            Message::Text(String::new())
        }
    }
}
