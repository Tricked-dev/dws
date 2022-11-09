use std::sync::Arc;

use serenity::{
    builder::{CreateCommand, CreateCommandOption, CreateInteractionResponseMessage},
    model::prelude::{command::CommandOptionType, CommandInteraction, ResolvedValue},
};
use uuid::Uuid;

use crate::{
    app_state::{AppState, User},
    bitflags::CosmeticFlags,
};

pub fn run(cmd: CommandInteraction, state: Arc<AppState>) -> CreateInteractionResponseMessage {
    let uuid = match cmd.data.options().get(0).map(|v| v.value.clone()) {
        Some(ResolvedValue::String(v)) => match Uuid::parse_str(v) {
            Ok(v) => v,
            Err(_) => return CreateInteractionResponseMessage::new().content("Invalid UUID".to_string()),
        },
        _ => return CreateInteractionResponseMessage::new().content("Invalid UUID".to_string()),
    };
    let bits = CosmeticFlags::from_bits(match cmd.data.options().get(1).map(|v| v.value.clone()) {
        Some(ResolvedValue::Integer(v)) => v as u32,
        Some(ResolvedValue::String(v)) => u32::from_str_radix(v, 2).unwrap(),
        _ => 0,
    })
    .unwrap();

    let user = match state.users.lock().get(&uuid) {
        Some(v) => {
            let mut v = v.clone();
            v.flags = bits;
            v
        }
        None => User {
            flags: bits,
            enabled_prefix: None,
        },
    };

    state.users.lock().insert(uuid, user);
    CreateInteractionResponseMessage::new().content(format!("Permission bits changed to {:b} for {}", bits, uuid))
}

pub fn register() -> CreateCommand {
    CreateCommand::new("change_perms")
        .description("Change the permissions of the user")
        .add_option(CreateCommandOption::new(CommandOptionType::String, "uuid", "The UUID of the user").required(true))
        .add_option(CreateCommandOption::new(
            CommandOptionType::Number,
            "int",
            "Int permission bits",
        ))
        .add_option(CreateCommandOption::new(
            CommandOptionType::String,
            "binary",
            "Binary permission bits",
        ))
}
