use std::num::{NonZeroU32, ParseIntError};

use clap::Parser;
use governor::Quota;
use serenity::model::prelude::{ApplicationId, ChannelId, RoleId};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Network port to use
    #[arg(env, long, default_value = "3000")]
    pub port: u16,
    /// Host to bind to 0.0.0.0 for public access
    #[arg(env, long, default_value = "127.0.0.1")]
    pub host: String,
    /// API secret
    #[arg(env, long, default_value = "secret")]
    pub api_secret: String,
    /// Admin dash enabled
    #[arg(env, long, default_value = "true")]
    pub admin_dash: bool,
    /// Cosmetics file
    #[arg(env, long, default_value = "cosmetics.json")]
    pub cosmetics_file: String,
    /// Ratelimit per minute
    #[arg(env, long, default_value = "100", value_parser = parse_quota)]
    pub ratelimit_per_minute: Quota,
    /// Discord bot token
    #[arg(env, long)]
    pub discord_token: String,
    /// Discord bot client ID
    #[arg(env, long, value_parser  = parse_app_id)]
    pub discord_client_id: ApplicationId,
    /// Discord bot public key
    #[arg(env, long)]
    pub discord_public_key: String,
    /// Discord admin role
    #[arg(env, long, value_parser  = parse_role_id)]
    pub discord_admin_role: Option<RoleId>,
    /// Discord IRC channel
    #[arg(env, long, value_parser  = parse_channel_id)]
    pub discord_irc_channel: Option<ChannelId>,
    /// Discord linked role
    #[arg(env, long, value_parser  = parse_role_id)]
    pub discord_linked_role: Option<RoleId>,
}

fn parse_app_id(src: &str) -> Result<ApplicationId, ParseIntError> {
    src.parse::<u64>().map(ApplicationId::new)
}

fn parse_role_id(src: &str) -> Result<RoleId, ParseIntError> {
    src.parse::<u64>().map(RoleId::new)
}
fn parse_channel_id(src: &str) -> Result<ChannelId, ParseIntError> {
    src.parse::<u64>().map(ChannelId::new)
}
fn parse_quota(src: &str) -> Result<Quota, ParseIntError> {
    src.parse::<NonZeroU32>().map(Quota::per_minute)
}
