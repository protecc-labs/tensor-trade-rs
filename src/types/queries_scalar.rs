use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Decimal(pub String);

impl Decimal {
    pub fn new(decimal: String) -> Self {
        Self(decimal)
    }

    // pub fn value(&self) -> &String {
    //     &self.0
    // }
}

impl From<String> for Decimal {
    fn from(item: String) -> Self {
        Decimal::new(item)
    }
}

// impl Into<String> for Decimal {
//     fn into(self) -> String {
//         self.0
//     }
// }

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Timestamp(pub i64);

impl Timestamp {
    pub fn new(timestamp: i64) -> Self {
        Self(timestamp)
    }

    // pub fn value(&self) -> i64 {
    //     self.0
    // }
}

impl From<i64> for Timestamp {
    fn from(item: i64) -> Self {
        Timestamp::new(item)
    }
}

// impl Into<i64> for Timestamp {
//     fn into(self) -> i64 {
//         self.0
//     }
// }

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BigInt(pub String);

impl BigInt {
    pub fn new(big_int: String) -> Self {
        Self(big_int)
    }
}

impl From<String> for BigInt {
    fn from(item: String) -> Self {
        BigInt::new(item)
    }
}

// impl Into<String> for BigInt {
//     fn into(self) -> String {
//         self.0
//     }
// }

#[derive(Debug, Clone, Deserialize)]
pub struct Byte {
    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,
}

impl Byte {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    // pub fn value(&self) -> &Vec<u8> {
    //     &self.data
    // }
}

impl From<Vec<u8>> for Byte {
    fn from(item: Vec<u8>) -> Self {
        Byte::new(item)
    }
}

// impl Into<Vec<u8>> for Byte {
//     fn into(self) -> Vec<u8> {
//         self.data
//     }
// }
