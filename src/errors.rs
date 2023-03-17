use core::fmt::Display;
use url::ParseError;

#[derive(Debug)]
pub enum WeatherError {
    UrlParseError(ParseError),
    RequestError(reqwest::Error),
    SerdeError(serde_json::Error),
}

impl Display for WeatherError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WeatherError::UrlParseError(parse_int_error) => write!(f, "{}", parse_int_error),
            WeatherError::RequestError(io_error) => write!(f, "{}", io_error),
            WeatherError::SerdeError(serder_error) => write!(f, "{}", serder_error),
        }
    }
}

impl From<reqwest::Error> for WeatherError {
    fn from(err: reqwest::Error) -> Self {
        WeatherError::RequestError(err)
    }
}

impl From<ParseError> for WeatherError {
    fn from(err: ParseError) -> Self {
        WeatherError::UrlParseError(err)
    }
}

impl From<serde_json::Error> for WeatherError {
    fn from(err: serde_json::Error) -> Self {
        WeatherError::SerdeError(err)
    }
}

impl std::error::Error for WeatherError {}
