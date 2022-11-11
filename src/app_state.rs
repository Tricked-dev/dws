use std::collections::{HashMap, HashSet};

use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;
use uuid::Uuid;

use crate::{bitflags::CosmeticFlags, messages::InternalMessages};

pub struct AppState {
    pub user_set: Mutex<HashSet<Uuid>>,
    pub tx: broadcast::Sender<InternalMessages>,
    pub broadcast_secret: String,
    pub users: Mutex<HashMap<Uuid, User>>,
    pub cosmetics: Mutex<Vec<Cosmetic>>,
    pub irc_blacklist: Mutex<HashSet<Uuid>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cosmetic {
    pub id: u8,
    pub name: String,
    pub display: String,
    pub description: String,
    pub required_flags: CosmeticFlags,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct User {
    pub flags: CosmeticFlags,
    pub enabled_prefix: Option<u8>,
}
