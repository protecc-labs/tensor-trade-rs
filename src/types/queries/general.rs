use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Decimal(pub String);

#[derive(Debug, Deserialize)]
pub struct Timestamp(pub i64);

#[derive(Debug, Deserialize)]
pub struct BigInt(pub String);

#[derive(Debug, Deserialize)]
pub struct Byte(pub String);
