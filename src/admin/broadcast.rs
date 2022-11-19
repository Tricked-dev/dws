use std::sync::Arc;

use axum::extract::{Json, State};

use crate::{app_state::AppState, error::Result, messages::InternalMessages};

pub async fn broadcast(
    State(state): State<Arc<AppState>>,
    Json(message): Json<InternalMessages>,
) -> Result<&'static str> {
    if let InternalMessages::BroadCastMessage { message, to } = message {
        state
            .tx
            .send(InternalMessages::BroadCastMessage { message, to })
            .unwrap();
    };
    Ok("Ok")
}
