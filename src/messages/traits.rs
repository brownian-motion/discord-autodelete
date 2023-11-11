use serenity::model::prelude::*;
use serenity::model::timestamp::Timestamp;
use crate::Result;
use async_trait::async_trait;

#[async_trait]
pub trait OldMessageGetter {
	async fn get_old_messages(&self, server_id: &GuildId, channel_id: &ChannelId, sent_before: &Timestamp) -> Result<Vec<MessageId>>;
}

#[async_trait]
pub trait OldMessageDeleter {
	async fn delete_old_messages(&self, server_id: &GuildId, channel_id: &ChannelId, messages: &[MessageId]) -> Result<Vec<MessageId>>;
}