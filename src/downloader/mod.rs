use std::fmt::Display;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::thread;
use std::time::Duration;

use log::{info, trace, warn};

use crate::downloader::id::DownloaderID;
use crate::error::Error;
use crate::error::Result;
use crate::util::reader::from_existing_file;
use crate::writer::Trade;
use crate::writer::Writer;

pub mod bf;
pub mod bitmex;
pub mod liquid;
pub mod id;

#[cfg(test)]
pub(crate) mod mock;

pub trait Downloader {
    type IDT: std::str::FromStr + Display;
    type ID: DownloaderID<Self::IDT> + From<Self::IDT>;
    type RAW;

    fn init_id(&self, default: Self::IDT, process_log_path: &Path) -> Result<Self::IDT> {
        match from_existing_file(process_log_path) {
            Ok(ref s) if s.is_empty() => {
                warn!("no content in {}", process_log_path.display());
                Ok(default)
            }
            Ok(s) => {
                let id_value = s
                    .parse::<Self::IDT>()
                    .map_err(|_| Error::ParseInitialValueError)?;
                trace!("initial value was successfully read from a file");
                Ok(id_value)
            }
            Err(e) => {
                warn!("cannot read from a file: {}", e);
                Ok(default)
            }
        }
    }

    fn start_id(&self) -> Self::IDT;
    fn end_id(&self) -> Self::IDT;

    fn continue_condition(&self, current: &Self::IDT, end: &Self::IDT) -> bool;

    fn fetch(&self, c: &Self::IDT) -> Result<Vec<Self::RAW>>;
    fn convert(&self, v: &Self::RAW) -> Result<Trade>;
    fn output(&self, u: Vec<Trade>, writer: &mut impl Writer) -> Result<Self::IDT>;

    fn record_progress(&self, path: &Path, id: &str) -> Result<()> {
        // TODO: progress writer??
        trace!("record ID: {}", id);
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;
        file.write_all(id.as_bytes())?;
        Ok(())
    }

    fn sleep_millis(&self) -> u64;

    fn run(&self, writer: &mut impl Writer, process_log_path: &Path) -> Result<()> {
        let init_id_value = self.init_id(self.start_id(), process_log_path)?;
        info!("start from {}", init_id_value);
        let mut init_id = Self::ID::from(init_id_value);
        let end_id_value = self.end_id();
        info!("run to {}", end_id_value);

        while self.continue_condition(&init_id.current(), &end_id_value) {
            self.fetch(init_id.current())
                .and_then(|v| v.iter().map(|t| self.convert(&t)).collect())
                .and_then(|v| self.output(v, writer))
                .and_then(|next_id| {
                    init_id.update(next_id).and_then(|_| {
                        self.record_progress(process_log_path, &init_id.to_string())
                            .map_err(Error::from)
                    })
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
