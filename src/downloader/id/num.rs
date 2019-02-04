use std::fmt::Display;

use crate::downloader::id::DownloaderID;
use crate::error::Result;

/// An ID implementation by integer numbers.
#[derive(Debug)]
pub struct OrdID<T> {
    current: T,
}

impl<T: Ord> OrdID<T> {
    /// Creates an ID with a given number.
    pub fn new(current: T) -> Self {
        Self { current }
    }
}

impl<T: Ord + Display> DownloaderID<T> for OrdID<T> {
    fn current(&self) -> &T {
        &self.current
    }

    fn update(&mut self, c: T) -> Result<()> {
        self.current = c;
        Ok(())
    }

    fn to_string(&self) -> String {
        format!("{}", self.current)
    }
}

impl<T: Ord> From<T> for OrdID<T> {
    fn from(idt: T) -> Self {
        Self::new(idt)
    }
}
