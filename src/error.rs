use std::fmt;
use std::io::Error as IoError;
use std::error::Error as ErrorTrait;
use std::convert::From;

#[derive(Debug)]
pub struct Error {
    pub message: String
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.message)
    }
}

impl ErrorTrait for Error {
    fn description(&self) -> &str {
        self.message.as_str()
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Error {
        Error {message: String::from(err.description())}
    }
}