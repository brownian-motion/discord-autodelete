use crate::Result;

use super::*;
use async_trait::async_trait;


// An OldMessageGetter that always returns the same response when asked to read or delete
struct SimpleOldMessageGetterStub(Box<dyn Sync + Fn(GetOldMessageRequest) -> Result<Vec<MessageId>>>);

pub fn getter_stub<F>(f: F) -> impl OldMessageGetter
	where F: Fn(GetOldMessageRequest) -> Result<Vec<MessageId>> + 'static + Sync
{
	SimpleOldMessageGetterStub(Box::new(f))
}

pub fn getter_noop() -> impl OldMessageGetter
{
	getter_stub(|_| Ok(vec![]))
}

#[async_trait]
impl OldMessageGetter for SimpleOldMessageGetterStub {
	async fn get_old_messages(&self, request: GetOldMessageRequest) -> Result<Vec<MessageId>> {
		self.0(request)
	}
}

// An OldMessageController that always returns the same response when asked to read or delete
struct SimpleOldMessageDeleterStub(Box<dyn Sync + Fn(&GuildId, &ChannelId, &[MessageId]) -> Result<()>>);


pub fn deleter_stub<F>(f: F) -> impl OldMessageDeleter
	where F: Fn(&GuildId, &ChannelId, &[MessageId]) -> Result<()> + 'static + Sync
{
	SimpleOldMessageDeleterStub(Box::new(f))
}

pub fn deleter_noop() -> impl OldMessageDeleter
{
	deleter_stub(|_,_,_| Ok(()))
}

#[async_trait]
impl OldMessageDeleter for SimpleOldMessageDeleterStub{
	async fn delete_old_messages(&self, server_id: &GuildId, channel_id: &ChannelId, messages: &[MessageId]) -> Result<()>{
		self.0(server_id, channel_id, messages)
	}
}

