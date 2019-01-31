use std;
use std::fmt::{self, Display};

use serde::{de, ser};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Message(String),
    TrailingCharacters,
    NotSupported,
    EndlessString,
    InvalidBool,
    /// An [`Option`] is only allowed as the last element of the input.
    InvalidOption,
    /// An invalid char was decoded.
    InvalidChar,
    IoError(std::io::Error),
    Utf8Error(std::str::Utf8Error),
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(std::error::Error::description(self))
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Message(ref msg) => msg,
            Error::TrailingCharacters => "the input has trailing characters",
            Error::IoError(ref err) => err.description(),
            Error::Utf8Error(ref err) => err.description(),
            Error::NotSupported => "the data type is not supported",
            Error::EndlessString => "string doesn't end with \0",
            Error::InvalidBool => "invalid bool",
            Error::InvalidOption => "invalid option",
            Error::InvalidChar => "invalid char",
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IoError(err)
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(err: std::str::Utf8Error) -> Error {
        Error::Utf8Error(err)
    }
}
