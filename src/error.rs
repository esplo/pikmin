use std::error::Error as _Error;
use std::fmt;

/// An alias for representing a downloading result easily.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors encountered by the downloading.
#[derive(Debug)]
pub enum Error {
    /// An `reqwest::Error` that occurred while fetching from an external API.
    Reqwest(reqwest::Error),
    /// An `std::num::ParseFloatError` that occurred while converting a value to a Float value.
    ParseFloat(std::num::ParseFloatError),
    /// An `std::num::ParseIntError` that occurred while converting a value to a Int value.
    ParseInt(std::num::ParseIntError),
    /// An error that occurred while reading an ID value from a string.
    ParseValueFromStr,
    /// An `mysql::error::Error` that occurred in general MySQL processing.
    MySql(Box<mysql::error::Error>),
    /// An error that occurred when parameter mappings are invalid.
    MySqlMissingNamedParameter(Box<std::error::Error>),
    /// An error that occurred when fetched data is empty.
    NotFound,
    /// An `io::Error` that occurred while trying to read or write to a network stream.
    IO(std::io::Error),
    /// An `chrono::format::ParseError` that occurred while converting a value to a chrono::DateTime.
    ChronoParse(chrono::format::ParseError),
    /// An error that occurred when it is impossible to fetch all trade data accurately since
    /// there are too many record at the same timestamp.
    CannotFetchTradesAccurately,
    /// An error that occurred when it failed to parse as a json value.
    ParseJson(serde_json::error::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Reqwest(ref e) => e.fmt(f),
            Error::ParseFloat(ref e) => e.fmt(f),
            Error::ParseInt(ref e) => e.fmt(f),
            Error::MySql(ref e) => e.fmt(f),
            Error::MySqlMissingNamedParameter(ref e) => e.fmt(f),
            Error::IO(ref e) => e.fmt(f),
            Error::ChronoParse(ref e) => e.fmt(f),
            Error::ParseJson(ref e) => e.fmt(f),

            ref e => f.write_str(e.description()),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Reqwest(ref e) => e.description(),
            Error::ParseFloat(ref e) => e.description(),
            Error::ParseInt(ref e) => e.description(),
            Error::ParseValueFromStr => "Cannot parse the initial value",
            Error::MySql(ref e) => e.description(),
            Error::MySqlMissingNamedParameter(ref e) => e.description(),
            Error::NotFound => "No data found",
            Error::IO(ref e) => e.description(),
            Error::ChronoParse(ref e) => e.description(),
            Error::CannotFetchTradesAccurately => "Cannot fetch from API",
            Error::ParseJson(ref e) => e.description(),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::Reqwest(err)
    }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(err: std::num::ParseFloatError) -> Error {
        Error::ParseFloat(err)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Error {
        Error::ParseInt(err)
    }
}

impl From<mysql::error::Error> for Error {
    fn from(err: mysql::error::Error) -> Error {
        Error::MySql(Box::new(err))
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IO(err)
    }
}

impl From<chrono::format::ParseError> for Error {
    fn from(err: chrono::format::ParseError) -> Error {
        Error::ChronoParse(err)
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(err: serde_json::error::Error) -> Error {
        Error::ParseJson(err)
    }
}
