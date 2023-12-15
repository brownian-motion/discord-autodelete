use serenity::model::id::{ChannelId, GuildId};
use serenity::http::Http;
use futures::prelude::*;
use crate::controller::{traits::*};
use async_trait::async_trait;

pub struct HttpNamer<H> {
	http: H,
}

impl<H> HttpNamer<H> {
	pub fn new(http: H) -> Self 
	where H: AsRef<Http> {
	    HttpNamer{ http }
	}
}

#[async_trait]
impl<H> Namer for HttpNamer<H> where H: AsRef<Http> + Send  + Sync {
	async fn name_guild(&self, guild_id: GuildId) -> String {
		self.http.as_ref().get_guild(guild_id).await
			.map(|g| g.name)
			.unwrap_or(format!("{:?}", guild_id))
	}

	async fn name_channel(&self, channel_id: ChannelId) -> String {
		self.http.as_ref().get_channel(channel_id).await
			.ok().and_then(|g| g.guild())
			.map(|c| c.name)
			.unwrap_or(format!("{:?}", channel_id))
	}
}