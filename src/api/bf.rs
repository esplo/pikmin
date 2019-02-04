use log::trace;
use reqwest::Client;
use serde_derive::{Deserialize, Serialize};

use crate::api::ExchangeAPIClient;
use crate::error::Error;
use crate::error::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct BfGetExecution {
    pub id: u64,
    pub side: String,
    pub price: f32,
    pub size: f32,
    pub exec_date: String,
    pub buy_child_order_acceptance_id: String,
    pub sell_child_order_acceptance_id: String,
}

#[derive(Debug)]
pub struct BfAPI {
    reqwest_client: Client,
    product_code: &'static str,
}

impl Default for BfAPI {
    fn default() -> Self {
        Self::new()
    }
}

impl BfAPI {
    pub fn new() -> Self {
        Self {
            reqwest_client: Client::new(),
            product_code: "FX_BTC_JPY",
        }
    }

    pub fn executions(&self, before_id: u64, limit: usize) -> Result<Vec<BfGetExecution>> {
        trace!("executions -- before_id:{}, limit:{}", before_id, limit);
        let path = format!(
            "/v1/executions?product_code={}&before={}&count={}",
            self.product_code, before_id, limit
        );
        let req = self.make_get_request(&path);
        self.send(req).map_err(Error::from)
    }
}

impl ExchangeAPIClient for BfAPI {
    fn base_url(&self) -> &'static str {
        "https://api.bitflyer.com"
    }

    fn reqwest_client(&self) -> &Client {
        &self.reqwest_client
    }
}
