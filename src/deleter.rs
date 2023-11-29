use crate::messages::*;
use crate::config::*;

pub struct DeleteRoutine<G,D> {
	pub getter: G,
	pub deleter: D,
}

impl<G,D> DeleteRoutine<G,D> where G: OldMessageGetter, D: OldMessageDeleter {
	pub async fn delete_old_messages(&mut self, config: &Config) {
	    println!("Deleting messages for {} schedules", config.schedules.len());
	    for schedule in &config.schedules {
	        // TODO: just pass the schedule instead
	        let cutoff_time = schedule.oldest_permitted_message_time();
	        let request = GetOldMessageRequest {
	            guild_id: schedule.guild_id,
	            channel_id: schedule.channel_id,
	            sent_before: cutoff_time,
	        };
	        println!("Fetching messages in channel {:?} older than {}h {}m", schedule.channel_id, schedule.delete_older_than.num_hours(), schedule.delete_older_than.num_minutes() % 60);
	        let messages = match self.getter.get_old_messages(request).await {
	            Ok(messages) => messages,
	            Err(e) => {
	                eprintln!("Error loading messages from channel {:?}: {:?}", schedule.channel_id, e);
	                continue;
	            },
	        };
	        if messages.is_empty() {
	            println!("Nothing to delete for channel {:?}", schedule.channel_id);
	            continue;
	        }
	        match self.deleter.delete_old_messages(&schedule.guild_id, &schedule.channel_id, &messages).await {
	            Ok(_) => println!("Deleted {} old messages from channel {:?}", messages.len(), schedule.channel_id),
	            Err(e) => eprintln!("Error deleting {} messages from channel {:?}: {:?}", messages.len(), schedule.channel_id, e),
	        };
	    }

	    println!("Finished deleting from {} channels", config.schedules.len());
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::messages::stubs::*;

	#[tokio::test]
	async fn nothing_fetched_when_schedule_is_empty() {
		let controller = DeleteRoutine {
			getter: getter_stub(|_| panic!("Should not read!")),
			deleter: deleter_stub(|_,_,_| panic!("Should not delete!")),
		};
		let config = Config{schedules: vec![]};
		controller.delete_old_messages(&config).await; // will panic if either stub is used
	}

	#[tokio::test]
	async fn nothing_deleted_when_no_messages() {
		let guild =  3063131093886218891u64;
		let channel = 8274993703618613416u64;
		let controller = DeleteRoutine {
			getter: getter_stub(move |req| {
				assert_eq!(req.channel_id, ChannelId(channel));
				assert_eq!(req.guild_id, GuildId(guild));
				Ok(vec![])
			}),
			deleter: deleter_stub(|_,_,_| panic!("Should not delete!")),
		};
		let config = Config {
			schedules: vec![
				// just a dummy value whose contents will be ignored
				DeleteSchedule{
					guild_id: GuildId(guild),
					channel_id: ChannelId(channel),
					delete_older_than: Duration::days(3) + Duration::minutes(7) + Duration::hours(5),
					last_run: Some(Utc.with_ymd_and_hms(2013,10,19,12,40,0).unwrap()),
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
		let controller = DeleteRoutine {
			getter: getter_stub(move |req| {
				assert_eq!(req.channel_id, ChannelId(channel));
				assert_eq!(req.guild_id, GuildId(guild));
				Ok(vec![MessageId(message)])
			}),
			deleter: deleter_stub(move |&gid, &cid, messages| {
				assert_eq!(gid, GuildId(guild));
				assert_eq!(cid, ChannelId(channel));
				assert_eq!(1, messages.len());
				assert_eq!(MessageId(message), messages[0]);
				Ok(())
			}),
		};
		let config = Config {
			schedules: vec![
				// just a dummy value whose contents will be ignored
				DeleteSchedule{
					guild_id: GuildId(guild),
					channel_id: ChannelId(channel),
					delete_older_than: Duration::days(3) + Duration::minutes(7) + Duration::hours(5),
					last_run: Some(Utc.with_ymd_and_hms(2013,10,19,12,40,0).unwrap()),
				},
			],
		};
		controller.delete_old_messages(&config).await;
	}
}