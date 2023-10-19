
use tokio::io::AsyncReadExt;
use std::env;
use tokio::fs::File;

const DEFAULT_DISCORD_TOKEN_PATH: &'static str = "/app/discord-token";

pub(crate) async fn load_bot_token() -> std::io::Result<String> {
    let token_path = env::var("DISCORD_TOKEN_PATH").unwrap_or_else(|_| DEFAULT_DISCORD_TOKEN_PATH.to_string());
    let mut f = File::open(token_path).await?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).await?;

    Ok(buffer.trim().to_string())
}