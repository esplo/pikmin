use std::fmt::Display;

use crate::error::Result;

pub mod datetime;
pub mod num;

pub trait DownloaderID<T: Display> {
    fn current(&self) -> &T;
    fn update(&mut self, c: T) -> Result<()>;
    fn to_string(&self) -> String;
}
