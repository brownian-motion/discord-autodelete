use crate::controller::*;
use crate::config::*;
use crate::types::*;

pub struct DeleteRoutine<G,D,N> {
	pub getter: G,
	pub deleter: D,
	pub namer: N,
}

impl<G,D,N> DeleteRoutine<G,D,N> where G: OldMessageGetter, D: OldMessageDeleter, N: Namer {
	pub async fn delete_old_messages(&mut self, config: &Config) {
		let num_schedules = config.delete_schedules().count();
	    println!("Deleting messages for {} channels in {} guilds", num_schedules, config.guild_configs.len());
	    for schedule in config.delete_schedules() {
	        let cutoff_time = schedule.oldest_permitted_message_time();
	        let guild_name = self.namer.name_guild(schedule.guild_id).await;
	        let channel_name = self.namer.name_channel(schedule.channel_id).await;
	        let guild = NamedGuild{ id: schedule.guild_id, name: guild_name };
	        let channel = NamedChannel{ id: schedule.channel_id, name: channel_name };
	        let request = GetOldMessageRequest {
	            guild: guild.clone(),
	            channel: channel.clone(),
	            sent_before: cutoff_time,
	            just_images: schedule.just_images,
	        };
	        println!("\tFetching messages from {} in {} older than {}h {}m", &channel, &guild, schedule.delete_older_than.num_hours(), schedule.delete_older_than.num_minutes() % 60);
	        let messages = match self.getter.get_old_messages(request).await {
	            Ok(messages) => messages,
	            Err(e) => {
	                eprintln!("\t\tError loading messages from {} in {}: {:?}", &channel, &guild, e);
	                continue;
	            },
	        };
	        if messages.is_empty() {
	            println!("\t\tNothing to delete for {} in {}", &channel, &guild);
	            continue;
	        }
	        let num_messages = messages.len();
	        let request = DeleteMessagesRequest { guild: guild.clone(), channel: channel.clone(), ids: messages };
	        match self.deleter.delete_old_messages(request).await {
	            Ok(_) => println!("Deleted {} old messages from {} in {}", num_messages, &channel, &guild),
	            Err(e) => eprintln!("Error deleting {} messages from {} in {}: {:?}", num_messages, &channel, &guild, e),
	        };
	    }

	    println!("\tFinished");
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use serenity::model::id::*;
	use crate::controller::stub::*;
	
	use chrono::Duration;

	#[tokio::test]
	async fn nothing_fetched_when_schedule_is_empty() {
		let mut controller = DeleteRoutine {
			getter: getter_stub(|_| panic!("Should not read!")),
			deleter: deleter_stub(|_| panic!("Should not delete!")),
			namer: dummy_namer(),
		};
		let config = Config{guild_configs: vec![]};
		controller.delete_old_messages(&config).await; // will panic if either stub is used
	}

	#[tokio::test]
	async fn nothing_deleted_when_no_messages() {
		let guild =  3063131093886218891u64;
		let channel = 8274993703618613416u64;
		let mut controller = DeleteRoutine {
			getter: getter_stub(move |req| {
				assert_eq!(req.channel.id, ChannelId::new(channel));
				assert_eq!(req.guild.id, GuildId::new(guild));
				Ok(vec![])
			}),
			deleter: deleter_stub(|_| panic!("Should not delete!")),
			namer: dummy_namer(),
		};
		let config = Config {
			guild_configs: vec![
				// just a dummy value whose contents will be ignored
				GuildConfig{
					guild_id: GuildId::new(guild),
					channel_configs: vec![
						ChannelConfig{
							channel_id: ChannelId::new(channel),
							delete_older_than: Duration::days(3) + Duration::minutes(7) + Duration::hours(5),
							just_images: false,
						},
					],
				},
			],
		};
		controller.delete_old_messages(&config).await; // will panic if the delete stub is used
	}

	#[tokio::test]
	async fn message_is_deleted_when_message_is_found() {
		let guild =  3063131093886218891u64;
		let channel = 8274993703618613416u64;
		let message = 5902119689978300948u64;
		let mut controller = DeleteRoutine {
			getter: getter_stub(move |req| {
				assert_eq!(req.channel.id, ChannelId::new(channel));
				assert_eq!(req.guild.id, GuildId::new(guild));
				Ok(vec![MessageId::new(message)])
			}),
			deleter: deleter_stub(move |req| {
				assert_eq!(req.guild.id, GuildId::new(guild));
				assert_eq!(req.channel.id, ChannelId::new(channel));
				assert_eq!(1, req.ids.len());
				assert_eq!(MessageId::new(message), req.ids[0]);
				Ok(())
			}),
			namer: dummy_namer(),
		};
		let config = Config {
			guild_configs: vec![
				// just a dummy value whose contents will be ignored
				GuildConfig{
					guild_id: GuildId::new(guild),
					channel_configs: vec![
						ChannelConfig{
							channel_id: ChannelId::new(channel),
							delete_older_than: Duration::days(3) + Duration::minutes(7) + Duration::hours(5),
							just_images: false,
						},
					],
				},
			],
		};
		controller.delete_old_messages(&config).await;
	}
}