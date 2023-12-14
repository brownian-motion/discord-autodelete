use super::traits::*;
use super::error::DeleteError;
use async_trait::async_trait;
use crate::types::*;

pub struct Deleter<W> {
	printer:  W,
}

impl<W> Deleter<W> where W: std::io::Write {
	pub fn new(printer: W) -> Self {
		Deleter{ printer }
	}
}

#[async_trait]
impl<W> OldMessageDeleter for Deleter<W> where W: std::io::Write + Sync + Send {
	async fn delete_old_messages(&mut self, request: DeleteMessagesRequest) -> Result<(), DeleteError>{
		writeln!(self.printer, "Deleting {} messages from {} in {}: {:?}", request.ids.len(), request.channel, request.guild, request.ids).expect("could not print messages to stdout");
		Ok(())
	}
}

