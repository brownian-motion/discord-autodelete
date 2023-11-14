
use serenity::model::prelude::*;
use serenity::model::timestamp::Timestamp;
use serenity::model::id::*;
use serenity::http::client::*;
use serenity::CacheAndHttp;
use crate::Result;
use futures::prelude::*;
use super::*;
use async_trait::async_trait;

use traits::*;


pub struct OldMessageController<H> {
	http:  H,
}

impl<H> OldMessageController<H> where H: AsRef<CacheAndHttp> + Sync {
	pub fn new(http: H) -> Self {
		OldMessageController{ http }
	}
}

#[async_trait]
impl<H> OldMessageGetter for OldMessageController<H> where H: AsRef<CacheAndHttp> + Sync {
	async fn get_old_messages(&self, server_id: &GuildId, channel_id: &ChannelId, sent_before: &Timestamp) -> Result<Vec<MessageId>> {
		// for now , assume the IDs can all fit in memory
		let mut stream = channel_id.messages_iter(self.http.as_ref()).boxed();
		let mut ids = vec![];
		while let Some(res) = stream.next().await {
			match res {
				Ok(m) => if m.timestamp.timestamp() < sent_before.timestamp() { 
					ids.push(m.id);
				},
				Err(e) => return Err(e.into()),
			}
		};
		Ok(ids)
	}
}


#[async_trait]
impl<H> OldMessageDeleter for OldMessageController<H> where H: AsRef<CacheAndHttp> + Sync {
	async fn delete_old_messages(&self, server_id: &GuildId, channel_id: &ChannelId, messages: &[MessageId]) -> Result<()>{
		// for now , assume the IDs can all fit in memory
		let http = self.http.as_ref();
		let _ = channel_id.delete_messages(http, messages).await?;
		drop(http);
		Ok(())
	}
}

