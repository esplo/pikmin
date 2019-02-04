use chrono::DateTime;
use chrono::Utc;
use log::{debug, info, warn};

use crate::api::mex::MexAPI;
use crate::api::mex::MexGetExecution;
use crate::downloader::Downloader;
use crate::downloader::id::datetime::DateTimeID;
use crate::downloader::id::paginated::PaginatedID;
use crate::downloader::id::paginated::Pagination;
use crate::error::Error;
use crate::error::Result;
use crate::writer::Trade;
use crate::writer::Writer;

/// A pre-built downloader for BitMEX.
#[derive(Debug)]
pub struct MexDownloader {
    start: Pagination<DateTime<Utc>, DateTimeID>,
    end: Pagination<DateTime<Utc>, DateTimeID>,
    api: MexAPI,
}

impl MexDownloader {
    fn limit(&self) -> usize {
        500
    }

    /// Creates a new downloader with a specific range.
    /// The input source is fixed.
    pub fn new(start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        Self {
            start: Pagination::new(DateTimeID::new(start), 0),
            end: Pagination::new(DateTimeID::new(end), 0),
            api: MexAPI::new(),
        }
    }
}

impl Downloader for MexDownloader {
    type IDT = Pagination<DateTime<Utc>, DateTimeID>;
    type ID = PaginatedID<DateTime<Utc>, DateTimeID>;
    type RAW = MexGetExecution;

    fn start_id(&self) -> Self::IDT {
        self.start.clone()
    }
    fn end_id(&self) -> Self::IDT {
        self.end.clone()
    }

    fn continue_condition(&self, current: &Self::IDT, end: &Self::IDT) -> bool {
        let c = current.tuple();
        let o = end.tuple();
        if c.0 == o.0 {
            c.1 <= o.1
        } else {
            c.0 <= o.0
        }
    }

    fn fetch(&self, c: &Self::IDT) -> Result<Vec<MexGetExecution>> {
        let (time, ofs) = c.tuple();
        self.api.executions(time, *ofs, self.limit())
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
    /// timestamp since the API will return just requested number fo trades for single request.
    /// In order to avoid this, the last trades in terms of the traded timestamp should be
    /// ignored at the first time, and request again from the timestamp.
    ///
    /// Mex API allows to send an offset parameter, so this will never get into an infinite loop.
    fn output(&self, u: Vec<Trade>, writer: &mut impl Writer) -> Result<Self::IDT> {
        match u.last() {
            Some(last) => {
                let last_ts = last.traded_at;
                let orig_len = u.len();
                let without_last_count = u.iter().filter(|e| e.traded_at != last_ts).count();

                if orig_len == self.limit() && without_last_count == 0 {
                    debug!("increment offset");
                    writer.write(&u).map(|num| {
                        info!("wrote {} data", num);
                        debug!("last id: {}", last_ts);
                        // this increments the offset
                        Pagination::new(DateTimeID::new(last_ts), self.limit() as u64)
                    })
                } else {
                    let without_last: Vec<Trade> =
                        u.into_iter().filter(|e| e.traded_at != last_ts).collect();

                    writer.write(without_last.as_slice()).map(|num| {
                        info!("wrote {} data", num);
                        debug!("last id: {}", last_ts);
                        Pagination::new(DateTimeID::new(last_ts), 0)
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
