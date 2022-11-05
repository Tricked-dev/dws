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
