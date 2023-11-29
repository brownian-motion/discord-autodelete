use serenity::model::prelude::*;

use crate::Result;
use async_trait::async_trait;
use super::GetOldMessageRequest;

#[async_trait]
pub trait OldMessageGetter {
	async fn get_old_messages(&self, request: GetOldMessageRequest) -> Result<Vec<MessageId>>;
}

#[async_trait]
pub trait OldMessageDeleter {
	async fn delete_old_messages(&mut self, server_id: &GuildId, channel_id: &ChannelId, messages: &[MessageId]) -> Result<()>;
}

// see https://doc.rust-lang.org/1.38.0/src/std/io/impls.rs.html#122-143 for example of using Box<dyn Trait>
#[async_trait]
impl<D: OldMessageDeleter + ?Sized + Send + Sync> OldMessageDeleter for Box<D> {
	#[inline]
	async fn delete_old_messages(&mut self, server_id: &GuildId, channel_id: &ChannelId, messages: &[MessageId]) -> Result<()>{
		(**self).delete_old_messages(server_id, channel_id, messages).await
	}
}