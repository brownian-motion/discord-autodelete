
use serenity::model::prelude::*;
use serenity::model::timestamp::Timestamp;
use crate::Result;

#![async_trait]
pub trait OldMessageGetter {
	async fn get_old_messages(&self, server_id: &GuildId, channel_id: &ChannelId, sent_before: &Timestamp) -> Result<Vec<MessageId>>;
}

#![async_trait]
pub trait OldMessageDeleter {
	async fn delete_old_messages(&self, server_id: &GuildId, channel_id: &ChannelId, messages: &[MessageId]) -> Result<Vec<MessageId>>;
}

pub struct OldMessageController {
	http:  Arc<CacheAndHttp>;
}

impl OldMessageController {
	pub fn new(http: Arc<CacheAndHttp>) -> Self {
		OldMessageController{ http }
	}
}

impl OldMessageGetter for OldMessageController {
	async fn get_old_messages(&self, server_id: &GuildId, channel_id: &ChannelId, sent_before: &Timestamp) -> Result<Vec<MessageId>> {
		Ok(
			channel_id
				.messages(&self.http, |r| r.before(sent_before).limit(200)).await?
				.map(|m| m.id)
				.collect()
		)
	}
}

pub async fn get_old_messages(server_id: &GuildId, channel_id: &ChannelId, sent_before: &Timestamp) -> Result<Vec<MessageId>> {
	unimplemented!()
	// use GuildChannel::messages()
}

pub async fn delete_old_messages(server_id: &GuildId, channel_id: &ChannelId, messages: &[MessageId]) -> Result<()> {
	unimplemented!()
	// use GuildChannel::delete_messages()
}