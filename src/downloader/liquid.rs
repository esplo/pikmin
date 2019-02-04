use chrono::DateTime;
use chrono::prelude::*;
use chrono::Utc;
use log::{error, info, warn};

use crate::api::liquid::LiquidAPI;
use crate::api::liquid::LiquidGetExecution;
use crate::downloader::Downloader;
use crate::downloader::id::datetime::DateTimeID;
use crate::error::Error;
use crate::error::Result;
use crate::writer::Trade;
use crate::writer::Writer;

/// A pre-built downloader for Liquid.
#[derive(Debug)]
pub struct LiquidDownloader {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    api: LiquidAPI,
}

impl LiquidDownloader {
    fn limit(&self) -> usize {
        1000
    }

    /// Creates a new downloader with a specific range.
    /// The input source is fixed.
    pub fn new(start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        Self {
            start,
            end,
            api: LiquidAPI::new(),
        }
    }
}

impl Downloader for LiquidDownloader {
    type IDT = DateTime<Utc>;
    type ID = DateTimeID;
    type RAW = LiquidGetExecution;

    fn start_id(&self) -> DateTime<Utc> {
        self.start
    }
    fn end_id(&self) -> DateTime<Utc> {
        self.end
    }

    fn continue_condition(&self, current: &DateTime<Utc>, end: &DateTime<Utc>) -> bool {
        current <= end
    }

    fn fetch(&self, c: &Self::IDT) -> Result<Vec<LiquidGetExecution>> {
        self.api.executions(c.timestamp() as u64, self.limit())
    }

    fn convert(&self, v: &LiquidGetExecution) -> Result<Trade> {
        let quantity = v.quantity.parse::<f32>()?;
        let price = v.price.parse::<f32>()?;
        let created_at = Utc.timestamp(v.created_at as i64, 0);

        Ok(Trade {
            id: format!("{}", v.id),
            quantity,
            price,
            traded_at: created_at,
        })
    }

    /// Liquid (by Quoine) API has inconsistent statements in the document (https://developers.quoine.com/#get-executions-by-timestamp).
    /// As `Get Executions by Timestamp` won't return the complete executions at the same timestamp,
    /// this function has to re-request the executions for the timestamp so as to obtain all of them.
    ///
    /// # Errors
    /// In order to avoid an infinite loop, this function will return an error
    /// when there are more than 1,000 executions at the same timestamp.
    fn output(&self, u: Vec<Trade>, writer: &mut impl Writer) -> Result<Self::IDT> {
        match u.last() {
            Some(last) => {
                let last_ts = last.traded_at;
                let orig_len = u.len();
                let without_last: Vec<Trade> =
                    u.into_iter().filter(|e| e.traded_at != last_ts).collect();

                if orig_len == self.limit() && without_last.is_empty() {
                    error!(
                        "more than {} executions at the same timestamp",
                        self.limit()
                    );
                    Err(Error::CannotFetchTradesAccurately)
                } else {
                    writer.write(without_last.as_slice()).map(|num| {
                        info!("wrote {} data", num);
                        last_ts
                    })
                }
            }
            None => {
                warn!("no output");
                Err(Error::NotFound)
            }
        }
    }

    fn sleep_millis(&self) -> u64 {
        1100
    }
}
