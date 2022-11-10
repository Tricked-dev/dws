use serde::Deserialize;
use uuid::Uuid;

use crate::error::Result;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct UuidToUsername {
    pub name: String,
}

pub async fn uuid_to_username(uuid: Uuid) -> Result<UuidToUsername> {
    println!("https://api.mojang.com/user/profile/{}", uuid.as_simple());
    let result = serde_json::from_slice(
        &reqwest::get(&format!("https://api.mojang.com/user/profile/{}", uuid.as_simple()))
            .await?
            .bytes()
            .await?,
    )?;
    Ok(result)
}
