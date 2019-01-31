use crate::error::Result;
use crate::writer::Trade;
use crate::writer::Writer;

pub struct MockWriter {
    pub store: Vec<Trade>,
}

impl MockWriter {
    pub fn new() -> Self {
        MockWriter { store: vec![] }
    }
}

impl Writer for MockWriter {
    fn write(&mut self, trades: &[Trade]) -> Result<u64> {
        for v in trades {
            self.store.push(v.clone());
        }
        Ok(trades.len() as u64)
    }
}
