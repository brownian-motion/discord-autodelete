use serenity::prelude::*;
use serenity::framework::standard::{StandardFramework};
use serenity::model::Timestamp;
use chrono::{Utc, Duration};
use serenity::model::id::*;
use clap::Parser;
use std::path::PathBuf;

mod login;

mod error;
pub use error::*;

mod client;
use client::*;

mod config;
use config::*;

mod messages;
use messages::*;

// const DEFAULT_DISCORD_TOKEN_PATH: &'static str = "/app/discord-token";

#[derive(Parser,Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, env = "DISCORD_BOT_TOKEN_PATH", default_value = "/app/discord-bot-token")]
    discord_bot_token_path: PathBuf,

    #[arg(short, long, env = "CONFIG_PATH", default_value = "/app/config.yml")]
    config_path: PathBuf,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = login::load_bot_token(&args.discord_bot_token_path).await.expect("could not load login token");
    let intents = GatewayIntents::non_privileged() | 
                        GatewayIntents::GUILD_MESSAGES;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    let config = test_config();
    delete_old_messages(&client, &config);

    // start listening for events by starting a single shard
    // if let Err(why) = client.start().await {
    //     println!("An error occurred while running the client: {:?}", why);
    // }
}

async fn delete_old_messages(client: &Client, config: &Config) {
    println!("Deleting messages for {} schedules", config.schedules.len());
    let controller = OldMessageController::new(client.cache_and_http.clone());
    for schedule in &config.schedules {
        // TODO: just pass the schedule instead
        let cutoff_time = schedule.oldest_permitted_message_time();
        let request = GetOldMessageRequest {
            guild_id: schedule.guild_id,
            channel_id: schedule.channel_id,
            sent_before: cutoff_time,
        };
        println!("Fetching messages in channel {:?} older than {} hours", schedule.channel_id, schedule.delete_older_than.num_hours());
        let messages = match controller.get_old_messages(request).await {
            Ok(messages) => messages,
            Err(e) => {
                eprintln!("Error loading messages from channel {:?}: {:?}", schedule.channel_id, e);
                continue;
            },
        };
        if messages.is_empty() {
            println!("Nothing to delete for channel {:?}", schedule.channel_id);
            continue;
        }
        match controller.delete_old_messages(&schedule.guild_id, &schedule.channel_id, &messages).await {
            Ok(_) => println!("Deleted {} old messages from channel {:?}", messages.len(), schedule.channel_id),
            Err(e) => eprintln!("Error deleting {} messages from channel {:?}: {:?}", messages.len(), schedule.channel_id, e),
        };
    }

    println!("Finished deleting from {} channels", config.schedules.len());
}

fn test_config() -> Config {
    Config {
        schedules: vec![
            DeleteSchedule {
                guild_id: GuildId(1091225753284268092),
                channel_id: ChannelId(1170843856145756210),
                delete_older_than: Duration::hours(1),
                last_run: None,
            },
        ]
    }
}