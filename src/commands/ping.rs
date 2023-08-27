use crate::Bot;
use crate::commands::*;

use serenity::prelude::*;
use serenity::model::channel::Message;

pub const NAME: &str = "ping";
pub const DESCRIPTION: &str = "Ping nation (login) to keep it from being deleted.";

pub async fn execute(bot: &Bot, ctx: Context, msg: Message, args: Vec<String>) -> CommandResult {
    if msg.guild_id.is_some() { return Ok(()); }

    let reqwest_client = &bot.reqwest_client;
    let query = [("nation", "Ballinngs"), ("q", "ping")];

    let text = reqwest_client.get("https://www.nationstates.net/cgi-bin/api.cgi")
        .header("X-Password", &args[1])
        .query(&query)
        .send().await?
        .text().await?;
    
    msg.channel_id.say(&ctx.http, text).await?;
    Ok(())
}

