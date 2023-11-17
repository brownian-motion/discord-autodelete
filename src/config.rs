use serenity::model::prelude::*;
use chrono::{Duration, DateTime, Utc, TimeZone};
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum Error {
	FileNotFound(PathBuf),
	NotFile(PathBuf),
	CannotRead(std::io::Error),
	Serde(serde_yaml::Error),
}

impl From<serde_yaml::Error> for Error {
	fn from(e: serde_yaml::Error) -> Self {
		Error::Serde(e)
	}
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Config {
	pub schedules: Vec<DeleteSchedule>,
}

// DeleteSchedule represents the specifications for ONE channel.
// A single guild may have many or none of these.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteSchedule {
	pub guild_id: GuildId,
	pub channel_id: ChannelId,
	#[serde(serialize_with = "duration_serialize", deserialize_with = "duration_deserialize")]
	pub delete_older_than: Duration,
	pub last_run: Option<DateTime<Utc>>, // TODO: move this to some other cache, so they're saved seperately
}

impl DeleteSchedule {
	pub fn oldest_permitted_message_time(&self) -> Timestamp {
		(Utc::now() - self.delete_older_than).into()
	}
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

impl Config{
	pub fn load_from_file(path: &Path) -> Result<Config, Error> {
		use Error::*;
		if !path.exists() {
			return Err(FileNotFound(path.to_path_buf()))
		}
		if !path.is_file() {
			return Err(NotFile(path.to_path_buf()))
		}
		match std::fs::read_to_string(path) {
			Ok(s) => Self::load_from_yaml(&s),
			Err(e) => Err(CannotRead(e)),
		}
	}

	pub fn load_from_yaml(data: &str) -> Result<Config, Error> {
		Ok(serde_yaml::from_str(data)?)
	}

	pub fn to_string(&self) -> Result<String, Error> {
		Ok(serde_yaml::to_string(self)?)
	}
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

		let parsed: DeleteSchedules = Config::load_from_yaml(config).unwrap();

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

		let actual: String = config.to_string().unwrap();

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

		let serialized: String = config.to_string().unwrap();
		let round_trip: DeleteSchedules = Config::load_from_yaml(&serialized).unwrap();

		assert_eq!(config, round_trip);
	}
}