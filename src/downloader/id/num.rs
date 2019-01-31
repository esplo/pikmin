use std::fmt::Display;

use crate::downloader::id::DownloaderID;
use crate::error::Result;

pub struct OrdID<T: Ord> {
    current: T,
}

impl<T: Ord> OrdID<T> {
    pub fn new(current: T) -> Self {
        OrdID { current }
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
        OrdID::new(idt)
    }
}
