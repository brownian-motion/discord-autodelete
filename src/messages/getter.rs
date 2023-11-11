
use serenity::model::prelude::*;
use serenity::model::timestamp::Timestamp;
use serenity::model::id::*;
use serenity::CacheAndHttp;
use crate::Result;
use futures::prelude::*;
use super::*;
use std::sync::Arc;
use async_trait::async_trait;

/*
#[derive(Debug)]
enum PageResult {
	NothingYet, // this is the initial state
	FetchedMessages(Vec<MessageId>), // holds the results we've already fetched but haven't yielded yet
	PageFinished(MessageId), // stores the last ID we saw
	Done, // the final state
}

impl Default for PageResult {
	fn default() -> Self {
		PageResult::NothingYet
	}
}

struct MessageIterator {
	http:  Arc<CacheAndHttp>,
	channel: ChannelId,
	buffer: PageResult,
}

impl MessageIterator {
	pub fn new(http: Arc<CacheAndHttp>, channel: ChannelId) -> Self {
		OldMessageController{ http: http , channel: channel , buffer: Default::default() }
	}

	// Tries to step forward return the next Message ID, by fetching more if necessary.
	// Mutates this iterator's buffer to get there.
	fn try_next(&mut self) -> Result<Option<MessageId>> {
		let next_buffer = self.fetch_new_buffer_if_needed()?;
	}

	fn fetch_new_buffer_if_needed(&self) -> Result<PageResult> {
		use PageResult::*;
		match self.buffer {
			NothingYet => self.fetch_first_page(),
			FetchedMessages(_) | Done => Ok(self.buffer),
			PageFinished(messageId) => self.fetch_next_page(messageId),
		}
	}

	fn fetch_first_page(&self) -> Result<PageResult> {
		self.channel.
	}
}
*/

pub use traits::*;

pub struct OldMessageController {
	http:  Arc<CacheAndHttp>,
}

impl OldMessageController {
	pub fn new(http: Arc<CacheAndHttp>) -> Self {
		OldMessageController{ http }
	}
}

#[async_trait]
impl OldMessageGetter for OldMessageController {
	async fn get_old_messages(&self, server_id: &GuildId, channel_id: &ChannelId, sent_before: &Timestamp) -> Result<Vec<MessageId>> {
		// for now , assume the IDs can all fit in memory
		let mut stream = channel_id.messages_iter(&*self.http).boxed();
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
