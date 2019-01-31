use chrono::DateTime;
use chrono::Utc;
use mysql::{params, Params};
use serde_derive::{Deserialize, Serialize};

use crate::error::Result;
use crate::writer::db_mysql::MySQLWriterElement;
use crate::writer::db_mysql::TableDef;
use crate::writer::stdout::StdOutWriterElement;

pub mod db_mysql;
#[cfg(test)]
pub mod mock;
pub mod stdout;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Trade {
    pub id: String,
    pub traded_at: DateTime<Utc>,
    pub quantity: f32,
    pub price: f32,
}

impl PartialEq for Trade {
    fn eq(&self, other: &Trade) -> bool {
        self.id == other.id
            && self.traded_at == other.traded_at
            && (self.quantity - other.quantity).abs() <= std::f32::EPSILON
            && (self.price - other.price).abs() <= std::f32::EPSILON
    }
}

impl Eq for Trade {}

impl MySQLWriterElement for Trade {
    fn to_params(&self) -> Params {
        let p = self.clone();
        (params! {
            "id" => p.id,
            "traded_at" => p.traded_at.naive_utc(),
            "amount" => p.quantity,
            "price" => p.price,
        })
            .into()
    }

    fn table_def() -> Vec<TableDef> {
        vec![
            TableDef::new("id", "BIGINT NOT NULL PRIMARY KEY"),
            TableDef::new("traded_at", "TIMESTAMP(3) NOT NULL"),
            TableDef::new("amount", "FLOAT NOT NULL"),
            TableDef::new("price", "FLOAT NOT NULL"),
        ]
    }
}

impl StdOutWriterElement for Trade {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

pub trait Writer {
    fn write(&mut self, trades: &[Trade]) -> Result<u64>;
}
