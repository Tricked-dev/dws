use governor::Quota;
use once_cell::sync::Lazy;
use serenity::model::prelude::{ApplicationId, RoleId};

const HOST_CONST: &str = "HOST";
const PORT_CONST: &str = "PORT";
const BROADCAST_SECRET_CONST: &str = "BROADCAST_SECRET";
const COSMETICS_FILE_CONST: &str = "COSMETICS_FILE";
const RATELIMIT_PER_MINUTE_CONST: &str = "RATELIMIT_PER_MINUTE";
const DISCORD_TOKEN_CONST: &str = "DISCORD_TOKEN";
const DISCORD_CLIENT_ID_CONST: &str = "DISCORD_CLIENT_ID";
const DISCORD_PUBLIC_KEY_CONST: &str = "DISCORD_PUBLIC_KEY";
const DISCORD_ADMIN_ROLE_CONST: &str = "DISCORD_ADMIN_ROLE";

pub static HOST: Lazy<String> = Lazy::new(|| std::env::var(HOST_CONST).unwrap_or_else(|_| "127.0.0.1".into()));
pub static PORT: Lazy<u16> = Lazy::new(|| {
    std::env::var(PORT_CONST)
        .unwrap_or_else(|_| "3000".into())
        .parse()
        .unwrap()
});

pub static BROADCAST_SECRET: Lazy<String> =
    Lazy::new(|| std::env::var(BROADCAST_SECRET_CONST).unwrap_or_else(|_| "secret".into()));

pub static COSMETICS_FILE: Lazy<String> =
    Lazy::new(|| std::env::var(COSMETICS_FILE_CONST).unwrap_or_else(|_| "cosmetics.json".into()));

pub static DISCORD_TOKEN: Lazy<String> =
    Lazy::new(|| std::env::var(DISCORD_TOKEN_CONST).expect("DISCORD_TOKEN not set"));

pub static DISCORD_CLIENT_ID: Lazy<ApplicationId> = Lazy::new(|| {
    ApplicationId::new(
        std::env::var(DISCORD_CLIENT_ID_CONST)
            .expect("DISCORD_CLIENT_ID not set")
            .parse::<u64>()
            .expect("Failed to parse DISCORD_CLIENT_ID"),
    )
});

pub static DISCORD_PUBLIC_KEY: Lazy<String> =
    Lazy::new(|| std::env::var(DISCORD_PUBLIC_KEY_CONST).expect("DISCORD_PUBLIC_KEY not set"));

pub static DISCORD_ADMIN_ROLE: Lazy<RoleId> = Lazy::new(|| {
    RoleId::from(
        std::env::var(DISCORD_ADMIN_ROLE_CONST)
            .unwrap_or("1".into())
            .parse::<u64>()
            .expect("Failed to parse DISCORD_ADMIN_ROLE"),
    )
});

pub static RATELIMIT_PER_MINUTE: Lazy<Quota> = Lazy::new(|| {
    Quota::per_minute(
        std::env::var(RATELIMIT_PER_MINUTE_CONST)
            .unwrap_or_else(|_| "20".into())
            .parse()
            .expect("Failed to parse RATELIMIT_PER_MINUTE"),
    )
});
