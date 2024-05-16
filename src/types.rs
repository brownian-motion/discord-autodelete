use serde::Serialize;
pub use serenity::model::id::{ChannelId, GuildId, MessageId};
pub use serenity::model::timestamp::Timestamp;
use std::fmt::{Debug, Display, Formatter, Result};

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
