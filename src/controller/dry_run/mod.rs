use super::traits::*;
use super::error::DeleteError;
use async_trait::async_trait;
use crate::types::*;
use log::{debug, as_serde};

#[derive(Default)]
pub struct Deleter {}

impl Deleter{
	pub fn new() -> Self {
		Deleter{}
	}
}

#[async_trait]
impl OldMessageDeleter for Deleter {
	async fn delete_old_messages(&mut self, request: DeleteMessagesRequest) -> Result<(), DeleteError>{
		debug!(request = as_serde!(request); "Deleting messages");
		Ok(())
	}
}

