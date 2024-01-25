use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Decimal(pub String);

impl Decimal {
    pub fn new(decimal: String) -> Self {
        Self(decimal)
    }
}

impl From<String> for Decimal {
    fn from(item: String) -> Self {
        Self::new(item)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Timestamp(pub i64);

impl Timestamp {
    pub fn new(timestamp: i64) -> Self {
        Self(timestamp)
    }
}

impl From<i64> for Timestamp {
    fn from(item: i64) -> Self {
        Self::new(item)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BigInt(pub String);

impl BigInt {
    pub fn new(big_int: String) -> Self {
        Self(big_int)
    }
}

impl From<String> for BigInt {
    fn from(item: String) -> Self {
        Self::new(item)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Byte {
    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,
}

impl Byte {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }
}

impl From<Vec<u8>> for Byte {
    fn from(item: Vec<u8>) -> Self {
        Self::new(item)
    }
}
