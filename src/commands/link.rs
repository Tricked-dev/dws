use std::sync::Arc;

use serenity::{
    builder::{CreateCommand, CreateCommandOption, CreateInteractionResponseMessage},
    model::prelude::{command::CommandOptionType, CommandInteraction, ResolvedValue},
};

use crate::{
    app_state::{AppState, User},
    utils::username_to_uuid_and_discord,
};

pub async fn run(cmd: CommandInteraction, state: Arc<AppState>) -> CreateInteractionResponseMessage {
    let mcusername = cmd.data.options().get(0).unwrap().value.string();
    let data = match username_to_uuid_and_discord(&mcusername).await {
        Ok(v) => v,
        Err(e) => {
            return {
                println!("{e:?}");
                CreateInteractionResponseMessage::new().content("Discord not linked!")
            }
        }
    };
    let user = cmd.user;
    let username = format!("{}#{:04}", user.name, user.discriminator);
    if username != data.links.discord {
        return CreateInteractionResponseMessage::new().content("Discord link does not match!");
    }
    let mut f = state.users.lock();
    f.insert(
        data.uuid,
        User {
            linked_discord: Some(user.id),
            ..Default::default()
        },
    );

    CreateInteractionResponseMessage::new().content(format!("Linked {} to {} ({})", username, mcusername, data.uuid))
}

pub fn register() -> CreateCommand {
    CreateCommand::new("link")
        .description("Link your discord account to your minecraft account")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "username", "Your minecraft username").required(true),
        )
}
trait PanicOrFuckingWork {
    fn string(&self) -> String;
}

impl<'a> PanicOrFuckingWork for ResolvedValue<'a> {
    fn string(&self) -> String {
        match self {
            ResolvedValue::String(v) => v.to_owned().to_string(),
            _ => panic!("Expected string"),
        }
    }
}
