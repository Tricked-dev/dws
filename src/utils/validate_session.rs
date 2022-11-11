use cfg_if::cfg_if;
use serde::Deserialize;

use serde_json::Value;
use uuid::Uuid;

use crate::{error::Result, utils::uuid_utils::username_to_uuid};

use super::uuid_utils::UuidAndUsername;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Response {
    pub id: String,
    pub name: String,
}

/// Validates a minecraft session
#[allow(unused)]
pub async fn validate_session(server_id: String, username: String) -> Result<UuidAndUsername> {
    cfg_if! {
        if #[cfg(not(debug_assertions))] {
            validate_release(server_id, username).await
        } else {
            validate_debug(username).await
        }
    }
}
async fn validate_debug(username: String) -> Result<UuidAndUsername> {
    let uuid = username_to_uuid(username).await?;
    Ok(uuid)
}

#[allow(unused)]
async fn validate_release(server_id: String, username: String) -> Result<UuidAndUsername> {
    let r = reqwest::get(format!(
        "https://sessionserver.mojang.com/session/minecraft/hasJoined?username={}&serverId={}",
        username, server_id
    ))
    .await?
    .text()
    .await?;
    tracing::info!(r);
    let data: Response = serde_json::from_str(&r)?;
    let uuid = username_to_uuid(data.name).await?;
    Ok(uuid)
}
