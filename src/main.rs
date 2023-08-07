mod commands;
mod events;

use serenity::prelude::*;

pub struct Bot {
    reqwest_client: reqwest::Client,
    persist: shuttle_persist::PersistInstance
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: shuttle_secrets::SecretStore,
    #[shuttle_persist::Persist] persist: shuttle_persist::PersistInstance
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = secret_store.get("DISCORD_TOKEN")
        .expect("'DISCORD_TOKEN' was not found");

    // Create a reqwest client for http requests
    let reqwest_client = reqwest::Client::builder()
        .user_agent("")
        .build()
        .expect("Err creating reqwest client");

    let intents = 
        GatewayIntents::GUILD_MESSAGES |
        GatewayIntents::MESSAGE_CONTENT | 
        GatewayIntents::DIRECT_MESSAGES;

    let client = Client::builder(&token, intents)
        .event_handler(Bot { reqwest_client, persist })
        .await
        .expect("Err creating client");
    
    Ok(client.into())
}