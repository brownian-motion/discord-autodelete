use crate::controller::{error::*, traits::*};
use crate::types::*;
use async_trait::async_trait;
use futures::prelude::*;
use log::trace;
use serenity::http::Http;
use serenity::model::{channel::Message, id::MessageId};

pub struct OldMessageController<H> {
    http: H,
}

impl<H> OldMessageController<H>
where
    H: AsRef<Http> + Sync,
{
    pub fn new(http: H) -> Self {
        OldMessageController { http }
    }
}

impl GetOldMessageRequest {
    fn matches(&self, message: &Message) -> bool {
        trace!(channel_id = message.channel_id.get(), guild_id = message.guild_id.map(|id| id.get()), message_id = message.id.get(); "Considering message");
        message.channel_id == self.channel.id
			&& !message.pinned
			// Timestamp doesn't implement `<`, so we compare the equivalent Unix timestamp instead
			&& message.timestamp.timestamp() < self.sent_before.timestamp()
			// only delete messages without images if configured to
			&& (!self.just_images || !message.embeds.is_empty() || !message.attachments.is_empty())
    }
}

#[async_trait]
impl<H> OldMessageGetter for OldMessageController<H>
where
    H: AsRef<Http> + Sync,
{
    async fn get_old_messages(
        &self,
        request: GetOldMessageRequest,
    ) -> Result<Vec<MessageId>, GetError> {
        // for now , assume the IDs can all fit in memory
        let mut stream = request.channel.id.messages_iter(self.http.as_ref()).boxed();
        let mut ids = vec![];
        while let Some(res) = stream.next().await {
            match res {
                Ok(m) => {
                    if request.matches(&m) {
                        ids.push(m.id);
                    }
                }
                Err(e) => return Err(e.into()),
            }
        }
        Ok(ids)
    }
}

impl<H> OldMessageController<H>
where
    H: AsRef<Http> + Sync + Send,
{
    async fn delete_single_message(
        &mut self,
        channel_id: ChannelId,
        message_id: MessageId,
    ) -> Result<(), DeleteError> {
        // for now , assume the IDs can all fit in memory
        let http = self.http.as_ref();
        channel_id.delete_message(http, message_id).await?;
        let _ = http;
        Ok(())
    }
    async fn delete_bulk_messages(
        &mut self,
        channel_id: ChannelId,
        message_ids: &[MessageId],
    ) -> Result<(), DeleteError> {
        // for now , assume the IDs can all fit in memory
        let http = self.http.as_ref();
        channel_id.delete_messages(http, message_ids).await?;
        let _ = http;
        Ok(())
    }
}

