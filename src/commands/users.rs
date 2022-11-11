use std::sync::Arc;

use serenity::{
    builder::{CreateCommand, CreateInteractionResponseMessage},
    model::prelude::CommandInteraction,
};

use crate::app_state::AppState;

pub fn run(_: CommandInteraction, state: Arc<AppState>) -> CreateInteractionResponseMessage {
    CreateInteractionResponseMessage::new().content(format!("Connected users: {}", state.user_set.lock().len()))
}

pub fn register() -> CreateCommand {
    CreateCommand::new("users").description("Get the number of connected users")
}
