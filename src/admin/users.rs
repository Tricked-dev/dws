use std::sync::Arc;

use axum::{
    extract::{Json, Multipart, State},
    response::Redirect,
};
use serde::Deserialize;
use serenity::model::prelude::UserId;
use uuid::Uuid;

use crate::{
    app_state::{AppState, User},
    bitflags::CosmeticFlags,
};

#[derive(Deserialize)]
pub struct AddUser {
    pub uuid: Uuid,
    pub linked_discord: Option<UserId>,
    pub enabled_prefix: Option<u8>,
    pub irc_blacklisted: Option<bool>,
    pub flags: Option<CosmeticFlags>,
}

pub async fn add_user(State(state): State<Arc<AppState>>, Json(data): Json<AddUser>) -> &'static str {
    let mut users = state.users.lock();
    let def = users.get(&data.uuid).cloned().unwrap_or_default();
    users.insert(
        data.uuid,
        User {
            linked_discord: data.linked_discord.or(def.linked_discord),
            enabled_prefix: data.enabled_prefix.or(def.enabled_prefix),
            irc_blacklisted: data.irc_blacklisted.unwrap_or(def.irc_blacklisted),
            flags: data.flags.unwrap_or(def.flags),
            ..def
        },
    );
    "ok"
}
pub async fn remove_user(State(state): State<Arc<AppState>>, Json(data): Json<Uuid>) -> Redirect {
    let mut users = state.users.lock();
    users.remove(&data);
    Redirect::temporary("/")
}
