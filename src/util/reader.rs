use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

use log::trace;

use crate::error::Result;

pub fn from_existing_file(path: &Path) -> Result<String> {
    trace!("open initial ID from {}", path.display());
    let mut file = OpenOptions::new().read(true).open(&path)?;
    let mut s = String::new();
    let size = file.read_to_string(&mut s)?;
    trace!("initial id: {}", s);
    trace!("read {} bytes", size);
    Ok(s)
}
