use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Json, State},
    headers::{authorization::Bearer, Authorization},
    response::IntoResponse,
    TypedHeader,
};
use serde_json::json;
use uuid::Uuid;

use crate::{app_state::AppState, bail, error::Result, utils::retrieve_cosmetics::retrieve_cosmetics};

pub async fn cosmetics(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let f = state.users.lock();

    let users_cosmetic_map = f
        .iter()
        .filter_map(|(uuid, user)| {
            if user.enabled_prefix.is_some() {
                Some((uuid, user.enabled_prefix.as_ref().unwrap()))
            } else {
                None
            }
        })
        .collect::<HashMap<&Uuid, &u8>>();

    Json(json!({
        "cosmetics": state.cosmetics,
        "users": users_cosmetic_map
    }))
}

pub async fn force_update(
    State(state): State<Arc<AppState>>,
    TypedHeader(authorization): TypedHeader<Authorization<Bearer>>,
) -> Result<&'static str> {
    if authorization.token() != state.broadcast_secret {
        bail!("invalid broadcast secret");
    } else {
        println!("Updating cosmetics");
        let cosmetics = retrieve_cosmetics().await;
        state.cosmetics.lock().clone_from(&cosmetics.cosmetics);
        state.users.lock().clone_from(&cosmetics.users);
        Ok("Ok")
    }
}
