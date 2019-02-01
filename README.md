# Pikmin

An extensible downloader for obtaining trade data simultaneously from exchanges' API.

[![Build Status](https://travis-ci.org/esplo/pikmin.svg?branch=master)](https://travis-ci.org/esplo/pikmin)
[![crates.io](https://img.shields.io/crates/v/pikmin.svg?style=flat)](https://crates.io/crates/pikmin)
[![Documentation](https://docs.rs/pikmin/badge.svg)](
https://docs.rs/pikmin)
[![codecov](https://codecov.io/gh/esplo/pikmin/branch/master/graph/badge.svg)](https://codecov.io/gh/esplo/pikmin)

`pikmin` is a trade (execution) data downloader for crypto-currency exchanges,
such as BitMex, bitFlyer, Liquid, etc. This library provides not only some pre-composed
downloaders, but also ability to build a custom downloader for users' demand.

## Pre-composed downloaders

Currently, this library has the following downloaders:

* BitMex (specify time, chronologically)
* bitFlyer (specify id, reverse-chronologically)
* Liquid (specify time, chronologically)

## Built-in Writer

`Writer` is a processor between trade data and destination (typically DB).
Pikmin has some built-in writers:

* MySQL
* stdout

You can create your own writer easily.

## Example

A simple downloader for Liquid with writing to stdout.
This program creates `./qn-progress.txt` for recording progress,
so delete it if you want to run again from the starting point.

```rust
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
            let progress_file = Path::new("./qn-progress.txt");

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
```

Other examples can be found in `./examples`.

## Future work

- create pre-composed downloaders for other exchanges
- parameterize the direction of downloading (chronologically or not)
- abstraction of the progress writer
