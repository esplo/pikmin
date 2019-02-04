use std::fmt;
use std::fmt::Display;
use std::marker::PhantomData;
use std::str::FromStr;

use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_derive::{Deserialize, Serialize};

use crate::downloader::id::DownloaderID;
use crate::error::Error;
use crate::error::Result;

/// An pagination. This keeps original ID and a pagination value.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Pagination<U, T> {
    id: T,
    num: u64,
    phantom: PhantomData<U>,
}

impl<U, T> Pagination<U, T>
    where
        T: DownloaderID<U> + DeserializeOwned + Serialize,
        U: Display,
{
    /// Creates new pagination with ID.
    pub fn new(id: T, num: u64) -> Self {
        Self {
            id,
            num,
            phantom: PhantomData,
        }
    }

    /// Decomposes the element of a pagination.
    pub fn tuple(&self) -> (&U, &u64) {
        (&self.id.current(), &self.num)
    }
}

impl<U, T> FromStr for Pagination<U, T>
    where
        T: DownloaderID<U> + DeserializeOwned + Serialize,
        U: Display,
{
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        serde_json::from_str(s).map_err(Error::from)
    }
}

impl<U, T> Display for Pagination<U, T>
    where
        T: DownloaderID<U> + DeserializeOwned + Serialize,
        U: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

/// An wrapper for ID with pagination.
#[derive(Debug, Deserialize, Serialize)]
pub struct PaginatedID<U, T> {
    value: Pagination<U, T>,
    phantom: PhantomData<U>,
}

impl<U, T> PaginatedID<U, T>
    where
        T: DownloaderID<U> + DeserializeOwned + Serialize,
        U: Display,
{
    fn new(value: Pagination<U, T>) -> Self {
        Self {
            value,
            phantom: PhantomData,
        }
    }
}

impl<U, T> DownloaderID<Pagination<U, T>> for PaginatedID<U, T>
    where
        T: DownloaderID<U> + DeserializeOwned + Serialize,
        U: Display,
{
    fn current(&self) -> &Pagination<U, T> {
        &self.value
    }

    fn update(&mut self, c: Pagination<U, T>) -> Result<()> {
        let offset = if c.num == 0 { 0 } else { self.value.num + c.num };
        self.value = Pagination::new(c.id, offset);
        Ok(())
    }
}

impl<U, T> From<Pagination<U, T>> for PaginatedID<U, T>
    where
        T: DownloaderID<U> + DeserializeOwned + Serialize,
        U: Display,
{
    fn from(v: Pagination<U, T>) -> Self {
        Self::new(v)
    }
}
