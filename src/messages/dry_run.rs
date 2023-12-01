use super::traits::*;
use super::error::DeleteError;
use async_trait::async_trait;
use crate::types::*;

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
	async fn delete_old_messages(&mut self, request: DeleteMessagesRequest) -> Result<(), DeleteError>{
		writeln!(self.printer, "Deleting {} messages for {} in {}: {:?}", request.ids.len(), request.channel, request.guild, request.ids).expect("could not print messages to stdout");
		Ok(())
	}
}

