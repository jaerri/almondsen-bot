use crate::Bot;
use crate::commands::*;

use serenity::prelude::*;
use serenity::model::channel::Message;

pub const NAME: &str = "hello";
pub const DESCRIPTION: &str = "Say \"world!\"";

pub async fn execute(_self: &Bot, ctx: Context, msg: Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "world!").await?;
    msg.author.dm(&ctx, |m| m.content("Hello!")).await?;
    Ok(())
}

