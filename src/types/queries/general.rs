use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Decimal(String);

#[derive(Debug, Deserialize)]
pub struct Timestamp(i64);

#[derive(Debug, Deserialize)]
pub struct BigInt(String);
