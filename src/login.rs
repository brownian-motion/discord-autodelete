use tokio::io::AsyncReadExt;

use tokio::fs::File;
use std::path::Path;

pub(crate) async fn load_bot_token(token_path: &Path) -> std::io::Result<String> {
    let mut f = File::open(token_path).await?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).await?;

    Ok(buffer.trim().to_string())
}