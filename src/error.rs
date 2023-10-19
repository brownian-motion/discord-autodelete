

pub enum Error {
	Framework(serenity::Error),
	PermissionError,
	ChannelNotFoundError,
	ServerNotFoundError,
}

pub type Result<T> = std::result::Result<T, Error>;