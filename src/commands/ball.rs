use crate::Bot;
use crate::commands::*;

use serenity::prelude::*;
use serenity::model::channel::Message;
use tracing::error;

pub const NAME: &str = "ball";
pub const DESCRIPTION: &str = "Send Ballinngs";

pub async fn execute(_self: &Bot, ctx: Context, msg: Message) -> CommandResult {
    let reqwest_client = &_self.reqwest_client;
    let text = reqwest_client.get("https://www.nationstates.net/cgi-bin/api.cgi?nation=Ballinngs")
        .header("User-Agent", "Email: bo31102007@gmail.com")
        .send().await.unwrap()
        .text().await.unwrap();
    if let Err(e) = msg.channel_id.say(&ctx.http, text).await {
        error!("Error sending message: {:?}", e);
    }
    Ok(())
}