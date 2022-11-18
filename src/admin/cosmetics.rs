use std::sync::Arc;

use axum::extract::{Json, Query, State};
use serde::Deserialize;

use crate::{
    app_state::{AppState, Cosmetic},
    bitflags::CosmeticFlags,
};

#[derive(Deserialize)]
pub struct AddCosmetic {
    pub id: u8,
    pub name: String,
    pub description: String,
    pub data: String,
    #[serde(default, rename = "type")]
    pub type_field: u8,
    pub required_flags: CosmeticFlags,
}

#[derive(Deserialize)]
pub struct DeleteCosmetic {
    pub id: u8,
}

pub async fn add_cosmetic(State(state): State<Arc<AppState>>, Json(data): Json<AddCosmetic>) -> &'static str {
    let mut cosmetics = state.cosmetics.lock();
    cosmetics.push(Cosmetic {
        id: data.id,
        name: data.name,
        description: data.description,
        data: data.data,
        type_field: data.type_field,
        required_flags: data.required_flags,
    });

    "ok"
}

pub async fn remove_cosmetic(State(state): State<Arc<AppState>>, Query(data): Query<DeleteCosmetic>) -> &'static str {
    let mut cosmetics = state.cosmetics.lock();
    cosmetics.retain(|c| c.id != data.id);
    "ok"
}
