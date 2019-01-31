use chrono::offset::TimeZone;
use chrono::Utc;

use crate::downloader::Downloader;
use crate::downloader::id::num::OrdID;
use crate::error::{Error, Result};
use crate::writer::Trade;
use crate::writer::Writer;

#[derive(Clone, Debug)]
pub struct RawData {
    pub id: u32,
}

pub struct MockDownloader {
    start_id: u32,
    end_id: u32,
    source: Vec<RawData>,
}

impl Downloader for MockDownloader {
    type IDT = u32;
    type ID = OrdID<Self::IDT>;
    type RAW = RawData;

    fn start_id(&self) -> Self::IDT {
        self.start_id
    }

    fn end_id(&self) -> Self::IDT {
        self.end_id
    }

    fn continue_condition(&self, current: &Self::IDT, end: &Self::IDT) -> bool {
        current < end
    }

    fn fetch(&self, c: &Self::IDT) -> Result<Vec<Self::RAW>> {
        let d: Vec<_> = self
            .source
            .iter()
            .cloned()
            .filter(|e| e.id >= *c)
            .take(2)
            .collect();
        if d.is_empty() {
            Err(Error::NotFound)
        } else {
            Ok(d)
        }
    }

    fn convert(&self, v: &Self::RAW) -> Result<Trade> {
        Ok(Trade {
            id: format!("{}", v.id),
            traded_at: Utc.timestamp(v.id.into(), 0),
            quantity: (v.id as f32) * 0.1,
            price: (v.id as f32) * 0.01,
        })
    }

    fn output(&self, u: Vec<Trade>, writer: &mut impl Writer) -> Result<Self::IDT> {
        writer.write(&u)?;
        u.last()
            .ok_or_else(|| Error::NotFound)
            .and_then(|e| e.id.parse::<u32>().map_err(Error::from))
            .map_err(Error::from)
            .map(|e| e + 1)
    }

    fn sleep_millis(&self) -> u64 {
        0
    }
}

impl MockDownloader {
    pub fn new(source: Vec<RawData>, start_id: u32, end_id: u32) -> Self {
        MockDownloader {
            source,
            start_id,
            end_id,
        }
    }
}
