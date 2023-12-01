use crate::types::*;
use super::traits::*;
use super::error::*;
use async_trait::async_trait;
use serenity::model::id::MessageId;


// An OldMessageGetter that always returns the same response when asked to read or delete
struct SimpleOldMessageGetterStub(Box<dyn Sync + Fn(GetOldMessageRequest) -> Result<Vec<MessageId>, GetError>>);

pub fn getter_stub<F>(f: F) -> impl OldMessageGetter
	where F: Fn(GetOldMessageRequest) -> Result<Vec<MessageId>, GetError> + 'static + Sync
{
	SimpleOldMessageGetterStub(Box::new(f))
}

pub fn getter_noop() -> impl OldMessageGetter
{
	getter_stub(|_| Ok(vec![]))
}

#[async_trait]
impl OldMessageGetter for SimpleOldMessageGetterStub {
	async fn get_old_messages(&self, request: GetOldMessageRequest) -> Result<Vec<MessageId>, GetError> {
		self.0(request)
	}
}

// An OldMessageController that always returns the same response when asked to read or delete
struct SimpleOldMessageDeleterStub(Box<dyn Send + Sync + Fn(DeleteMessagesRequest) -> Result<(), DeleteError>>);


pub fn deleter_stub<F>(f: F) -> impl OldMessageDeleter
	where F: Fn(DeleteMessagesRequest) -> Result<(), DeleteError> + 'static + Sync + Send
{
	SimpleOldMessageDeleterStub(Box::new(f))
}

pub fn deleter_noop() -> impl OldMessageDeleter
{
	deleter_stub(|_| Ok(()))
}

#[async_trait]
impl OldMessageDeleter for SimpleOldMessageDeleterStub{
	async fn delete_old_messages(&mut self, request: DeleteMessagesRequest) -> Result<(), DeleteError>{
		self.0(request)
	}
}

