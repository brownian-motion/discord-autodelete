use serenity::model::prelude::*;
use serenity::model::timestamp::Timestamp;

mod traits;
pub use traits::*;

mod controller;
pub use controller::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GetOldMessageRequest {
	pub guild_id: GuildId,
	pub channel_id: ChannelId,
	pub sent_before: Timestamp,
}