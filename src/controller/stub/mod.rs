use super::error::*;
use super::traits::*;
use crate::types::*;
use async_trait::async_trait;
use serenity::model::id::{ChannelId, GuildId, MessageId};

// An OldMessageGetter that always returns the same response when asked to read or delete
struct SimpleOldMessageGetterStub(
    Box<dyn Sync + Fn(GetOldMessageRequest) -> Result<Vec<MessageId>, GetError>>,
);

pub fn getter_stub<F>(f: F) -> impl OldMessageGetter
where
    F: Fn(GetOldMessageRequest) -> Result<Vec<MessageId>, GetError> + 'static + Sync,
{
    SimpleOldMessageGetterStub(Box::new(f))
}

pub fn getter_noop() -> impl OldMessageGetter {
    getter_stub(|_| Ok(vec![]))
}

#[async_trait]
impl OldMessageGetter for SimpleOldMessageGetterStub {
    async fn get_old_messages(
        &self,
        request: GetOldMessageRequest,
    ) -> Result<Vec<MessageId>, GetError> {
        self.0(request)
    }
}

// An OldMessageController that always returns the same response when asked to read or delete
struct SimpleOldMessageDeleterStub(
    Box<dyn Send + Sync + Fn(DeleteMessagesRequest) -> Result<(), DeleteError>>,
);

pub fn deleter_stub<F>(f: F) -> impl OldMessageDeleter
where
    F: Fn(DeleteMessagesRequest) -> Result<(), DeleteError> + 'static + Sync + Send,
{
    SimpleOldMessageDeleterStub(Box::new(f))
}

pub fn deleter_noop() -> impl OldMessageDeleter {
    deleter_stub(|_| Ok(()))
}

#[async_trait]
impl OldMessageDeleter for SimpleOldMessageDeleterStub {
    async fn delete_old_messages(
        &mut self,
        request: DeleteMessagesRequest,
    ) -> Result<(), DeleteError> {
        self.0(request)
    }
}

struct SimpleNamerStub {
    channel_namer: Box<dyn Send + Sync + Fn(ChannelId) -> String>,
    guild_namer: Box<dyn Send + Sync + Fn(GuildId) -> String>,
}

pub fn namer_stub<F1, F2>(channel_namer: F1, guild_namer: F2) -> impl Namer
where
    F1: 'static + Send + Sync + Fn(ChannelId) -> String,
    F2: 'static + Send + Sync + Fn(GuildId) -> String,
{
    SimpleNamerStub {
        channel_namer: Box::new(channel_namer),
        guild_namer: Box::new(guild_namer),
    }
}

#[async_trait]
impl Namer for SimpleNamerStub {
    async fn name_guild(&self, guild_id: GuildId) -> String {
        (self.guild_namer)(guild_id)
    }

    async fn name_channel(&self, channel_id: ChannelId) -> String {
        (self.channel_namer)(channel_id)
    }
}

pub fn dummy_namer() -> impl Namer {
    namer_stub(|_| String::new(), |_| String::new())
}
