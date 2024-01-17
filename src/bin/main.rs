use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let api_key =
        std::env::var("TENSOR_TRADE_API_KEY").expect("tensor trade api key variable must be set");
    let private_key = std::env::var("PRIVATE_KEY").expect("private key variable must be set");

    let client = tensor_trade::TensorTradeClient::new(api_key, private_key, None)?;

    let user_wallet = "CHrpFgkN89fcAMV8BcKpGS1RueJc4ZyoLy9xxdTtiQaA";

    // dbg!(
    //     client
    //         .user()
    //         .get_active_listings(wallets, sort_by, cursor, limit, slug)
    //         .await
    // );: &str

    // dbg!(
    //     client
    //         .collection()
    //         .get_token_mint_list(
    //             "05c52d84-2e49-4ed9-a473-b43cab41e777".to_string(),
    //             None,
    //             None
    //         )
    //         .await?
    // );

    // dbg!(
    //     client
    //         .tensorswap()
    //         .get_active_orders("duckpunkzuniverse".to_string())
    //         .await?
    // );

    // dbg!(
    //     client
    //         .user()
    //         .get_active_listings(
    //             vec!["9gVndQ5SdugdFfGzyuKmePLRJZkCreKZ2iUTEg4agR5g".to_string()],
    //             tensor_trade_rs::ActiveListingsSortBy::PriceAsc,
    //             None,
    //             Some(1),
    //             None,
    //         )
    //         .await?
    // );

    // dbg!(
    //     client
    //         .collection()
    //         .get_stats("tensorians".to_string())
    //         .await?
    // );

    // dbg!(
    //     client
    //         .collection()
    //         .get_stats("ijfurfnerufnef".to_string())
    //         .await?
    // );

    // dbg!(
    //     client
    //         .collection()
    //         .get_token_mints(
    //             "degods".to_string(),
    //             tensor_trade_rs::CollectionMintsSortBy::default(),
    //             None,
    //             None,
    //             None
    //         )
    //         .await?
    // );

    // dbg!(
    //     client
    //         .collection()
    //         .is_compressed("tensorians".to_string())
    //         .await?
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
    //         .user()
    //         .get_active_listings(
    //             "CHrpFgkN89fcAMV8BcKpGS1RueJc4ZyoLy9xxdTtiQaA".to_string(),
    //             tensor_trade::ActiveListingsSortBy::PriceAsc,
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
    //         .tensorswap()
    //         .get_tensorswap_buy_nft(
    //             "CHrpFgkN89fcAMV8BcKpGS1RueJc4ZyoLy9xxdTtiQaA".to_string(),
    //             "3000000000".to_string(),
    //             "8zkfLBNFwo1SN13tDA6XE5VDXFDpG8jZLNo4pyCexFhP".to_string(),
    //         )
    //         .await?
    // );

    // dbg!(
    //     client
    //         .tensorswap()
    //         .get_buy_order_tx(
    //             "CHrpFgkN89fcAMV8BcKpGS1RueJc4ZyoLy9xxdTtiQaA".to_string(),
    //             "2900000000".to_string(),
    //             "6372Z4BSZBuVChZJWeHpJz68WoXqiSagA4egmQrubUT9".to_string(),
    //         )
    //         .await?
    // );

    // dbg!(
    //     client
    //         .tensorswap()
    //         .get_buy_listing_tx(
    //             "CHrpFgkN89fcAMV8BcKpGS1RueJc4ZyoLy9xxdTtiQaA".to_string(),
    //             "2900000000".to_string(),
    //             "6372Z4BSZBuVChZJWeHpJz68WoXqiSagA4egmQrubUT9".to_string(),
    //         )
    //         .await?
    // );

    // dbg!(
    //     client
    //         .tensorswap()
    //         .get_list_nft_tx(
    //             "32aexhaNYWdaN8vpuUmk4YR1EN6jfQnWV2myekYYNyEN",
    //             "CHrpFgkN89fcAMV8BcKpGS1RueJc4ZyoLy9xxdTtiQaA",
    //             "13000000000"
    //         )
    //         .await?
    // );

    // dbg!(
    //     client
    //         .tensorswap()
    //         .get_edit_listing_tx(
    //             "32aexhaNYWdaN8vpuUmk4YR1EN6jfQnWV2myekYYNyEN",
    //             "CHrpFgkN89fcAMV8BcKpGS1RueJc4ZyoLy9xxdTtiQaA",
    //             "13000000000"
    //         )
    //         .await?
    // );

    dbg!(
        client
            .execute()
            .buy_listing(
                user_wallet.to_string(),
                "89q5wsiN34xbvYR8qzfTzctb6vE8U3BGrT8yyPWmKGx6".to_string()
            )
            .await?
    );

    Ok(())
}
