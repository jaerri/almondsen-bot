mod commands;
mod event_handler;
mod utils;

use std::env;
use serenity::prelude::*;

pub struct Bot {
    reqwest_client: reqwest::Client,
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: shuttle_secrets::SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = secret_store.get("DISCORD_TOKEN")
        .expect("'DISCORD_TOKEN' was not found");

    // Get USER_AGENT
    static APP_USER_AGENT: &str = concat!(
        env!("CARGO_PKG_NAME"), "/v", env!("CARGO_PKG_VERSION"), " (", env!("CARGO_PKG_REPOSITORY"), ")"
    );
    // Create a reqwest client for http requests
    let reqwest_client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .expect("Err creating reqwest client");
    
    let intents = 
        GatewayIntents::GUILD_MESSAGES |
        GatewayIntents::MESSAGE_CONTENT | 
        GatewayIntents::DIRECT_MESSAGES;

    let client = Client::builder(&token, intents)
        .event_handler(Bot { reqwest_client})
        .await
        .expect("Err creating client");
    
    Ok(client.into())
}
