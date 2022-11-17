use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    app_state::{Cosmetic, User},
    config::CONFIG,
};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CosmeticFile {
    #[serde(default)]
    pub cosmetics: Vec<Cosmetic>,
    #[serde(default)]
    pub users: HashMap<Uuid, User>,
}

pub async fn retrieve_cosmetics() -> CosmeticFile {
    if let Ok(file) = &tokio::fs::read_to_string(&CONFIG.cosmetics_file).await {
        serde_json::from_str(file).expect("Failed to parse cosmetics.json")
    } else {
        CosmeticFile::default()
    }
}
