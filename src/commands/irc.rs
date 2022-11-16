use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use serenity::{
    builder::{CreateCommand, CreateCommandOption, CreateInteractionResponseMessage},
    model::prelude::{command::CommandOptionType, CommandInteraction, ResolvedValue},
};
use uuid::Uuid;

use crate::{
    app_state::{AppState, User},
    messages::InternalMessages,
    utils::sanitize::sanitize_message,
};

pub async fn run(cmd: CommandInteraction, state: Arc<AppState>, admin: bool) -> CreateInteractionResponseMessage {
    let options = cmd.data.options();
    let sub = options.get(0).unwrap();
    let options = match sub.value.clone() {
        ResolvedValue::SubCommand(v) => v,
        _ => return CreateInteractionResponseMessage::new().content("Expected subcommand"),
    };
    match (sub.name, admin) {
        ("list", true) => {
            let mut res = "Blacklisted uuids\n----------------\n".to_owned();
            for uuid in state.users.lock().iter().filter(|x| x.1.irc_blacklisted) {
                res.push_str(&format!("{}\n", uuid.0));
            }
            CreateInteractionResponseMessage::new().content(res)
        }
        ("blacklist", true) => {
            let add = options.get(0).unwrap().value.string() == "add";
            let uuid = options.get(1).unwrap().value.string();
            let uuid = match Uuid::parse_str(&uuid) {
                Ok(v) => v,
                Err(_) => return CreateInteractionResponseMessage::new().content("Invalid UUID".to_string()),
            };
            let mut users = state.users.lock();

            match users.get(&uuid) {
                Some(v) => {
                    let mut user = v.clone();
                    user.irc_blacklisted = add;
                    users.insert(uuid, user);
                }
                None => {
                    let user = User {
                        irc_blacklisted: add,
                        ..Default::default()
                    };
                    users.insert(uuid, user);
                }
            };

            CreateInteractionResponseMessage::new().content(format!(
                "{} {} the blacklist",
                if add { "Added to" } else { "Removed from" },
                uuid
            ))
        }
        ("send", _) => {
            let users = state
                .users
                .lock()
                .iter()
                .find(|x| x.1.linked_discord == Some(cmd.user.id))
                .map(|x| x.0.clone());

            let uuid = match users {
                Some(v) => v,
                None => {
                    return CreateInteractionResponseMessage::new().content("You are not linked to a minecraft account")
                }
            };

            let msg = sanitize_message(&options.get(0).unwrap().value.string());
            let _ = state.tx.send(InternalMessages::IrcCreate {
                sender: uuid,
                message: msg.clone(),
                date: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis(),
            });
            CreateInteractionResponseMessage::new().ephemeral(true).content("Send!")
        }

        _ => CreateInteractionResponseMessage::new().content("Invalid subcommand".to_string()),
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
                "send",
                "Send a message to the irc channel",
            )
            .add_sub_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "message",
                    "Message to send to the irc channel",
                )
                .required(true),
            ),
        )
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
