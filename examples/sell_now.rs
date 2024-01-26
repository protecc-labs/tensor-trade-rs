use anyhow::Result;

use tensor_trade::TensorTradeClient;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let api_key =
        std::env::var("TENSOR_TRADE_API_KEY").expect("tensor trade api key variable must be set");
    let private_key = std::env::var("PRIVATE_KEY").expect("private key variable must be set");

    let tensor_trade_client = TensorTradeClient::new(api_key, private_key, None)?;

    let signature = tensor_trade_client
        .execute()
        .sell_now("64nfraAs39sJJXmmdt4tUGQRbkxNC6Aw58wCSwFcysYy")
        .await?;

    println!("Signature: {}", signature);

    Ok(())
}
