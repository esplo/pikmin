use log::trace;
use reqwest::Client;
use reqwest::RequestBuilder;
use reqwest::Response;

use crate::error::Error;
use crate::error::Result;

pub mod bf;
pub mod liquid;
pub mod mex;

pub trait ExchangeAPIClient {
    fn base_url(&self) -> &'static str;
    fn reqwest_client(&self) -> &Client;

    fn url_builder(&self, path: &str) -> String {
        format!("{}{}", self.base_url(), path)
    }

    fn with_common_header(&self, r: RequestBuilder) -> RequestBuilder {
        r.header("Content-Type", "application/json")
    }

    fn make_get_request(&self, path: &str) -> RequestBuilder {
        trace!("make GET request: {}", path);
        self.with_common_header(self.reqwest_client().get(&self.url_builder(path)))
    }

    fn resp_to_json<T>(&self, resp: Response) -> reqwest::Result<T>
        where
                for<'de> T: serde::Deserialize<'de>,
    {
        let mut r = resp;
        r.json()
    }

    fn send<T>(&self, resp: RequestBuilder) -> Result<T>
        where
                for<'de> T: serde::Deserialize<'de>,
    {
        resp.send()
            .and_then(|req| req.error_for_status())
            .and_then(|resp| self.resp_to_json(resp))
            .map_err(Error::from)
    }
}
