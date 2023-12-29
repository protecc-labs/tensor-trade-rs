use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Decimal(String);

#[derive(Debug, Deserialize)]
pub struct Timestamp(i64);

#[derive(Debug, Deserialize)]
pub struct BigInt(String);
