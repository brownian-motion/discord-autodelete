use serenity::http::HttpError;
use serenity::http::error::ErrorResponse;
use serenity::http::StatusCode;
use crate::messages::{GetError, DeleteError};
use crate::config::Error as ConfigError;

#[derive(Debug)]
pub enum Error {
	GetError(GetError),
	DeleteError(DeleteError),
	Config(ConfigError),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<GetError> for Error {
	fn from(e: GetError) -> Self {
		Error::GetError(e)
	}
}

impl From<DeleteError> for Error {
	fn from(e: DeleteError) -> Self {
		Error::DeleteError(e)
	}
}

impl From<ConfigError> for Error {
	fn from(e: ConfigError) -> Self {
		Error::Config(e)
	}
}