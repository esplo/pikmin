use chrono::DateTime;
use chrono::Utc;
use mysql::{params, Params};
use serde_derive::{Deserialize, Serialize};

use crate::error::Result;
use crate::writer::db_mysql::MySQLWriterElement;
use crate::writer::db_mysql::TableDef;
use crate::writer::stdout::StdOutWriterElement;

/// A writer implementation for MySQL.
pub mod db_mysql;
#[cfg(test)]
pub mod mock;
/// A writer implementation for stdout.
pub mod stdout;

/// Standard trade implementation. The data from an external source must be converted into this.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Trade {
    /// An unique string as a `primary key`. This value is useful to prevent recording
    /// the same data twice.
    pub id: String,
    /// A timestamp to represent when trade occurred.
    pub traded_at: DateTime<Utc>,
    /// A size of the trade. If this value is negative, this trade is `short` direction.
    pub quantity: f32,
    /// A traded price.
    pub price: f32,
}

impl PartialEq for Trade {
    fn eq(&self, other: &Self) -> bool {
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
            TableDef::new("id", "VARCHAR(64) NOT NULL PRIMARY KEY"),
            TableDef::new("traded_at", "TIMESTAMP(3) NOT NULL"),
            TableDef::new("amount", "FLOAT NOT NULL"),
            TableDef::new("price", "FLOAT NOT NULL"),
        ]
    }

    fn index_names() -> Vec<String> {
        vec!["traded_at".to_owned()]
    }
}

impl StdOutWriterElement for Trade {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

/// An abstraction of recorders. See StdOutWriter for an usage example.
pub trait Writer {
    /// Records `trades` on somewhere (typically DB).
    fn write(&mut self, trades: &[Trade]) -> Result<u64>;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mysql_create_table_test() {
        let exp = "CREATE TABLE IF NOT EXISTS test ( id VARCHAR(64) NOT NULL PRIMARY KEY, traded_at TIMESTAMP(3) NOT NULL, amount FLOAT NOT NULL, price FLOAT NOT NULL, KEY `ind_traded_at` (`traded_at`) );";
        assert_eq!(
            exp,
            Trade::create_table_stmt("test"),
        );
    }
}
