use async_trait::async_trait;
use super::{GetError, DeleteError};
use crate::types::*;
use serenity::model::id::{GuildId, ChannelId, MessageId};

#[async_trait]
pub trait OldMessageGetter {
	async fn get_old_messages(&self, request: GetOldMessageRequest) -> Result<Vec<MessageId>, GetError>;
}

#[async_trait]
pub trait OldMessageDeleter {
	async fn delete_old_messages(&mut self, request: DeleteMessagesRequest) -> Result<(), DeleteError>;
}

// see https://doc.rust-lang.org/1.38.0/src/std/io/impls.rs.html#122-143 for example of using Box<dyn Trait>
#[async_trait]
impl<D: OldMessageDeleter + ?Sized + Send + Sync> OldMessageDeleter for Box<D> {
	#[inline]
	async fn delete_old_messages(&mut self, request: DeleteMessagesRequest) -> Result<(), DeleteError>{
		(**self).delete_old_messages(request).await
	}
}

#[async_trait]
pub trait Namer {
	// async fn name_message(&self, MessageId) -> String;
	async fn name_channel(&self, channel_id: ChannelId) -> String;
	async fn name_guild(&self, guild_id: GuildId) -> String;
}