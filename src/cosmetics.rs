use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Json, State, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    response::IntoResponse,
};
use serde_json::json;
use uuid::Uuid;

use crate::{app_state::AppState, messages::InternalMessages};

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
