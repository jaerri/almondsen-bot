use crate::Bot;
use crate::commands::*;

use serenity::prelude::*;
use serenity::model::channel::Message;
use toml::Table;

pub const NAME: &str = "addlogin";
pub const DESCRIPTION: &str = "addlogin";

pub async fn execute(_self: &Bot, ctx: Context, msg: Message, _args: Vec<String>) -> CommandResult {
    msg.channel_id.say(ctx.http, "Enter toml").await?;
    let reply = msg.channel_id.await_reply(ctx.shard)
        .author_id(msg.author.id)
        .await
        .ok_or_else(|| anyhow::anyhow!("Err getting credentials"))?;
    let data: Table = toml::from_str(&reply.content)?;

    println!("{:?}", data);
    Ok(())
}