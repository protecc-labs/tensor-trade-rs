use anyhow::Result;

use tensor_trade::{Getters, TensorTradeClient};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let api_key =
        std::env::var("TENSOR_TRADE_API_KEY").expect("tensor trade api key variable must be set");
    let private_key = std::env::var("PRIVATE_KEY").expect("private key variable must be set");

    let tensor_trade_client = TensorTradeClient::new(api_key, private_key, None)?;

    let account_balance = tensor_trade_client.this_account().unwrap();

    println!("Account balance: {}", account_balance);

    Ok(())
}
