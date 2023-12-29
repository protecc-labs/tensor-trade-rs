use anyhow::Result;

extern crate tensor_trade_rs;

use tensor_trade_rs::TensorTrade;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let api_key = std::env::var("TENSOR_TRADE_API_KEY")?;

    let client = tensor_trade_rs::TensorTradeClient::new(api_key);

    // client
    //     .get_collection_stats("rfijruifrnufnre".to_string())
    //     .await;
    // client.get_collection_stats("tensorians".to_string()).await;

    client
        .get_active_orders("duckpunkzuniverse".to_string())
        .await?;

    Ok(())
}
