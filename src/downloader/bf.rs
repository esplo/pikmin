use chrono::prelude::*;
use chrono::Utc;
use log::{info, warn};

use crate::api::bf::BfAPI;
use crate::api::bf::BfGetExecution;
use crate::downloader::Downloader;
use crate::downloader::id::num::OrdID;
use crate::error::Error;
use crate::error::Result;
use crate::writer::Trade;
use crate::writer::Writer;

pub struct BfDownloader {
    start: u64,
    end: u64,
    api: BfAPI,
}

impl BfDownloader {
    fn limit(&self) -> usize {
        1000
    }

    pub fn new(newer: u64, older: u64) -> Self {
        BfDownloader {
            start: older,
            end: newer,
            api: BfAPI::new(),
        }
    }
}

impl Downloader for BfDownloader {
    type IDT = u64;
    type ID = OrdID<Self::IDT>;
    type RAW = BfGetExecution;

    fn start_id(&self) -> Self::IDT {
        self.start
    }

    fn end_id(&self) -> Self::IDT {
        self.end
    }

    fn continue_condition(&self, current: &Self::IDT, end: &Self::IDT) -> bool {
        current > end
    }

    fn fetch(&self, c: &Self::IDT) -> Result<Vec<Self::RAW>> {
        self.api.executions(*c, self.limit())
    }

    fn convert(&self, v: &Self::RAW) -> Result<Trade> {
        let quantity = if v.side == "BUY" {
            v.size
        } else {
            -1.0 * v.size
        };
        let price = v.price;
        let traded_at = Utc.datetime_from_str(&v.exec_date, "%Y-%m-%dT%H:%M:%S%.3f")?;

        Ok(Trade {
            id: format!("{}", v.id),
            quantity,
            price,
            traded_at,
        })
    }

    fn output(&self, u: Vec<Trade>, writer: &mut impl Writer) -> Result<Self::IDT> {
        match u.last() {
            Some(last) => {
                writer.write(u.as_slice()).and_then(|num| {
                    info!("wrote {} data", num);
                    // TODO: redundant
                    last.id.parse::<u64>().map_err(Error::from)
                })
            }
            None => {
                warn!("no output");
                Err(Error::NotFound)
            }
        }
    }

    fn sleep_millis(&self) -> u64 {
        (1000 * 60) / 500 + 10
    }
}
