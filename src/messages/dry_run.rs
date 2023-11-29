
use serenity::model::prelude::*;



use serenity::CacheAndHttp;
use crate::Result;
use futures::prelude::*;
use super::*;
use async_trait::async_trait;



pub struct DryRunDeleter<W> {
	printer:  W,
}

impl<W> DryRunDeleter<W> where W: std::io::Write {
	pub fn new(printer: W) -> Self {
		DryRunDeleter{ printer }
	}
}


#[async_trait]
impl<W> OldMessageDeleter for DryRunDeleter<W> where W: std::io::Write + Sync + Send {
	async fn delete_old_messages(&mut self, server_id: &GuildId, channel_id: &ChannelId, messages: &[MessageId]) -> Result<()>{
		writeln!(self.printer, "Deleting {} messages for channel {:?} in server {:?}: {:?}", messages.len(), channel_id, server_id, messages).expect("could not print messages to stdout");
		Ok(())
	}
}

