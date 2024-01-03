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

    // dbg!(
    //     client
    //         .get_collection_stats("05c52d84-2e49-4ed9-a473-b43cab41e777".to_string())
    //         .await
    // );

    // dbg!(
    //     client
    //         .is_compressed_collection("froganas".to_string())
    //         .await?
    // );

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

    // dbg!(
    //     client
    //         .get_tensorswap_buy_single_nft_from_listing(
    //             "CHrpFgkN89fcAMV8BcKpGS1RueJc4ZyoLy9xxdTtiQaA".to_string(),
    //             Decimal("500000000000".to_string()),
    //             "8zkfLBNFwo1SN13tDA6XE5VDXFDpG8jZLNo4pyCexFhP".to_string(),
    //             "9gVndQ5SdugdFfGzyuKmePLRJZkCreKZ2iUTEg4agR5g".to_string(),
    //         )
    //         .await?
    // );

    // dbg!(
    //     client
    //         .get_tensorswap_buy_nft(
    //             "CHrpFgkN89fcAMV8BcKpGS1RueJc4ZyoLy9xxdTtiQaA".to_string(),
    //             Decimal("3000000000".to_string()),
    //             "8zkfLBNFwo1SN13tDA6XE5VDXFDpG8jZLNo4pyCexFhP".to_string(),
    //         )
    //         .await?
    // );

    dbg!(
        client
            .get_tensorswap_buy_nft(
                "CHrpFgkN89fcAMV8BcKpGS1RueJc4ZyoLy9xxdTtiQaA".to_string(),
                Decimal("2900000000".to_string()),
                "6372Z4BSZBuVChZJWeHpJz68WoXqiSagA4egmQrubUT9".to_string(),
            )
            .await?
    );

    Ok(())
}
