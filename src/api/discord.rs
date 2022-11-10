use std::sync::Arc;

use anyhow::anyhow;
use axum::{
    body::Bytes,
    extract::{Json, State},
    http::HeaderMap,
};
use once_cell::sync::Lazy;
use serde_json::Value;
use serenity::{builder::*, interactions_endpoint::Verifier, model::application::interaction::*};

use crate::{app_state::AppState, bail, commands::handle_command, config::DISCORD_PUBLIC_KEY, error::Result};

pub async fn handle_request(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    body_bytes: Bytes,
) -> Result<Json<Value>> {
    static VERIFIER: Lazy<Verifier> = Lazy::new(|| Verifier::new(&DISCORD_PUBLIC_KEY));
    let find_header = |name| headers.get(name).and_then(|v| v.to_str().ok());
    let signature = find_header("X-Signature-Ed25519").ok_or_else(|| anyhow!("missing signature header"))?;
    let timestamp = find_header("X-Signature-Timestamp").ok_or_else(|| anyhow!("missing timestamp header"))?;
    if VERIFIER.verify(signature, timestamp, &body_bytes).is_err() {
        bail!("invalid signature");
    }

    let response = match serde_json::from_slice::<Interaction>(&body_bytes).unwrap() {
        Interaction::Ping(_) => CreateInteractionResponse::Pong,
        Interaction::Command(interaction) => handle_command(interaction, state).await,
        _ => {
            bail!("unknown interaction type");
        }
    };

    Ok(Json(
        serde_json::from_slice(&serde_json::to_vec(&response).unwrap()).unwrap(),
    ))
}
