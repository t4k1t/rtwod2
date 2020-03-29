use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct UpdateError(String);

impl Error for UpdateError {}

impl fmt::Display for UpdateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "failed to update recorded IP: {}", self.0)
    }
}

impl From<reqwest::Error> for UpdateError {
    fn from(err: reqwest::Error) -> Self {
        UpdateError(err.to_string())
    }
}

#[derive(Debug)]
pub struct FetchError(String);

impl Error for FetchError {}

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "failed to fetch IP: {}", self.0)
    }
}

impl From<reqwest::Error> for FetchError {
    fn from(err: reqwest::Error) -> Self {
        FetchError(err.to_string())
    }
}
