use clap::Parser;
use log::*;
use serde::Serialize;
use serenity::framework::standard::StandardFramework;
use serenity::prelude::*;
use std::path::PathBuf;
use structured_logger::{async_json::new_writer, Builder as LogBuilder};
use tokio::time::{sleep, Duration};

mod login;

pub mod error;
pub use error::*;

mod client;
use client::*;

mod config;
use config::{Config, Error as ConfigError};

mod controller;
use controller::{dry_run::Deleter as DryRunDeleter, http::*, *};

mod deleter;
use deleter::*;

pub mod types;

#[derive(Parser, Debug, Serialize)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(
        short,
        long,
        env = "DISCORD_BOT_TOKEN_PATH",
        default_value = "/app/config/discord-bot-token.txt"
    )]
    discord_bot_token_path: PathBuf,

    #[arg(
        short,
        long,
        env = "CONFIG_PATH",
        default_value = "/app/config/config.yml"
    )]
    config_path: PathBuf,

    #[arg(long, action)]
    dry_run: bool,

    #[arg(long, env = "POLL_INTERVAL_MINUTES", default_value_t = 2)]
    poll_interval_minutes: u64,

    #[arg(long, env = "RUST_LOG", default_value = "info")]
    log_level: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    LogBuilder::with_level(&args.log_level)
        .with_default_writer(new_writer(tokio::io::sink())) // I don't want to see printouts from serenity
        .with_target_writer("discord_autodelete*", new_writer(tokio::io::stdout()))
        .init();

    let framework = StandardFramework::new()
        // .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = login::load_bot_token(&args.discord_bot_token_path)
        .await
        .expect("could not load login token");
    let intents = GatewayIntents::empty()
                        | GatewayIntents::GUILD_MESSAGES
                        | GatewayIntents::MESSAGE_CONTENT /* to know if it has an attachment */;
    let client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    info!(args = as_serde!(&args); "starting");

    loop {
        info!("reloading config");
        let config = load_config(&args).expect("could not load config file");

        info!("deleting");
        delete_old_messages(&client, &config, &args).await;

        info!(num_minutes = args.poll_interval_minutes; "sleeping");
        sleep(Duration::from_secs(args.poll_interval_minutes * 60)).await;
    }

    // start listening for events by starting a single shard
    // if let Err(why) = client.start().await {
    //     println!("An error occurred while running the client: {:?}", why);
    // }
}

fn get_deleter(client: &Client, args: &Args) -> Box<dyn OldMessageDeleter + Send + Sync> {
    if args.dry_run {
        Box::new(DryRunDeleter::new())
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
            warn!("Config file does not exist, creating an empty one...");
            let c = Config::empty();
            c.save_to_file(&args.config_path)?;
            Ok(c)
        }
        res => Ok(res?),
    }
}
