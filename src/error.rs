#[derive(Debug)]
pub enum Error {
	Framework(serenity::Error),
	PermissionError,
	ChannelNotFoundError,
	ServerNotFoundError,
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<serenity::Error> for Error {
	fn from(e: serenity::Error) -> Self {
		Error::Framework(e)
	}
}