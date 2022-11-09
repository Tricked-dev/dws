use std::sync::Arc;

use once_cell::sync::Lazy;
use serenity::{builder::*, http::Http, model::application::interaction::application_command::*};

use crate::{
    app_state::AppState,
    config::{DISCORD_CLIENT_ID, DISCORD_TOKEN},
    Result,
};

mod change_perms;
mod users;

static REST: Lazy<Http> = Lazy::new(|| {
    let http = Http::new(&DISCORD_TOKEN);
    http.set_application_id(*DISCORD_CLIENT_ID);
    http
});

pub fn handle_command(interaction: CommandInteraction, state: Arc<AppState>) -> CreateInteractionResponse {
    let res = match interaction.data.name.as_str() {
        "users" => users::run(interaction, state),
        "change_perms" => change_perms::run(interaction, state),
        _ => CreateInteractionResponseMessage::new().content("404 command not found lol".to_string()),
    };
    CreateInteractionResponse::Message(res)
}
pub async fn register() -> Result<()> {
    REST.create_global_application_commands(&vec![users::register(), change_perms::register()])
        .await?;
    Ok(())
}
