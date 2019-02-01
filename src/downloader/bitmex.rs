use chrono::DateTime;
use chrono::Utc;
use log::{error, info, warn};

use crate::api::mex::MexAPI;
use crate::api::mex::MexGetExecution;
use crate::downloader::Downloader;
use crate::downloader::id::datetime::DateTimeID;
use crate::error::Error;
use crate::error::Result;
use crate::writer::Trade;
use crate::writer::Writer;

/// A pre-built downloader for BitMEX.
#[derive(Debug)]
pub struct MexDownloader {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    api: MexAPI,
}

impl MexDownloader {
    fn limit(&self) -> usize {
        500
    }

    /// Creates a new downloader with a specific range.
    /// The input source is fixed.
    pub fn new(start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        MexDownloader {
            start,
            end,
            api: MexAPI::new(),
        }
    }
}

impl Downloader for MexDownloader {
    type IDT = DateTime<Utc>;
    type ID = DateTimeID;
    type RAW = MexGetExecution;

    fn start_id(&self) -> DateTime<Utc> {
        self.start
    }
    fn end_id(&self) -> DateTime<Utc> {
        self.end
    }

    fn continue_condition(&self, current: &DateTime<Utc>, end: &DateTime<Utc>) -> bool {
        current <= end
    }

    fn fetch(&self, c: &Self::IDT) -> Result<Vec<MexGetExecution>> {
        self.api.executions(c, self.limit())
    }

    fn convert(&self, v: &MexGetExecution) -> Result<Trade> {
        let btc_amount = v.size as f32 / v.price;
        // TODO: parse from JSON
        let quantity = if v.side == "Buy" {
            btc_amount
        } else {
            -1.0 * btc_amount
        };
        let price = v.price;
        let traded_at = v.timestamp;

        Ok(Trade {
            id: v.trdMatchID.clone(),
            quantity,
            price,
            traded_at,
        })
    }

    /// Possibly some trades will be cut from the response if we just increment the starting
    /// timestamp since the API will return just 1,000 trades for single request.
    /// In order to avoid this, the last trades in terms of the traded timestamp should be
    /// ignored at the first time, and request again from the timestamp.
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

    /// if we login, the API limit becomes 300/5min
    /// if not, it's 150/5min
    fn sleep_millis(&self) -> u64 {
        (1000 * 60) / 30
    }
}
