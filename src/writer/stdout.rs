use log::trace;

use crate::error::Result;
use crate::writer::Trade;
use crate::writer::Writer;

/// An constraint for StdOutWriter.
pub trait StdOutWriterElement {
    /// Converts a contents into a string.
    fn to_string(&self) -> String;
}

/// A writer implementation for stdout.
#[derive(Default, Debug)]
pub struct StdOutWriter {}

impl Writer for StdOutWriter {
    fn write(&mut self, trades: &[Trade]) -> Result<u64> {
        for v in trades {
            trace!("write: {:?}", v.to_string());
            println!("{:?}", v.to_string());
        }
        Ok(trades.len() as u64)
    }
}
