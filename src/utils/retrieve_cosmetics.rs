use std::{collections::HashMap, env};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::app_state::{Cosmetic, User};

pub static COSMETIC_FILE: Lazy<String> =
    Lazy::new(|| env::var("COSMETICS_FILE").unwrap_or_else(|_| "cosmetics.json".to_owned()));

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CosmeticFile {
    pub cosmetics: Vec<Cosmetic>,
    pub users: HashMap<Uuid, User>,
}

pub async fn retrieve_cosmetics() -> CosmeticFile {
    if let Ok(file) = &tokio::fs::read_to_string(&*COSMETIC_FILE).await {
        serde_json::from_str(file).expect("Failed to parse cosmetics.json")
    } else {
        CosmeticFile::default()
    }
}
