use std::path::Path;
use std::thread;
use std::time::Duration;

use chrono::offset::TimeZone;
use chrono::Utc;

use pikmin::{MySQLWriter, StdOutWriter};
use pikmin::{BfDownloader, LiquidDownloader, MexDownloader};
use pikmin::downloader::Downloader;

// You can easily create a custom writer (this will be used in the `mex` example)
struct CustomWriter {}

impl pikmin::writer::Writer for CustomWriter {
    fn write(&mut self, trades: &[pikmin::writer::Trade]) -> pikmin::error::Result<u64> {
        for t in trades.iter() {
            println!("{}", t.id);
        }
        Ok(trades.len() as u64)
    }
}

fn main() {
    // by using thread, you can run multiple downloaders
    let liquid = thread::spawn(move || {
        while {
            // download data from 2019-01-01T01:01:01Z to 2019-01-01T01:03:01Z
            // output the downloaded data to standard out (println!)
            let downloader = LiquidDownloader::new(
                Utc.ymd(2019, 1, 1).and_hms(1, 1, 1),
                Utc.ymd(2019, 1, 1).and_hms(1, 3, 1),
            );

            // Locate progress file to `/tmp/qn-progress.txt`.
            // You can control the starting point of downloading
            // by preparing this file before you run.
            let progress_file = Path::new("/tmp/qn-progress.txt");

            // write out to standard out. simple writer for debugging
            let mut writer = StdOutWriter::default();

            println!("start QnDownloader");

            // run!
            match downloader.run(&mut writer, &progress_file) {
                Ok(_) => {
                    println!("finished");
                    false
                }
                Err(e) => {
                    println!("error: {}", e);
                    println!("retry...");
                    thread::sleep(Duration::from_secs(5));
                    true
                }
            }
        } {}
    });

    let bf = thread::spawn(move || {
        while {
            // Sadly, BitFlyer does not provide a pagination by a timestamp.
            // Use u64::MAX and 0 to fetch all the data.
            // Notice: execution data older than 1 month are not allowed to download on BitFlyer
            let downloader = BfDownloader::new(764637994, 764677430);

            let progress_file = Path::new("/tmp/bf-progress.txt");

            // Make MySQL connection
            let mut mysql_writer =
                MySQLWriter::new("bf", "mysql://root:hoge@localhost:3333/trades");

            println!("start BfDownloader");

            match downloader.run(&mut mysql_writer, &progress_file) {
                Ok(_) => {
                    println!("finished");
                    false
                }
                Err(e) => {
                    println!("error: {}", e);
                    println!("retry...");
                    thread::sleep(Duration::from_secs(5));
                    true
                }
            }
        } {}
    });

    let mex = thread::spawn(move || {
        while {
            let downloader = MexDownloader::new(
                Utc.ymd(2019, 1, 1).and_hms(1, 1, 1),
                Utc.ymd(2019, 1, 1).and_hms(1, 3, 1),
            );

            let progress_file = Path::new("/tmp/mex-progress.txt");

            // custom writer
            let mut writer = CustomWriter {};

            println!("start MexDownloader");

            match downloader.run(&mut writer, &progress_file) {
                Ok(_) => {
                    println!("finished");
                    false
                }
                Err(e) => {
                    println!("error: {}", e);
                    println!("retry...");
                    thread::sleep(Duration::from_secs(5));
                    true
                }
            }
        } {}
    });

    match liquid
        .join()
        .and_then(|_| bf.join())
        .and_then(|_| mex.join())
        {
            Ok(_) => println!("finish all"),
            Err(_) => println!("threading error"),
        }
}
