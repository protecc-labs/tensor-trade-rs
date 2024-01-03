use anyhow::Result;

extern crate tensor_trade_rs;

use tensor_trade_rs::{types::queries::general::Decimal, TensorTrade};

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

    // dbg!(
    //     client
    //         .get_mint_list("degods".to_string(), None, None)
    //         .await?
    // );

    // dbg!(
    //     client
    //         .get_collection_mints(
    //             "degods".to_string(),
    //             tensor_trade_rs::types::queries::collection_mints::collection_mints::CollectionMintsSortBy::RankHrttAsc,
    //             None,
    //             None,
    //             Some(100)
    //         )
    //         .await?
    // );

    // dbg!(
    //     client
    //         .get_users_active_listings(
    //             vec!["CHrpFgkN89fcAMV8BcKpGS1RueJc4ZyoLy9xxdTtiQaA".to_string()],
    //             tensor_trade_rs::types::queries::user_active_listings::user_active_listings_v2::ActiveListingsSortBy::PriceAsc,
    //             None,
    //             Some(1),
    //             None,
    //         )
    //         .await?
    // );

    // dbg!(
    //     client
    //         .get_user_tensorswap_active_orders(
    //             "CHrpFgkN89fcAMV8BcKpGS1RueJc4ZyoLy9xxdTtiQaA".to_string(),
    //         )
    //         .await?
    // );

    // dbg!(
    //     client
    //         .get_user_tcomp_active_bids("CHrpFgkN89fcAMV8BcKpGS1RueJc4ZyoLy9xxdTtiQaA".to_string(),)
    //         .await?
    // );

    // sign_transaction::sign_transaction().await;

    dbg!(
        client
            .get_tensorswap_buy_single_nft_from_listing(
                "8aNhUxFBxKcM4HxBVeson2cPFFvEZeKWGLL2nyfQTtvj".to_string(),
                Decimal("420000000000".to_string()),
                "44kyDF8LrjaovuxmQcsNAnwTqmpDQa3NipkbYu1VgkKK".to_string(),
                "3e2A1hrt33rBfzRupkeECYSPUPL44A3wLXtF6dSoEhnn".to_string(),
            )
            .await?
    );

    Ok(())
}
