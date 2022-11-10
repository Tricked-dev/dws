use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use uuid::Uuid;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "t", content = "c")]
pub enum Messages {
    #[serde(rename = "/is_online")]
    IsOnline { uuid: Uuid, nonce: Option<String> },
    #[serde(rename = "/is_online/bulk")]
    IsOnlineBulk { uuids: Vec<Uuid>, nonce: Option<String> },
    #[serde(rename = "/connect")]
    Connect(Uuid),
    #[serde(rename = "/is_online")]
    IsOnlineResponse {
        is_online: bool,
        uuid: Uuid,
        nonce: Option<String>,
    },
    #[serde(rename = "/is_online/bulk")]
    IsOnlineBulkResponse {
        users: HashMap<Uuid, bool>,
        nonce: Option<String>,
    },
    #[serde(rename = "/connected")]
    ConnectedResponse(bool),
    #[serde(rename = "/error")]
    Error { error: String, nonce: Option<String> },
    #[serde(rename = "/broadcast")]
    Broadcast(String),
    #[serde(rename = "/ping")]
    Ping(Option<String>),
    #[serde(rename = "/pong")]
    Pong(Option<String>),
    #[serde(rename = "/cosmetics/update")]
    CosmeticsUpdate {
        cosmetic_id: Option<u8>,
        nonce: Option<String>,
    },
    #[serde(rename = "/cosmetics/updated")]
    CosmeticsUpdated {
        cosmetic_id: Option<u8>,
        nonce: Option<String>,
    },
    #[serde(rename = "/cosmetics/ack")]
    CosmeticAck,
    #[serde(rename = "/irc/create")]
    IrcCreate { message: String },
    #[serde(rename = "/irc/created")]
    IrcCreated { message: String, sender: Uuid, date: u128 },
}
