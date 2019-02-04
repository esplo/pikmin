use crate::error::Result;

/// A progress recorder on a file.
pub mod file;
/// A progress recorder on memory.
pub mod memory;

/// An abstraction of a progress recorder, with reading and writing.
pub trait ProgressRecorder {
    /// Reads current progress from somewhere.
    fn read(&self) -> Result<String>;
    /// Writes current progress into somewhere.
    fn out(&mut self, json: &str) -> Result<()>;
}
