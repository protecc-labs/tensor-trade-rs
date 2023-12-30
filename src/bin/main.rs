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

    // client
    //     .get_active_orders("duckpunkzuniverse".to_string())
    //     .await?;

    // dbg!(
    //     client
    //         .get_tcomp_bids("duckpunkzuniverse".to_string())
    //         .await?
    // );

    // dbg!(
    //     client
    //         .get_token_mints_slugs(vec![
    //             "8h5qaBJxgaDYkt8GKbDtDGUDNbQCWyW5SFCCEnLK76Fe".to_string()
    //         ])
    //         .await?
    // );

    dbg!(
        client
            .get_mint_list("degods".to_string(), None, None)
            .await?
    );

    Ok(())
}
