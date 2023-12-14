use serenity::prelude::*;
use serenity::framework::standard::{StandardFramework};
use clap::Parser;
use std::path::PathBuf;
use tokio::time::{sleep, Duration};

mod login;

pub mod error;
pub use error::*;

mod client;
use client::*;

mod config;
use config::{Config, Error as ConfigError};

mod messages;
use messages::*;

mod deleter;
use deleter::*; 

pub mod types;


// const DEFAULT_DISCORD_TOKEN_PATH: &'static str = "/app/discord-token";

#[derive(Parser,Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, env = "DISCORD_BOT_TOKEN_PATH", default_value = "/app/config/discord-bot-token.txt")]
    discord_bot_token_path: PathBuf,

    #[arg(short, long, env = "CONFIG_PATH", default_value = "/app/config/config.yml")]
    config_path: PathBuf,

    #[arg(long, action)]
    dry_run: bool,

    #[arg(long, env = "POLL_INTERVAL_MINUTES", default_value_t = 2)]
    poll_interval_minutes: u64,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let framework = StandardFramework::new()
        // .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = login::load_bot_token(&args.discord_bot_token_path).await.expect("could not load login token");
    let intents = GatewayIntents::non_privileged() | 
                        GatewayIntents::GUILD_MESSAGES;
    let client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    loop {
        println!("reloading config...");
        let config = load_config(&args).expect("could not load config file");

        println!("deleting...");
        delete_old_messages(&client, &config, &args).await;

        println!("sleeping...");
        sleep(Duration::from_secs(args.poll_interval_minutes * 60)).await;
    }

    // start listening for events by starting a single shard
    // if let Err(why) = client.start().await {
    //     println!("An error occurred while running the client: {:?}", why);
    // }
}

fn get_deleter(client: &Client, args: &Args) -> Box<dyn OldMessageDeleter + Send + Sync> {
    if args.dry_run {
        Box::new(DryRunDeleter::new(std::io::stdout()))
    } else {
        Box::new(OldMessageController::new(client.http.clone()))
    }
}

async fn delete_old_messages(client: &Client, config: &Config, args: &Args) {
    let deleter = get_deleter(client, args);

    let mut delete_routine = DeleteRoutine {
        getter: OldMessageController::new(client.http.clone()),
        deleter: deleter,
        namer: HttpNamer::new(client.http.clone()),
    };
    delete_routine.delete_old_messages(config).await;
}

fn load_config(args: &Args) -> Result<Config> {
    match Config::load_from_file(&args.config_path) {
        // bootstrap a new config file if none exists at the target address
        Err(ConfigError::FileNotFound(_)) => {
            println!("Config file does not exist, creating an empty one...");
            let c = Config::empty();
            c.save_to_file(&args.config_path)?;
            Ok(c)
        },
        res => Ok(res?),
    }
}
