use std::sync::Arc;

use serenity::{
    builder::{CreateCommand, CreateInteractionResponseMessage},
    model::prelude::CommandInteraction,
};

use crate::app_state::AppState;

pub fn run(_cmd: CommandInteraction, _state: Arc<AppState>) -> CreateInteractionResponseMessage {
    CreateInteractionResponseMessage::new().content(format!("Connected users: {}", 0))
}

pub fn register() -> CreateCommand {
    CreateCommand::new("users").description("Get the number of connected users")
}
