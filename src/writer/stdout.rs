use log::trace;

use crate::error::Result;
use crate::writer::Trade;
use crate::writer::Writer;

pub trait StdOutWriterElement {
    fn to_string(&self) -> String;
}

#[derive(Default)]
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
