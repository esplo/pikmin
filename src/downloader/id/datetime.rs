use std::fmt;

use chrono::DateTime;
use chrono::Utc;
use serde_derive::{Deserialize, Serialize};

use crate::downloader::id::DownloaderID;
use crate::error::Result;

/// An ID implementation by chrono::DateTime<Utc>.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DateTimeID {
    current: DateTime<Utc>,
}

impl DateTimeID {
    /// Creates an ID with a given time.
    pub fn new(current: DateTime<Utc>) -> Self {
        Self { current }
    }
}

impl DownloaderID<DateTime<Utc>> for DateTimeID {
    fn current(&self) -> &DateTime<Utc> {
        &self.current
    }

    fn update(&mut self, c: DateTime<Utc>) -> Result<()> {
        self.current = c;
        Ok(())
    }

    fn to_string(&self) -> String {
        self.current.format("%Y-%m-%dT%H:%M:%SZ").to_string()
    }
}

impl From<DateTime<Utc>> for DateTimeID {
    fn from(idt: DateTime<Utc>) -> Self {
        Self::new(idt)
    }
}

impl fmt::Display for DateTimeID where {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.current().to_string())
    }
}
