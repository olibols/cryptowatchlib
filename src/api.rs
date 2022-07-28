use reqwest::{blocking::{Client, RequestBuilder}, StatusCode};
use thiserror::Error;
use crate::api::models::api_results::CWPriceResult;

mod models;

const CRYPTOWATCH_API_URL: &str = "https://api.cryptowat.ch/";
type CwResult<T> = Result<T, CwErrors>;

//impl From<reqwest::Error> for CwResult<f32>{

//}

#[derive(Error, Debug)]
pub enum CwErrors {
    #[error("Reqwest Error")]
    RequestError(#[from] reqwest::Error),
    #[error("Generic Error")]
    GenericError,
}

#[derive(Clone, Debug)]
struct Config {
    currency: String,
}

impl Default for Config {
    fn default() -> Self {
        Self{
            currency: "USD".into(), 
        }
    }
}

pub struct WatchBuilder {
    client: Client,
    config: Config
}

impl WatchBuilder {
    pub fn new() -> Self {
        let client = Client::builder().pool_idle_timeout(None).build().unwrap();

        Self {
            client,
            config: Config::default()
        }
    }

    pub fn build(self) -> CryptoWatch {
        CryptoWatch {client: self.client, config: self.config}
    }
}

pub struct CryptoWatch {
    client: Client,
    config: Config
}

impl CryptoWatch {
    pub fn new() -> Self {
        WatchBuilder::new().build()
    }

    fn add_endpoint(&self, endpoint: &str) -> RequestBuilder {
        self.client
        .get(format!("{}{}", CRYPTOWATCH_API_URL, endpoint))
        .header("Accepts", "Json")
    }

    pub fn get_price(&self, exchange: &str, pair: &str) -> CwResult<f32> {
        let rb = self
        .add_endpoint(format!("markets/{}/{}/price", exchange, pair).as_str())
        .send().unwrap();
        
        match rb.status() {
            StatusCode::OK => {
            let json: CWPriceResult = rb.json::<CWPriceResult>()?;

            Ok(json.result.price)
            }
            code => {
            Err(CwErrors::GenericError)
            }
        }

    }
}