use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use serenity::prelude::*;

#[group]
#[commands(ping)]
pub struct General;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[command]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}
