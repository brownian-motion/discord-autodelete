use serenity::http::HttpError;
use serenity::http::ErrorResponse;
use serenity::http::StatusCode;

#[derive(Debug)]
pub enum GetError {
	Framework(serenity::Error),
	Http(HttpError),
	CannotFetchMessages(ErrorResponse),
	ChannelNotFoundError,
	ServerNotFoundError,
	Forbidden,
}

impl From<serenity::Error> for GetError {
	fn from(e: serenity::Error) -> Self {
		if let serenity::Error::Http(http_err) = e {
			return http_err.into()
		}
		GetError::Framework(e)
	}
}

impl From<HttpError> for GetError {
	fn from(e: HttpError) -> Self {
		if let HttpError::UnsuccessfulRequest(resp) = e {
			return match resp.status_code {
				StatusCode::FORBIDDEN => GetError::Forbidden,
				StatusCode::NOT_FOUND => GetError::ChannelNotFoundError,
				_ => GetError::CannotFetchMessages(resp),
			}
		}
		GetError::Http(e)
	}
}

#[derive(Debug)]
pub enum DeleteError {
	Framework(serenity::Error),
	Http(HttpError),
	CannotDeleteMessages(ErrorResponse),
	MessageNotFoundError,
	Forbidden,
}

impl From<serenity::Error> for DeleteError {
	fn from(e: serenity::Error) -> Self {
		if let serenity::Error::Http(http_err) = e {
			return http_err.into()
		}
		DeleteError::Framework(e)
	}
}

impl From<HttpError> for DeleteError {
	fn from(e: HttpError) -> Self {
		if let HttpError::UnsuccessfulRequest(resp) = e {
			return match resp.status_code {
				StatusCode::FORBIDDEN => DeleteError::Forbidden,
				StatusCode::NOT_FOUND => DeleteError::MessageNotFoundError,
				_ => DeleteError::CannotDeleteMessages(resp),
			}
		}
		DeleteError::Http(e)
	}
}
