use serenity::model::prelude::*;
use chrono::{Duration, Utc};
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use std::path::{Path, PathBuf};
use std::ops::Not;

#[derive(Debug)]
pub enum Error {
	FileNotFound(PathBuf),
	NotFile(PathBuf),
	CannotRead(std::io::Error),
	Serde(serde_yaml::Error),
	CannotSave(std::io::Error),
}

impl From<serde_yaml::Error> for Error {
	fn from(e: serde_yaml::Error) -> Self {
		Error::Serde(e)
	}
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Config {
	#[serde(rename = "guilds")]
	pub guild_configs: Vec<GuildConfig>,
}

impl Config {
	pub fn empty() -> Self {
		Config{guild_configs: vec![]}
	}

	pub fn delete_schedules<'a>(&'a self) -> impl Iterator<Item = DeleteSchedule> + 'a {
		self.guild_configs.iter().flat_map(|c| c.delete_schedules())
	}
}

// GuildConfig represents the config saved for all channels in one guild.
// There should only be one of these per guild.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GuildConfig {
	#[serde(rename = "id")]
	pub guild_id: GuildId,
	#[serde(rename = "channels")]
	pub channel_configs: Vec<ChannelConfig>,
}

impl GuildConfig {
	pub fn delete_schedules<'a>(&'a self) -> impl Iterator<Item = DeleteSchedule> + 'a{
		self.channel_configs.iter().map(|c| DeleteSchedule {
			guild_id: self.guild_id,
			channel_id: c.channel_id,
			delete_older_than: c.delete_older_than,
			just_images: c.just_images,
		})
	}
}

// ChannelConfig represents the config saved for ONE channel.
// A single guild may have many or none of these.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChannelConfig {
	#[serde(rename = "id")]
	pub channel_id: ChannelId,
	#[serde(serialize_with = "duration_serialize", deserialize_with = "duration_deserialize")]
	pub delete_older_than: Duration,
	#[serde(default, skip_serializing_if = "Not::not")]
	pub just_images: bool,
}

// DeleteSchedule represents the full specification for ONE channel.
// A single guild may have many or none of these.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteSchedule {
	#[serde(rename = "id")]
	pub guild_id: GuildId,
	pub channel_id: ChannelId,
	#[serde(serialize_with = "duration_serialize", deserialize_with = "duration_deserialize")]
	pub delete_older_than: Duration,
	#[serde(default, skip_serializing_if = "Not::not")]
	pub just_images: bool,
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

	pub fn save_to_file(&self, path: &Path) -> Result<(), Error> {
		std::fs::write(path, self.to_string()?).map_err(|e| Error::CannotSave(e))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn deserializes() {
		let config = "
guilds:
- id: 3063131093886218891
  channels:
  - id: 8274993703618613416
    delete_older_than:
      days: 3
        ";

		let parsed: Config = Config::load_from_yaml(config).unwrap();

		let expected = Config {
			guild_configs: vec![
				GuildConfig{
					guild_id: GuildId::new(3063131093886218891u64),
					channel_configs: vec![
						ChannelConfig{
							just_images: false,
							channel_id: ChannelId::new(8274993703618613416u64),
							delete_older_than: Duration::days(3),
						},
					],
				},
			],
		};

		assert_eq!(expected, parsed);
	}

	#[test]
	fn deserializes_with_just_images_setting() {
		let config = "
guilds:
- id: 3063131093886218891
  channels:
  - id: 8274993703618613416
    just_images: true
    delete_older_than:
      days: 3
        ";

		let parsed: Config = Config::load_from_yaml(config).unwrap();

		let expected = Config {
			guild_configs: vec![
				GuildConfig{
					guild_id: GuildId::new(3063131093886218891u64),
					channel_configs: vec![
						ChannelConfig{
							channel_id: ChannelId::new(8274993703618613416u64),
							delete_older_than: Duration::days(3),
							just_images: true,
						},
					],
				},
			],
		};

		assert_eq!(expected, parsed);
	}

	#[test]
	fn serializes() {
		let expected = "
guilds:
- id: '3063131093886218891'
  channels:
  - id: '8274993703618613416'
    delete_older_than:
      days: 3
      hours: 5
      minutes: 7
        ".trim();

		let config = Config {
			guild_configs: vec![
				GuildConfig{
					guild_id: GuildId::new(3063131093886218891u64),
					channel_configs: vec![
						ChannelConfig{
							channel_id: ChannelId::new(8274993703618613416u64),
							delete_older_than: Duration::days(3) + Duration::minutes(7) + Duration::hours(5),
							just_images: false,
						},
					],
				},
			],
		};

		let actual: String = config.to_string().unwrap();

		assert_eq!(expected, actual.trim());
	}

	#[test]
	fn round_trip() {
		let config = Config {
			guild_configs: vec![
				GuildConfig{
					guild_id: GuildId::new(3063131093886218891u64),
					channel_configs: vec![
						ChannelConfig{
							channel_id: ChannelId::new(8274993703618613416u64),
							delete_older_than: Duration::days(3) + Duration::minutes(7) + Duration::hours(5),
							just_images: false,
						},
					],
				},
			],
		};

		let serialized: String = config.to_string().unwrap();
		let round_trip: Config = Config::load_from_yaml(&serialized).unwrap();

		assert_eq!(config, round_trip);
	}


	#[test]
	fn delete_schedules_single_schedule() {
		let config = "
guilds:
- id: '3063131093886218891'
  channels:
  - id: '8274993703618613416'
    delete_older_than:
      days: 3
      hours: 5
      minutes: 7
        ".trim();

		let config = Config::load_from_yaml(config).unwrap();


		let expected = vec![
			DeleteSchedule {
				guild_id: GuildId::new(3063131093886218891u64),
				channel_id: ChannelId::new(8274993703618613416u64),
				delete_older_than: Duration::days(3) + Duration::minutes(7) + Duration::hours(5),
				just_images: false,
			}
		];

		let actual: Vec<DeleteSchedule> = config.delete_schedules().collect();

		assert_eq!(expected, actual);
	}


	#[test]
	fn delete_schedules_many_schedules() {
		let config = "
guilds:
- id: '3063131093886218891'
  channels:
  - id: '8274993703618613416'
    delete_older_than:
      days: 3
      hours: 5
      minutes: 7
  - id: '8690347484951214837'
    just_images: true
    delete_older_than:
      hours: 1
      minutes: 30
- id: '8690347484951214837'
  channels:
  - id: '8159836460754921542'
    delete_older_than:
      days: 2
        ".trim();

		let config = Config::load_from_yaml(config).unwrap();

		let expected = vec![
			DeleteSchedule {
				guild_id: GuildId::new(3063131093886218891u64),
				channel_id: ChannelId::new(8274993703618613416u64),
				delete_older_than: Duration::days(3) + Duration::minutes(7) + Duration::hours(5),
				just_images: false,
			},
			DeleteSchedule {
				guild_id: GuildId::new(3063131093886218891u64),
				channel_id: ChannelId::new(8690347484951214837),
				delete_older_than: Duration::minutes(30) + Duration::hours(1),
				just_images: true,
			},
			DeleteSchedule {
				guild_id: GuildId::new(8690347484951214837),
				channel_id: ChannelId::new(8159836460754921542),
				delete_older_than: Duration::days(2),
				just_images: false,
			},
		];

		let actual: Vec<DeleteSchedule> = config.delete_schedules().collect();

		assert_eq!(expected, actual);
	}

}