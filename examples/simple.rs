use std::path::Path;
use std::thread;
use std::time::Duration;

use chrono::offset::TimeZone;
use chrono::Utc;

use pikmin::{LiquidDownloader, StdOutWriter};
use pikmin::downloader::Downloader;

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

    match liquid.join() {
        Ok(_) => println!("finish all"),
        Err(_) => println!("threading error"),
    }
}
