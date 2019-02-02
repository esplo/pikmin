use log::trace;
use reqwest::Client;
use reqwest::RequestBuilder;
use serde_derive::{Deserialize, Serialize};

use crate::api::ExchangeAPIClient;
use crate::error::Error;
use crate::error::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct LiquidGetExecution {
    pub id: u64,
    pub quantity: String,
    pub price: String,
    pub taker_side: String,
    pub created_at: u64,
}

#[derive(Debug)]
pub struct LiquidAPI {
    reqwest_client: Client,
    product_code: &'static str,
}

impl Default for LiquidAPI {
    fn default() -> Self {
        Self::new()
    }
}

impl LiquidAPI {
    pub fn new() -> Self {
        LiquidAPI {
            reqwest_client: Client::new(),
            product_code: "5",
        }
    }

    pub fn executions(&self, timestamp: u64, limit: usize) -> Result<Vec<LiquidGetExecution>> {
        trace!("executions -- timestamp:{}, limit:{}", timestamp, limit);
        let path = format!(
            "/executions?product_id={}&timestamp={}&limit={}",
            self.product_code, timestamp, limit
        );
        let req = self.make_get_request(&path);
        self.send(req).map_err(Error::from)
    }
}

impl ExchangeAPIClient for LiquidAPI {
    fn base_url(&self) -> &'static str {
        "https://api.liquid.com"
    }

    fn reqwest_client(&self) -> &Client {
        &self.reqwest_client
    }

    fn with_common_header(&self, r: RequestBuilder) -> RequestBuilder {
        r.header("X-Quoine-API-Version", "2")
            .header("Content-Type", "application/json")
    }
}
