use chrono::DateTime;
use chrono::Utc;
use log::trace;
use reqwest::Client;
use serde_derive::{Deserialize, Serialize};

use crate::api::ExchangeAPIClient;
use crate::error::Error;
use crate::error::Result;

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct MexGetExecution {
    pub timestamp: DateTime<Utc>,
    symbol: String,
    pub side: String,
    pub size: u64,
    pub price: f32,
    tickDirection: String,
    pub trdMatchID: String,
    grossValue: u64,
    homeNotional: f32,
    foreignNotional: u64,
}

#[derive(Debug)]
pub struct MexAPI {
    reqwest_client: Client,
    product_code: &'static str,
}

impl Default for MexAPI {
    fn default() -> Self {
        Self::new()
    }
}

impl MexAPI {
    pub fn new() -> Self {
        MexAPI {
            reqwest_client: Client::new(),
            product_code: "XBTUSD",
        }
    }

    pub fn executions(
        &self,
        start_time: &DateTime<Utc>,
        limit: usize,
    ) -> Result<Vec<MexGetExecution>> {
        trace!("executions -- start_time:{}, limit:{}", start_time, limit);
        let path = format!(
            "/api/v1/trade?symbol={}&startTime={}&count={}&reverse=false",
            self.product_code, start_time, limit
        );
        let req = self.make_get_request(&path);
        self.send(req).map_err(Error::from)
    }
}

impl ExchangeAPIClient for MexAPI {
    fn base_url(&self) -> &'static str {
        "https://www.bitmex.com"
    }

    fn reqwest_client(&self) -> &Client {
        &self.reqwest_client
    }
}