#[async_trait]
impl<H> OldMessageDeleter for OldMessageController<H>
where
    H: AsRef<Http> + Sync + Send,
{
    async fn delete_old_messages(
        &mut self,
        request: DeleteMessagesRequest,
    ) -> Result<(), DeleteError> {
        // first try using bulk deletion
        if self
            .delete_bulk_messages(request.channel.id, &request.ids)
            .await
            .is_ok()
        {
            return Ok(());
        }
        // if that doesn't work, try single-message deletion
        for message_id in request.ids {
            self.delete_single_message(request.channel.id, message_id)
                .await?
        }
        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use crate::types::*;
    use serenity::model::channel::Message;
    use table_test::*;

    const GUILD_ONE: NamedGuild = NamedGuild {
        name: String::new(),
        id: GuildId::new(7452357945687619511),
    };
    const CHANNEL_ONE: NamedChannel = NamedChannel {
        name: String::new(),
        id: ChannelId::new(4878609913481757359),
    };

    #[test]
    fn matches() {
        let old_time = Timestamp::parse("2016-04-30T11:18:25Z").unwrap();
        let new_time = Timestamp::parse("2023-12-13T21:21:59Z").unwrap();

        let old_message = || {
            let mut m: Message = Default::default();
            m.guild_id = Some(GUILD_ONE.id);
            m.channel_id = CHANNEL_ONE.id;
            m.timestamp = old_time;
            m
        };
        let new_message = || {
            let mut m = old_message();
            m.timestamp = new_time;
            m
        };
        // THIS MUST NOT BE ACCESSED
        // let unsafe_empty_attachment = || unsafe {
        // 	const SIZE: usize = std::mem::size_of::<Attachment>();
        // 	std::mem::transmute::<[u8; SIZE], Attachment>([0u8; SIZE])
        // };

        let test_cases = vec![
            (
                (
                    "old message",
                    GetOldMessageRequest {
                        guild: GUILD_ONE,
                        channel: CHANNEL_ONE,
                        sent_before: Timestamp::parse("2020-01-01T01:00:00Z").unwrap(),
                        just_images: false,
                    },
                    old_message(),
                ),
                true,
            ),
            (
                (
                    "new message",
                    GetOldMessageRequest {
                        guild: GUILD_ONE,
                        channel: CHANNEL_ONE,
                        sent_before: Timestamp::parse("2020-01-01T01:00:00Z").unwrap(),
                        just_images: false,
                    },
                    new_message(),
                ),
                false,
            ),
            (
                (
                    "old pinned message",
                    GetOldMessageRequest {
                        guild: GUILD_ONE,
                        channel: CHANNEL_ONE,
                        sent_before: Timestamp::parse("2020-01-01T01:00:00Z").unwrap(),
                        just_images: false,
                    },
                    {
                        let mut m = old_message();
                        m.pinned = true;
                        m
                    },
                ),
                false,
            ),
            (
                (
                    "just images => doesn't get text-only",
                    GetOldMessageRequest {
                        guild: GUILD_ONE,
                        channel: CHANNEL_ONE,
                        sent_before: Timestamp::parse("2020-01-01T01:00:00Z").unwrap(),
                        just_images: true,
                    },
                    old_message(),
                ),
                false,
            ),
            // (
            // 	(
            // 		"just images => gets messages with attachments",
            // 		GetOldMessageRequest {
            // 			guild: GUILD_ONE,
            // 			channel: CHANNEL_ONE,
            // 			sent_before: Timestamp::parse("2020-01-01T01:00:00Z").unwrap(),
            // 			just_images: true,
            // 		},
            // 		{
            // 			let mut m = old_message();
            // 			m.attachments = vec![unsafe_empty_attachment()];
            // 			m
            // 		}
            // 	),
            // 	true
            // ),
            (
                (
                    "just images => gets messages with embeds",
                    GetOldMessageRequest {
                        guild: GUILD_ONE,
                        channel: CHANNEL_ONE,
                        sent_before: Timestamp::parse("2020-01-01T01:00:00Z").unwrap(),
                        just_images: true,
                    },
                    {
                        let mut m = old_message();
                        m.embeds = vec![Default::default()];
                        m
                    },
                ),
                true,
            ),
            (
                (
                    "just images => skips pinned messages with embeds",
                    GetOldMessageRequest {
                        guild: GUILD_ONE,
                        channel: CHANNEL_ONE,
                        sent_before: Timestamp::parse("2020-01-01T01:00:00Z").unwrap(),
                        just_images: true,
                    },
                    {
                        let mut m = old_message();
                        m.pinned = true;
                        m.embeds = vec![Default::default()];
                        m
                    },
                ),
                false,
            ),
            // (
            // 	(
            // 		"just images => skips pinned messages with attachments",
            // 		GetOldMessageRequest {
            // 			guild: GUILD_ONE,
            // 			channel: CHANNEL_ONE,
            // 			sent_before: Timestamp::parse("2020-01-01T01:00:00Z").unwrap(),
            // 			just_images: true,
            // 		},
            // 		{
            // 			let mut m = old_message();
            // 			m.pinned = true;
            // 			m.attachments = vec![unsafe_empty_attachment()];
            // 			m
            // 		}
            // 	),
            // 	false
            // ),
        ];

        for (validator, (title, request, message), expected) in table_test!(test_cases) {
            validator
                .description(title)
                .given(&format!("{:?}", &message))
                .when(&format!("{:?}", &request))
                .then(&format!("matches: {}", &expected))
                .assert_eq(expected, request.matches(&message));
        }
    }
}
