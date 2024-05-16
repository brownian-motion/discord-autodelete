use super::error::DeleteError;
use super::traits::*;
use crate::types::*;
use async_trait::async_trait;
use log::{as_serde, debug};

#[derive(Default)]
pub struct Deleter {}

impl Deleter {
    pub fn new() -> Self {
        Deleter {}
    }
}

#[async_trait]
impl OldMessageDeleter for Deleter {
    async fn delete_old_messages(
        &mut self,
        request: DeleteMessagesRequest,
    ) -> Result<(), DeleteError> {
        debug!(request = as_serde!(request); "Deleting messages");
        Ok(())
    }
}
