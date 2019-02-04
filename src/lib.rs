//! An extensible downloader for obtaining trade data simultaneously from exchanges' API.
//!
//! `pikmin` is a trade (execution) data downloader for crypto-currency exchanges,
//! such as BitMex, bitFlyer, Liquid, etc. This library provides not only some pre-composed
//! downloaders, but also ability to build a custom downloader for users' demand.
//!
//! A downloader is composed of ID and Writer. An ID specifies the point where to start downloading,
//! and a Writer outputs trade data to external storage.
//!
//! There are some pre-composed downloaders in `downloader` module.
//!
//! # Example
//!
//! A simple downloader for Liquid with writing to stdout.
//! This program creates `/tmp/qn-progress.txt` for recording progress,
//! so delete it if you want to run again from the starting point.
//!
//! ```
//! use std::path::Path;
//! use std::thread;
//! use std::time::Duration;
//!
//! use chrono::offset::TimeZone;
//! use chrono::Utc;
//!
//! use pikmin::{LiquidDownloader, StdOutWriter};
//! use pikmin::downloader::Downloader;
//!
//! fn main() {
//!     // by using thread, you can run multiple downloaders
//!     let liquid = thread::spawn(move || {
//!         while {
//!             // download data from 2019-01-01T01:01:01Z to 2019-01-01T01:03:01Z
//!             // output the downloaded data to standard out (println!)
//!             let downloader = LiquidDownloader::new(
//!                 Utc.ymd(2019, 1, 1).and_hms(1, 1, 1),
//!                 Utc.ymd(2019, 1, 1).and_hms(1, 3, 1),
//!             );
//!
//!             // Locate progress file to `/tmp/qn-progress.txt`.
//!             // You can control the starting point of downloading
//!             // by preparing this file before you run.
//!             let progress_file = Path::new("/tmp/qn-progress.txt");
//!
//!             // write out to standard out. simple writer for debugging
//!             let mut writer = StdOutWriter::default();
//!
//!             println!("start QnDownloader");
//!
//!             // run!
//!             match downloader.run(&mut writer, &progress_file) {
//!                 Ok(_) => {
//!                     println!("finished");
//!                     false
//!                 }
//!                 Err(e) => {
//!                     println!("error: {}", e);
//!                     println!("retry...");
//!                     thread::sleep(Duration::from_secs(5));
//!                     true
//!                 }
//!             }
//!         } {}
//!     });
//!
//!     match liquid.join() {
//!         Ok(_) => println!("finish all"),
//!         Err(_) => println!("threading error"),
//!     }
//! }
//! ```

pub use self::downloader::bf::BfDownloader;
pub use self::downloader::bitmex::MexDownloader;
pub use self::downloader::liquid::LiquidDownloader;
pub use self::writer::db_mysql::MySQLWriter;
pub use self::writer::stdout::StdOutWriter;

/// Clients for exchange APIs.
mod api;
/// Composed downloaders.
pub mod downloader;
/// Error utilities.
pub mod error;
/// Utilities for creating a downloader
mod util;
/// Writers to output the trade data into external storage.
pub mod writer;

#[cfg(test)]
mod tests {
    use std::io::prelude::*;

    use chrono::offset::TimeZone;
    use chrono::Utc;
    use tempfile::NamedTempFile;

    use crate::downloader::Downloader;
    use crate::downloader::mock::RawData;
    use crate::error::Result;
    use crate::writer::Trade;

    use super::*;

    #[test]
    fn downloader_test() {
        let data = vec![
            RawData { id: 4 },
            RawData { id: 8 },
            RawData { id: 10 },
            RawData { id: 11 },
            RawData { id: 15 },
            RawData { id: 17 },
            RawData { id: 19 },
        ];

        let downloader = downloader::mock::MockDownloader::new(data, 10, 16);

        let mut writer = writer::mock::MockWriter::new();

        let actual_fn = || -> Result<_> {
            let mut tmpf = NamedTempFile::new()?;
            let stored: Vec<Trade> = downloader
                .run(&mut writer, &tmpf.path())
                .map(|_| writer.store)?;
            let mut progress = String::new();
            tmpf.read_to_string(&mut progress)?;
            Ok((stored, progress))
        };

        let actual = actual_fn();
        assert_eq!(actual.is_ok(), true);
        let act = actual.unwrap();
        assert_eq!(
            act.0,
            vec![
                Trade {
                    id: String::from("10"),
                    traded_at: Utc.ymd(1970, 1, 1).and_hms(0, 0, 10),
                    quantity: 1.0,
                    price: 0.1,
                },
                Trade {
                    id: String::from("11"),
                    traded_at: Utc.ymd(1970, 1, 1).and_hms(0, 0, 11),
                    quantity: 1.1,
                    price: 0.11,
                },
                Trade {
                    id: String::from("15"),
                    traded_at: Utc.ymd(1970, 1, 1).and_hms(0, 0, 15),
                    quantity: 1.5,
                    price: 0.15,
                },
                // Notice: the downloading will overrun
                Trade {
                    id: String::from("17"),
                    traded_at: Utc.ymd(1970, 1, 1).and_hms(0, 0, 17),
                    quantity: 1.7,
                    price: 0.17,
                },
            ],
        );
        // 17 + 1
        assert_eq!(act.1, r#"{"current":18}"#);
    }
}
