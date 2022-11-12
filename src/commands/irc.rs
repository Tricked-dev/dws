use std::sync::Arc;

use serenity::{
    builder::{CreateCommand, CreateCommandOption, CreateInteractionResponseMessage},
    model::prelude::{command::CommandOptionType, CommandInteraction, ResolvedValue},
};
use uuid::Uuid;

use crate::app_state::AppState;

pub async fn run(cmd: CommandInteraction, state: Arc<AppState>) -> CreateInteractionResponseMessage {
    let options = cmd.data.options();
    let sub = options.get(0).unwrap();
    match sub.name {
        "list" => {
            let mut res = "Blacklisted uuids\n----------------\n".to_owned();
            for uuid in state.irc_blacklist.lock().iter() {
                res.push_str(&format!("{}\n", uuid));
            }
            CreateInteractionResponseMessage::new().content(res)
        }
        "blacklist" => {
            let options = sub.value.clone();
            if let ResolvedValue::SubCommand(v) = options {
                let add = v.get(0).unwrap().value.string() == "add";
                let uuid = v.get(1).unwrap().value.string();
                let uuid = match Uuid::parse_str(&uuid) {
                    Ok(v) => v,
                    Err(_) => return CreateInteractionResponseMessage::new().content("Invalid UUID".to_string()),
                };
                if add {
                    state.irc_blacklist.lock().insert(uuid);
                } else {
                    state.irc_blacklist.lock().remove(&uuid);
                }

                CreateInteractionResponseMessage::new().content(format!(
                    "{} {} {} the blacklist",
                    if add { "Added" } else { "Removed" },
                    if add { "to" } else { "from" },
                    uuid
                ))
            } else {
                unreachable!()
            }
        }
        _ => unreachable!(),
    }
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

pub fn register() -> CreateCommand {
    CreateCommand::new("irc")
        .description("Change the permissions of the user")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "blacklist",
                "Add or remove people from the blacklist",
            )
            .add_sub_option(
                CreateCommandOption::new(CommandOptionType::String, "act", "act")
                    .add_string_choice("Remove", "remove")
                    .add_string_choice("Add", "add")
                    .required(true),
            )
            .add_sub_option(CreateCommandOption::new(CommandOptionType::String, "uuid", "uuid").required(true)),
        )
        .add_option(CreateCommandOption::new(
            CommandOptionType::SubCommand,
            "list",
            "List the blacklist",
        ))
}