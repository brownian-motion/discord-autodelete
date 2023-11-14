
use serenity::prelude::*;
use serenity::framework::standard::{StandardFramework};
use serenity::model::Timestamp;
use chrono::{Utc, Duration};

use serenity::model::id::*;

mod login;

mod error;
pub use error::*;

mod client;
use client::*;

mod config;
use config::*;

mod messages;
use messages::*;

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = login::load_bot_token().await.expect("could not load login token");
    let intents = GatewayIntents::non_privileged() | 
                        GatewayIntents::GUILD_MESSAGES;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    let TEST_GUILD_ID = GuildId(1091225753284268092);
    let TEST_CHANNEL_ID = ChannelId(1170843856145756210);
    let AN_HOUR_AGO: Timestamp = (Utc::now() - Duration::hours(1)).into();
    let controller = OldMessageController::new(client.cache_and_http.clone());
    if let Ok(messages) = dbg!(controller.get_old_messages(&TEST_GUILD_ID, &TEST_CHANNEL_ID, &AN_HOUR_AGO).await) {
        dbg!(controller.delete_old_messages(&TEST_GUILD_ID, &TEST_CHANNEL_ID, &messages).await);
    }

    // start listening for events by starting a single shard
    // if let Err(why) = client.start().await {
    //     println!("An error occurred while running the client: {:?}", why);
    // }
}