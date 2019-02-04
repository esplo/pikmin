use std::fmt::Display;

use crate::error::Result;

/// An ID implementation by chrono::DateTime<Utc>.
pub mod datetime;
/// An ID implementation by integer numbers.
pub mod num;
/// An ID Wrapper for pagination.
pub mod paginated;

/// An abstraction for IDs. This trait must be implemented if you want to create a downloader
/// with a new ID management.
pub trait DownloaderID<T: Display> {
    /// Returns current ID.
    fn current(&self) -> &T;
    /// Updates current ID with a given ID.
    fn update(&mut self, c: T) -> Result<()>;
    /// Converts current ID to a String value in order to record the progress.
    fn to_string(&self) -> String;
}
