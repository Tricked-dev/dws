use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use uuid::Uuid;

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
    UserInvalidJson {
        requester_id: Uuid,
        error: String,
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
    CosmeticsUpdate {
        requester_id: Uuid,
        cosmetic_id: Option<u8>,
        nonce: Option<String>,
    },
    UserError {
        requester_id: Uuid,
        error: String,
        nonce: Option<String>,
    },
    IrcCreate {
        message: String,
        sender: Uuid,
        date: u128,
    },
}
