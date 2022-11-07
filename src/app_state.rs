use std::collections::{HashMap, HashSet};

use bitflags::bitflags;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize, Serializer};
use tokio::sync::broadcast;
use uuid::Uuid;

use crate::messages::InternalMessages;

pub struct AppState {
    pub user_set: Mutex<HashSet<Uuid>>,
    pub tx: broadcast::Sender<InternalMessages>,
    pub broadcast_secret: String,
    pub users: Mutex<HashMap<Uuid, User>>,
    pub cosmetics: Mutex<Vec<Cosmetic>>,
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

bitflags! {
    pub struct CosmeticFlags: u32 {
        const DEVELOPER = 0b00000001;
        const EARLY_USER = 0b00000010;
        const STAFF = 0b00000100;
    }
}

impl Default for CosmeticFlags {
    fn default() -> Self {
        Self::empty()
    }
}
/* source: https://github.com/novacrazy/serde_shims/blob/master/bitflags/src/lib.rs */
impl Serialize for CosmeticFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.bits().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for CosmeticFlags {
    fn deserialize<D>(deserializer: D) -> Result<CosmeticFlags, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <_ as Deserialize<'de>>::deserialize(deserializer)?;

        CosmeticFlags::from_bits(value).ok_or_else(|| {
            serde::de::Error::custom(format!("Invalid bits {:#X} for {}", value, stringify!(CosmeticFlags)))
        })
    }
}
