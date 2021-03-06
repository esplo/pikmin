use chrono::DateTime;
use chrono::prelude::*;
use chrono::Utc;
use log::{error, info, warn};

use crate::api::liquid::LiquidAPI;
use crate::api::liquid::LiquidGetExecution;
use crate::downloader::Downloader;
use crate::downloader::id::datetime::DateTimeID;
use crate::error::Error;
use crate::error::Error::InvalidSide;
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
        let f_quantity = v.quantity.parse::<f32>()?;
        let quantity = match v.taker_side.as_ref() {
            "buy" => Ok(f_quantity),
            "sell" => Ok(-1.0 * f_quantity),
            s => Err(InvalidSide(s.to_owned())),
        }?;
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
        if let Some(last) = u.last() {
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
        } else {
            warn!("no output");
            Err(Error::NotFound)
        }
    }

    fn sleep_millis(&self) -> u64 {
        1100
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn liquid_converter_test() {
        let dummy = Utc.ymd(2000, 1, 1).and_hms(0, 0, 0);
        let ld = LiquidDownloader::new(dummy, dummy);

        {
            let arg = LiquidGetExecution {
                id: 5,
                quantity: "0.12".to_string(),
                price: "123.4".to_string(),
                taker_side: "buy".to_string(),
                created_at: 1323318775,
            };
            let exp = Trade {
                id: "5".to_string(),
                traded_at: Utc.ymd(2011, 12, 8).and_hms(4, 32, 55),
                quantity: 0.12,
                price: 123.4,
            };
            let res = ld.convert(&arg);
            assert_eq!(true, res.is_ok());
            assert_eq!(exp, res.unwrap());
        }

        {
            let arg = LiquidGetExecution {
                id: 5,
                quantity: "0.12".to_string(),
                price: "123.4".to_string(),
                taker_side: "sell".to_string(),
                created_at: 1323318775,
            };
            let exp = Trade {
                id: "5".to_string(),
                traded_at: Utc.ymd(2011, 12, 8).and_hms(4, 32, 55),
                quantity: -0.12,
                price: 123.4,
            };
            let res = ld.convert(&arg);
            assert_eq!(true, res.is_ok());
            assert_eq!(exp, res.unwrap());
        }
    }
}
