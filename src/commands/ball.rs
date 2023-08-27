use crate::Bot;
use crate::commands::*;

use serenity::prelude::*;
use serenity::model::channel::Message;
use tracing::error;

pub const NAME: &str = "ball";
pub const DESCRIPTION: &str = "Send Ballinngs";

pub async fn execute(bot: &Bot, ctx: Context, msg: Message, args: Vec<String>) -> CommandResult {
    let reqwest_client = &bot.reqwest_client;
    let text = reqwest_client.get("https://www.nationstates.net/cgi-bin/api.cgi?nation=Ballinngs")
        .send().await?
        .text().await?;
    msg.channel_id.say(&ctx.http, text).await?;
    Ok(())
}