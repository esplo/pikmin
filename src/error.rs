use std::error::Error as _Error;
use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    ParseFloat(std::num::ParseFloatError),
    ParseInt(std::num::ParseIntError),
    ParseInitialValue,
    MySql(Box<mysql::error::Error>),
    MySqlMissingNamedParameter(Box<std::error::Error>),
    NotFound,
    /// An `io::Error` that occurred while trying to read or write to a network stream.
    IO(std::io::Error),
    ChronoParse(chrono::format::ParseError),
    CannotFetchExecutions,
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
            Error::ParseInitialValue => "Cannot parse the initial value",
            Error::MySql(ref e) => e.description(),
            Error::MySqlMissingNamedParameter(ref e) => e.description(),
            Error::NotFound => "No data found",
            Error::IO(ref e) => e.description(),
            Error::ChronoParse(ref e) => e.description(),
            Error::CannotFetchExecutions => "Cannot fetch from API",
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
