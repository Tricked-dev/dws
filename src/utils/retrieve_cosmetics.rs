use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    app_state::{Cosmetic, User},
    config::COSMETICS_FILE,
};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CosmeticFile {
    #[serde(default)]
    pub cosmetics: Vec<Cosmetic>,
    #[serde(default)]
    pub users: HashMap<Uuid, User>,
    #[serde(default)]
    pub irc_blacklist: HashSet<Uuid>,
}

pub async fn retrieve_cosmetics() -> CosmeticFile {
    if let Ok(file) = &tokio::fs::read_to_string(&*COSMETICS_FILE).await {
        serde_json::from_str(file).expect("Failed to parse cosmetics.json")
    } else {
        CosmeticFile::default()
    }
}
