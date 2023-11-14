use serenity::http::HttpError;
use serenity::http::error::ErrorResponse;
use serenity::http::StatusCode;

#[derive(Debug)]
pub enum Error {
	Framework(serenity::Error),
	Http(HttpError),
	CannotFetchMessages(ErrorResponse),
	CannotDeleteMessages(ErrorResponse),
	ChannelNotFoundError,
	ServerNotFoundError,
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<serenity::Error> for Error {
	fn from(e: serenity::Error) -> Self {
		if let serenity::Error::Http(http_err) = e {
			return (*http_err).into()
		}
		Error::Framework(e)
	}
}

impl From<HttpError> for Error {
	fn from(e: HttpError) -> Self {
		if let HttpError::UnsuccessfulRequest(resp) = e {
			match resp.status_code {
				StatusCode::FORBIDDEN => Error::CannotFetchMessages(resp),
				StatusCode::NOT_FOUND => Error::ChannelNotFoundError,
				_ => Error::Http(HttpError::UnsuccessfulRequest(resp)),
			}
		}
		Error::Http(e)
	}
}