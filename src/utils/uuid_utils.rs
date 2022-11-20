use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::Result;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UuidAndUsername {
    pub name: String,
    pub id: Uuid,
}

pub async fn uuid_to_username(uuid: Uuid) -> Result<UuidAndUsername> {
    let result = serde_json::from_slice(
        &reqwest::get(&format!("https://api.mojang.com/user/profile/{}", uuid.as_simple()))
            .await?
            .bytes()
            .await?,
    )?;
    Ok(result)
}

pub async fn username_to_uuid(username: String) -> Result<UuidAndUsername> {
    let result = serde_json::from_slice(
        &reqwest::get(&format!("https://api.mojang.com/users/profiles/minecraft/{}", username))
            .await?
            .bytes()
            .await?,
    )?;
    Ok(result)
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct SlothUser {
    pub uuid: Uuid,
    pub links: Links,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Links {
    #[serde(rename = "DISCORD")]
    pub discord: String,
}

pub async fn username_to_uuid_and_discord(username: &str) -> Result<SlothUser> {
    let result = serde_json::from_slice(
        &reqwest::get(&format!("https://api.slothpixel.me/api/players/{username}"))
            .await?
            .bytes()
            .await?,
    )?;
    Ok(result)
}
