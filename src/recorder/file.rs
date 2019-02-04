use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

use log::trace;

use crate::error::Result;
use crate::recorder::ProgressRecorder;

/// A progress recorder on a file.
#[derive(Debug)]
pub struct FileRecorder {
    path: PathBuf,
}

impl FileRecorder {
    /// Creates a new file recorder with a given path.
    pub fn new(path: PathBuf) -> Self {
        Self {
            path
        }
    }
}

impl ProgressRecorder for FileRecorder {
    fn read(&self) -> Result<String> {
        trace!("read from {}", self.path.display());
        let mut file = OpenOptions::new().read(true).open(&self.path)?;
        let mut s = String::new();
        let size = file.read_to_string(&mut s)?;
        trace!("read: {} ({} bytes)", s, size);
        Ok(s)
    }

    fn out(&mut self, json: &str) -> Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(self.path.clone())?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }
}
