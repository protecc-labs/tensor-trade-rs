use serde::{Deserialize, Serialize};
use std::num::ParseFloatError;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Decimal(String);

impl Decimal {
    pub fn new(decimal: String) -> Self {
        Self(decimal)
    }
}

impl From<Decimal> for String {
    fn from(decimal: Decimal) -> Self {
        decimal.0
    }
}

impl From<Decimal> for Result<f64, ParseFloatError> {
    fn from(decimal: Decimal) -> Self {
        decimal.0.parse::<f64>()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Timestamp(i64);

impl From<Timestamp> for i64 {
    fn from(timestamp: Timestamp) -> Self {
        timestamp.0
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BigInt(String);

impl From<BigInt> for String {
    fn from(big_int: BigInt) -> Self {
        big_int.0
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Byte {
    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,
}
