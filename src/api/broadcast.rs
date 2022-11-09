use std::sync::Arc;

use axum::{
    extract::{Json, State, TypedHeader},
    headers::{authorization::Bearer, Authorization},
};

use crate::{app_state::AppState, bail, error::Result, messages::InternalMessages};

pub async fn broadcast(
    State(state): State<Arc<AppState>>,
    TypedHeader(authorization): TypedHeader<Authorization<Bearer>>,
    Json(message): Json<InternalMessages>,
) -> Result<&'static str> {
    if authorization.token() != state.broadcast_secret {
        bail!("invalid broadcast secret");
    } else {
        if let InternalMessages::BroadCastMessage { message, to } = message {
            state
                .tx
                .send(InternalMessages::BroadCastMessage { message, to })
                .unwrap();
        };
        Ok("Ok")
    }
}
