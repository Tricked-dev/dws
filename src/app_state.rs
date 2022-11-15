use std::{collections::HashMap, sync::atomic::AtomicU16};

use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use serenity::model::prelude::UserId;
use tokio::sync::broadcast;
use uuid::Uuid;

use crate::{bitflags::CosmeticFlags, messages::InternalMessages};

pub struct AppState {
    pub tx: broadcast::Sender<InternalMessages>,
    pub broadcast_secret: String,
    pub users: Mutex<HashMap<Uuid, User>>,
    pub cosmetics: Mutex<Vec<Cosmetic>>,
    pub messages_sec: AtomicU16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cosmetic {
    pub id: u8,
    pub name: String,
    pub description: String,
    pub data: String,
    #[serde(default, rename = "type")]
    pub type_field: u8,
    pub required_flags: CosmeticFlags,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct User {
    #[serde(default, skip_serializing_if = "CosmeticFlags::is_empty")]
    pub flags: CosmeticFlags,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled_prefix: Option<u8>,
    #[serde(default, skip)]
    pub connected: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_discord: Option<UserId>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub irc_blacklisted: bool,
}

fn is_false(b: &bool) -> bool {
    !b
}
