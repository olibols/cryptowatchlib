use serde::{Deserialize, Serialize};
use serde_json::Value;


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CWCoinPrice {
    pub price: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CWAllowance {
    pub cost: f32,
    pub remaining: f32,
    pub upgrade: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CWPriceResult {
    pub result: CWCoinPrice,
    allowance: CWAllowance,
}