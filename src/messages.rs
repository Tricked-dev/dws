use axum::extract::ws::Message;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "t", content = "c")]
pub enum Messages {
    #[serde(rename = "/is_online")]
    IsOnline(Uuid),
    #[serde(rename = "/connect")]
    Connect(Uuid),
    #[serde(rename = "/disconnect")]
    Disconnect(i32),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "status", content = "c")]
pub enum Responses {
    #[serde(rename = "200")]
    IsOnline { is_online: bool, uuid: Uuid },
    #[serde(rename = "200")]
    Connected(bool),
    #[serde(rename = "400")]
    Error(String),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum InternalMessages {
    RequestUser {
        user_id: Uuid,
        requester_id: Uuid,
    },
    UserRequestResponse {
        requester_id: Uuid,
        is_online: bool,
        user_id: Uuid,
    },
}

pub fn parse_ws_message(msg: &str) -> Option<Messages> {
    let msg = serde_json::from_str::<Messages>(msg);
    match msg {
        Ok(msg) => Some(msg),
        Err(e) => {
            tracing::error!("Error parsing message: {}", e);
            None
        }
    }
}

pub fn to_ws_message(msg: Responses) -> Message {
    let msg = serde_json::to_string(&msg);
    match msg {
        Ok(msg) => Message::Text(msg),
        Err(e) => {
            tracing::error!("Error parsing message: {}", e);
            Message::Text(String::new())
        }
    }
}
