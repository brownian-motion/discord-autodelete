use serenity::model::prelude::*;
use chrono::{Duration, DateTime, Utc, TimeZone};
use serde::{Serialize, Deserialize, Serializer, Deserializer};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteSchedule {
	pub guild_id: GuildId,
	pub channel_id: ChannelId,
	#[serde(serialize_with = "duration_serialize", deserialize_with = "duration_deserialize")]
	pub delete_older_than: Duration,
	pub last_run: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct SerializedDuration {
	#[serde(default)]
	days: u16,
	#[serde(default)]
	hours: u8,
	#[serde(default)]
	minutes: u8,
}

impl From<Duration> for SerializedDuration {
	fn from(duration: Duration) -> SerializedDuration {
	let days = duration.num_days();
	let dur = duration - Duration::days(days);
	let hours = dur.num_hours();
	let dur = dur - Duration::hours(hours);
	let minutes = dur.num_minutes();

	SerializedDuration { 
		days: days as u16, 
		hours: hours as u8,
		minutes: minutes as u8,
	}
}
}

impl Into<Duration> for SerializedDuration {
	fn into(self) -> Duration {
		Duration::days(self.days as i64) + Duration::hours(self.hours as i64) + Duration::minutes(self.minutes as i64)
	}
}

fn duration_serialize<S: Serializer>(duration: &Duration, s: S) -> Result<S::Ok, S::Error> {
	SerializedDuration::from(*duration).serialize(s)
}


fn duration_deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Duration, D::Error> {
	SerializedDuration::deserialize(d).map(|dur| dur.into())
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteSchedules {
	pub schedules: Vec<DeleteSchedule>,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn deserializes() {
		let config = "
schedules:
    - guild_id: 3063131093886218891
      channel_id: 8274993703618613416
      delete_older_than:
        days: 3
        ";

		let parsed: DeleteSchedules = serde_yaml::from_str(&config).unwrap();

		let expected = DeleteSchedules {
			schedules: vec![
				DeleteSchedule{
					guild_id: GuildId(3063131093886218891u64),
					channel_id: ChannelId(8274993703618613416u64),
					delete_older_than: Duration::days(3),
					last_run: None,
				},
			],
		};

		assert_eq!(expected, parsed);
	}

	#[test]
	fn serializes() {
		let expected = "
schedules:
- guild_id: '3063131093886218891'
  channel_id: '8274993703618613416'
  delete_older_than:
    days: 3
    hours: 5
    minutes: 7
  last_run: 2013-10-19T12:40:00Z
        ".trim();

		let config = DeleteSchedules {
			schedules: vec![
				DeleteSchedule{
					guild_id: GuildId(3063131093886218891u64),
					channel_id: ChannelId(8274993703618613416u64),
					delete_older_than: Duration::days(3) + Duration::minutes(7) + Duration::hours(5),
					last_run: Some(Utc.with_ymd_and_hms(2013,10,19,12,40,0).unwrap()),
				},
			],
		};

		let actual: String = serde_yaml::to_string(&config).unwrap();

		assert_eq!(expected, actual.trim());
	}

	#[test]
	fn round_trip() {
		let config = DeleteSchedules {
			schedules: vec![
				DeleteSchedule{
					guild_id: GuildId(3063131093886218891u64),
					channel_id: ChannelId(8274993703618613416u64),
					delete_older_than: Duration::days(3) + Duration::minutes(7) + Duration::hours(5),
					last_run: Some(Utc.with_ymd_and_hms(2013,10,19,12,40,0).unwrap()),
				},
			],
		};

		let serialized: String = serde_yaml::to_string(&config).unwrap();
		let round_trip: DeleteSchedules = serde_yaml::from_str(&serialized).unwrap();


		assert_eq!(config, round_trip);
	}
}