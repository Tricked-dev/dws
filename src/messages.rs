use std::collections::HashMap;

use axum::extract::ws::Message;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "t", content = "c")]
pub enum Messages {
    #[serde(rename = "/is_online")]
    IsOnline {
        uuid: Uuid,
        #[serde(skip_serializing_if = "Option::is_none")]
        nonce: Option<String>,
    },
    #[serde(rename = "/is_online/bulk")]
    IsOnlineBulk {
        uuids: Vec<Uuid>,
        #[serde(skip_serializing_if = "Option::is_none")]
        nonce: Option<String>,
    },
    #[serde(rename = "/connect")]
    Connect(Uuid),
    #[serde(rename = "/is_online")]
    IsOnlineResponse {
        is_online: bool,
        uuid: Uuid,
        #[serde(skip_serializing_if = "Option::is_none")]
        nonce: Option<String>,
    },
    #[serde(rename = "/is_online/bulk")]
    IsOnlineBulkResponse {
        users: HashMap<Uuid, bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        nonce: Option<String>,
    },
    #[serde(rename = "/connected")]
    ConnectedResponse(bool),
    #[serde(rename = "/error")]
    Error(String),
    #[serde(rename = "/broadcast")]
    Broadcast(String),
    #[serde(rename = "/ping")]
    Ping(#[serde(skip_serializing_if = "Option::is_none")] Option<String>),
    #[serde(rename = "/pong")]
    Pong(#[serde(skip_serializing_if = "Option::is_none")] Option<String>),
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
    Pong {
        nonce: Option<String>,
        uuid: Uuid,
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
