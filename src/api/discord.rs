use std::sync::Arc;

use axum::{
    body::Bytes,
    extract::{Json, State},
    http::HeaderMap,
    response::IntoResponse,
};
use once_cell::sync::Lazy;
use serde_json::json;
use serenity::{builder::*, interactions_endpoint::Verifier, model::application::interaction::*};

use crate::{app_state::AppState, commands::handle_command, config::DISCORD_PUBLIC_KEY};

pub async fn handle_request(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    body_bytes: Bytes,
) -> impl IntoResponse {
    static VERIFIER: Lazy<Verifier> = Lazy::new(|| Verifier::new(&DISCORD_PUBLIC_KEY));
    let find_header = |name| headers.get(name).and_then(|v| v.to_str().ok());
    let signature = find_header("X-Signature-Ed25519")
        .ok_or("missing signature header")
        .unwrap();
    let timestamp = find_header("X-Signature-Timestamp")
        .ok_or("missing timestamp header")
        .unwrap();
    if VERIFIER.verify(signature, timestamp, &body_bytes).is_err() {
        return Json(json!({
            "status": 401,
            "message": "Invalid request signature"
        }));
    }

    // Build Discord response
    let response = match serde_json::from_slice::<Interaction>(&body_bytes).unwrap() {
        // Discord rejects the interaction endpoints URL if pings are not acknowledged
        Interaction::Ping(_) => CreateInteractionResponse::Pong,
        Interaction::Command(interaction) => handle_command(interaction, state),
        _ => {
            return Json(json!( {
                "error": "Unknown interaction type"
            }))
        }
    };

    Json(serde_json::from_slice(&serde_json::to_vec(&response).unwrap()).unwrap())
}
