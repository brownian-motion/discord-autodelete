
use serenity::model::id::{MessageId};
use serenity::http::Http;
use futures::prelude::*;
use super::*;
use async_trait::async_trait;
use crate::types::*;

pub struct OldMessageController<H> {
	http:  H,
}

impl<H> OldMessageController<H> where H: AsRef<Http> + Sync {
	pub fn new(http: H) -> Self {
		OldMessageController{ http }
	}
}

#[async_trait]
impl<H> OldMessageGetter for OldMessageController<H> where H: AsRef<Http> + Sync {
	async fn get_old_messages(&self, request: GetOldMessageRequest) -> Result<Vec<MessageId>, GetError> {
		//  server_id: &GuildId, channel_id: &ChannelId, sent_before: &Timestamp
		// for now , assume the IDs can all fit in memory
		let mut stream = request.channel.id.messages_iter(self.http.as_ref()).boxed();
		let mut ids = vec![];
		while let Some(res) = stream.next().await {
			match res {
				// Timestamp doesn't implement `<`, so we compare the equivalent Unix timestamp instead
				Ok(m) => if !m.pinned && m.timestamp.timestamp() < request.sent_before.timestamp() { 
					ids.push(m.id);
				},
				Err(e) => return Err(e.into()),
			}
		};
		Ok(ids)
	}
}


#[async_trait]
impl<H> OldMessageDeleter for OldMessageController<H> where H: AsRef<Http> + Sync + Send {
	async fn delete_old_messages(&mut self, request: DeleteMessagesRequest) -> Result<(), DeleteError>{
		// for now , assume the IDs can all fit in memory
		let http = self.http.as_ref();
		let _ = request.channel.id.delete_messages(http, &request.ids).await?;
		drop(http);
		Ok(())
	}
}

