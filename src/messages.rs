use std::collections::HashMap;

use axum::extract::ws::Message;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "t", content = "c")]
pub enum Messages {
    #[serde(rename = "/is_online")]
    IsOnline { uuid: Uuid, nonce: Option<String> },
    #[serde(rename = "/is_online/bulk")]
    IsOnlineBulk { uuids: Vec<Uuid>, nonce: Option<String> },
    #[serde(rename = "/connect")]
    Connect(Uuid),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "t", content = "c")]
pub enum Responses {
    #[serde(rename = "/is_online")]
    IsOnline {
        is_online: bool,
        uuid: Uuid,
        nonce: Option<String>,
    },
    #[serde(rename = "/is_online/bulk")]
    IsOnlineBulk {
        users: HashMap<Uuid, bool>,
        nonce: Option<String>,
    },
    #[serde(rename = "/connected")]
    Connected(bool),
    #[serde(rename = "/error")]
    Error(String),
    #[serde(rename = "/broadcast")]
    Broadcast(String),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum InternalMessages {
    RequestUser {
        user_id: Uuid,
        requester_id: Uuid,
        nonce: Option<String>,
    },
    RequestUsersBulk {
        user_ids: Vec<Uuid>,
        requester_id: Uuid,
        nonce: Option<String>,
    },
    UserRequestResponse {
        requester_id: Uuid,
        is_online: bool,
        user_id: Uuid,
        nonce: Option<String>,
    },
    UserRequestBulkResponse {
        requester_id: Uuid,
        users: HashMap<Uuid, bool>,
        nonce: Option<String>,
    },
    BroadCastMessage {
        // Minecraft Chat Codes
        message: String,
        to: Vec<Uuid>,
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
