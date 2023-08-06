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
        // mon boy (it robbed me of my sanity)
        macro_rules! exec {
            ($content: expr, [$($cmd: ident),*]) => {{
                match $content {
                    $(
                        $cmd::NAME => { $cmd::execute(self, ctx, msg).await } 
                    )*
                    _ => Ok(())
                }
            }};
        }
        
        const PREFIX: &str = "!";
        let msg_content = &msg.content[1..];
        if !msg.content.starts_with(PREFIX) { return; }

        // Add command module here
        let result: CommandResult = exec!(&msg.content[1..], 
            [ hello, ball ]
        );

        /*match &msg.content[1..] {
            commands::hello::NAME => { commands::hello::execute(self, ctx, msg).await } 
            _ => Ok(())
        };*/

        if let Ok(m) = result {

        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}