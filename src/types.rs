pub use serenity::model::id::{GuildId, ChannelId, MessageId};
pub use serenity::model::timestamp::Timestamp;
use std::fmt::{Display, Debug, Formatter, Result};
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct NamedGuild {
	pub id: GuildId,
	pub name: String,
}

impl Display for NamedGuild {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "\"{}\"", self.name)
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct NamedChannel {
	pub id: ChannelId,
	pub name: String,
}

impl Display for NamedChannel {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "#{}", self.name)
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct GetOldMessageRequest {
	pub guild: NamedGuild,
	pub channel: NamedChannel,
	pub sent_before: Timestamp,
	pub just_images: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct DeleteMessagesRequest {
	pub guild: NamedGuild,
	pub channel: NamedChannel,
	pub ids: Vec<MessageId>,
}
