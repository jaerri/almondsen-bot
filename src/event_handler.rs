use crate::Bot;
use crate::commands::*;

use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use tracing::{error, info};

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {     
        if !msg.content.starts_with(PREFIX) { return; }
        const PREFIX: &str = "!";
        
        let args: Vec<String> = msg.content[1..]
            .split(" ")
            .map(|s| s.to_string())
            .collect();
        
        // mon boy (it robbed me of my sanity)
        macro_rules! exec {
            ($content: expr, [$($cmd: ident),*]) => {{
                match $content {
                    $(
                        $cmd::NAME => { $cmd::execute(self, ctx, msg, args).await } 
                    )*
                    _ => Ok(())
                }
            }};
        }
        // Add command name here
        let result: CommandResult = exec!(args[0].as_str(), 
            [ hello, ball, ping, addlogin ]
        );

        /*match &msg.content[1..] {
            commands::hello::NAME => { commands::hello::execute(self, ctx, msg).await } 
            _ => Ok(())
        };*/
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} reporting, your mom gay.", ready.user.name);
    }
}