use std::sync::Arc;

use axum::{
    body::Bytes,
    extract::{Json, State, TypedHeader},
    headers::{
        self,
        authorization::{self, Bearer, Credentials},
        Authorization,
    },
    response::IntoResponse,
};

use crate::{app_state::AppState, messages::InternalMessages};

pub async fn broadcast(
    State(state): State<Arc<AppState>>,
    TypedHeader(authorization): TypedHeader<Authorization<Bearer>>,
    Json(message): Json<InternalMessages>,
) -> impl IntoResponse {
    if authorization.token() != state.broadcast_secret {
        "Invalid token"
    } else {
        if let InternalMessages::BroadCastMessage { message, to } = message {
            state
                .tx
                .send(InternalMessages::BroadCastMessage { message, to })
                .unwrap();
        };
        "Ok"
    }
}