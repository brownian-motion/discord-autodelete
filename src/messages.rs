
use serenity::model::prelude::*;
use serenity::model::timestamp::Timestamp;
use crate::Result;

pub async fn get_old_messages(server_id: &GuildId, channel_id: &ChannelId, sent_before: &Timestamp) -> Result<Vec<MessageId>> {
	unimplemented!()
	// use GuildChannel::messages()
}

pub async fn delete_old_messages(server_id: &GuildId, channel_id: &ChannelId, messages: &[MessageId]) -> Result<()> {
	unimplemented()
	// use GuildChannel::delete_messages()
}