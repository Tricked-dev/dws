use std::{env, sync::Arc};

use once_cell::sync::Lazy;
use serenity::{
    builder::*,
    http::Http,
    model::{application::interaction::application_command::*, prelude::ApplicationId},
};

use crate::{app_state::AppState, Result};

mod users;

static REST: Lazy<Http> = Lazy::new(|| {
    let http = Http::new(&std::env::var("DISCORD_TOKEN").unwrap());
    http.set_application_id(ApplicationId::new(
        env::var("DISCORD_ID").unwrap().parse::<u64>().unwrap(),
    ));
    http
});

pub fn handle_command(interaction: CommandInteraction, state: Arc<AppState>) -> CreateInteractionResponse {
    let res = match interaction.data.name.as_str() {
        "users" => users::run(interaction, state),
        _ => CreateInteractionResponseMessage::new().content("404 command not found lol".to_string()),
    };
    CreateInteractionResponse::Message(res)
}
pub async fn register() -> Result<()> {
    REST.create_global_application_commands(&vec![users::register()])
        .await?;
    Ok(())
}
