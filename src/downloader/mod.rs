use std::fmt::Display;
use std::thread;
use std::time::Duration;

use log::{info, trace, warn};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::downloader::id::DownloaderID;
use crate::error::Error;
use crate::error::Result;
use crate::recorder::ProgressRecorder;
use crate::writer::Trade;
use crate::writer::Writer;

/// A downloader for bitFlyer.
pub mod bf;
/// A downloader for BitMEX.
pub mod bitmex;
/// A downloader for Liquid.
pub mod liquid;
/// ID implementations for composing a Downloader.
pub mod id;

#[cfg(test)]
pub(crate) mod mock;

/// An abstraction of downloaders. You can make a new downloader by implementing this trait.
pub trait Downloader {
    /// A type of element which ID is made up of.
    type IDT: DeserializeOwned + Display;
    /// A type of ID for specifying the downloading point in an API client.
    type ID: DownloaderID<Self::IDT> + From<Self::IDT> + DeserializeOwned + Serialize;
    /// A type of downloaded trade data.
    type RAW;

    /// Returns initial ID from a progress file. If reading is failed, use a given `default` value.
    fn init_id(&self, default: Self::IDT, recorder: &mut impl ProgressRecorder) -> Result<Self::ID> {
        match recorder.read() {
            Ok(ref s) if s.is_empty() => {
                warn!("no content, start with the default value: {}", default);
                Ok(Self::ID::from(default))
            }
            Ok(s) => {
                let id = serde_json::from_str(&s).map_err(Error::ParseJson)?;
                trace!("initial value was successfully read from a file");
                Ok(id)
            }
            Err(e) => {
                warn!("cannot read from a file: {}", e);
                Ok(Self::ID::from(default))
            }
        }
    }

    /// Returns beginning ID for a downloading process.
    fn start_id(&self) -> Self::IDT;
    /// Returns ending ID for a downloading process.
    fn end_id(&self) -> Self::IDT;

    /// Judges whether to continue downloading or not.
    fn continue_condition(&self, current: &Self::IDT, end: &Self::IDT) -> bool;

    /// Fetches trade data from somewhere (typically API).
    fn fetch(&self, c: &Self::IDT) -> Result<Vec<Self::RAW>>;
    /// Converts a raw trade datum into a standard trade struct.
    fn convert(&self, v: &Self::RAW) -> Result<Trade>;
    /// Writes trade data to somewhere with a given writer.
    /// This must return an id for the next iteration, and this will be recorded on a progress file.
    fn output(&self, u: Vec<Trade>, writer: &mut impl Writer) -> Result<Self::IDT>;

    /// Returns milli seconds to sleep between fetching processes.
    fn sleep_millis(&self) -> u64;

    /// Executes downloading.
    fn run(&self, writer: &mut impl Writer, recorder: &mut impl ProgressRecorder) -> Result<()> {
        let mut init_id = self.init_id(self.start_id(), recorder)?;
        info!("start from {}", init_id.current());
        let end_id_value = self.end_id();
        info!("run to {}", end_id_value);

        while self.continue_condition(&init_id.current(), &end_id_value) {
            self.fetch(init_id.current())
                .and_then(|v| v.iter().map(|t| self.convert(&t)).collect())
                .and_then(|v| self.output(v, writer))
                .and_then(|next_id| {
                    init_id
                        .update(next_id)
                        .and_then(|_| serde_json::to_string(&init_id).map_err(Error::from))
                        .and_then(|json| recorder.out(&json))
                })
                .map(|_| {
                    let millis = self.sleep_millis();
                    trace!("sleep {} ms", millis);
                    thread::sleep(Duration::from_millis(millis));
                })?
        }

        Ok(())
    }
}
