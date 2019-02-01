use log::trace;
use mysql::Params;
use mysql::Pool;
use mysql_common::params::Params::Positional;
use smallvec::smallvec;

use crate::error::Error;
use crate::error::Result;
use crate::writer::Trade;
use crate::writer::Writer;

/// An constraint for MySQLWriter.
pub trait MySQLWriterElement {
    /// Converts a contents into MySQL params.
    fn to_params(&self) -> Params;
    /// Returns a table definition for MySQL.
    fn table_def() -> Vec<TableDef>;
}

/// A definition for MySQL.
/// First value is a name of the column, and 2nd is a following definition in a `CREATE TABLE` statement.
///
/// # Example
///
/// ```
/// use pikmin::writer::db_mysql::TableDef;
/// fn table_def() -> Vec<TableDef> {
///     vec![
///         TableDef::new("id", "BIGINT NOT NULL PRIMARY KEY"),
///         TableDef::new("traded_at", "TIMESTAMP(3) NOT NULL"),
///         TableDef::new("amount", "FLOAT NOT NULL"),
///         TableDef::new("price", "FLOAT NOT NULL"),
///     ]
/// }
/// ```
#[derive(Debug)]
pub struct TableDef(String, String);

impl TableDef {
    /// Creates a column of a table definition.
    pub fn new(s1: &str, s2: &str) -> Self {
        TableDef(String::from(s1), String::from(s2))
    }
}

/// A writer implementation for MySQL.
#[derive(Debug)]
pub struct MySQLWriter<'a> {
    table_name: &'a str,
    connection: Pool,
}

impl<'a> MySQLWriter<'a> {
    /// Creates a writer for MySQL with a given URL and a table name.
    pub fn new(table_name: &'a str, database_url: &str) -> Self {
        trace!("connect to {}", database_url);
        let connection = Pool::new(database_url).expect("cannot connect to DB");

        MySQLWriter {
            table_name,
            connection,
        }
    }

    fn create_table<T: MySQLWriterElement>(&self) {
        // create column definition
        // e.g.
        //
        // ```
        // id BIGINT NOT NULL PRIMARY KEY,
        // traded_at TIMESTAMP(3) NOT NULL,
        // amount FLOAT NOT NULL,
        // price FLOAT NOT NULL
        // ```
        let definition = T::table_def()
            .iter()
            .map(|TableDef(k, v)| format!("{} {}", k, v))
            .collect::<Vec<String>>()
            .join(",");

        let create_stmt = format!(
            r"CREATE TABLE IF NOT EXISTS {} ( {} );",
            self.table_name, definition,
        );
        self.connection.prep_exec(create_stmt, ()).unwrap();
    }

    fn bulk_insert<T: MySQLWriterElement>(&self, v: &[T]) -> Result<u64> {
        // consider maximum number of params for MySQL
        const MAX_PARAM_NUM: usize = 10000;

        let col_names = T::table_def()
            .iter()
            .map(|TableDef(k, _)| k.clone())
            .collect::<Vec<String>>();

        v.chunks(MAX_PARAM_NUM)
            .map(|x| {
                let placeholder = format!(
                    "({})",
                    col_names
                        .iter()
                        .map(|_| "?")
                        .collect::<Vec<&str>>()
                        .join(",")
                );
                let stmt = format!(
                    r#"REPLACE INTO {} ({}) VALUES {} ;"#,
                    self.table_name,
                    col_names.join(","),
                    vec![placeholder; x.len()].join(",")
                );

                let params: Result<Params> = x
                    .iter()
                    .map(|e| e.to_params())
                    .map(|p: Params| {
                        p.into_positional(&col_names)
                            .map_err(|e| Error::MySqlMissingNamedParameter(Box::new(e)))
                    })
                    .collect::<Result<Vec<Params>>>()
                    .map(|vp| {
                        let u = vp
                            .into_iter()
                            .flat_map(|v| match v {
                                Positional(ps) => ps,
                                _ => smallvec![],
                            })
                            .collect();
                        Positional(u)
                    });

                params.and_then(|p: Params| {
                    self.connection
                        .prep_exec(stmt, p)
                        .map(|result| result.affected_rows())
                        .map_err(Error::from)
                })
            })
            .collect::<Result<Vec<u64>>>()
            .and_then(|e| Ok(e.iter().sum()))
    }
}

impl<'a> Writer for MySQLWriter<'a> {
    fn write(&mut self, trades: &[Trade]) -> Result<u64> {
        self.create_table::<Trade>();
        self.bulk_insert(trades)
    }
}
