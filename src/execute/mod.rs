use anyhow::Ok;
use solana_sdk::transaction::VersionedTransaction;

use crate::collection::active_listings_query;

use super::TensorTradeClient;

pub struct Execute<'a>(pub(crate) &'a TensorTradeClient);

impl<'a> Execute<'a> {
    pub async fn buy_listing(
        &self,
        buyer: String,
        token_mint: String,
    ) -> Result<bool, anyhow::Error> {
        let slug = self.0.collection().get_slug(token_mint.clone()).await?;

        let active_listings = self
            .0
            .collection()
            .get_active_listings(
                slug,
                active_listings_query::ActiveListingsSortBy::PriceAsc,
                None,
                None,
                None,
            )
            .await?;

        // Find the active listing equal to the token mint.
        let listing = active_listings
            .iter()
            .find(|listing| listing.mint.onchain_id == token_mint);

        if let None = listing {
            return Err(anyhow::anyhow!("no listing"));
        }

        let listing = listing.unwrap();

        dbg!(&listing);

        let listing = listing.clone().tx;

        // let buyer = self.0.utils().get_this_account().await?;

        let (_, tx_v0) = self
            .0
            .tensorswap()
            .get_buy_listing_tx(
                buyer,
                listing.gross_amount.unwrap().0,
                token_mint,
                listing.seller_id.unwrap(),
            )
            .await?
            .unwrap();
        let tx_v0 = tx_v0.data;

        let transaction: VersionedTransaction = bincode::deserialize(&tx_v0).unwrap();
        let signed_transaction = self.0.utils().sign_transaction(transaction.message).await?;

        let signature = self
            .0
            .utils()
            .execute_transaction(signed_transaction)
            .await?;
        dbg!(signature);

        Ok(true)
    }

    pub async fn buy_order(
        &self,
        buyer: String,
        token_mint: String,
    ) -> Result<bool, anyhow::Error> {
        let slug = self.0.collection().get_slug(token_mint.clone()).await?;

        let active_orders = self.0.tensorswap().get_active_orders(slug).await?;

        let order = active_orders.iter().find(|order| {
            order
                .nfts_for_sale
                .iter()
                .find(|nft_for_sale| nft_for_sale.onchain_id == token_mint)
                .is_some()
        });

        let order = match order {
            Some(order) => order,
            None => return Err(anyhow::anyhow!("no order")),
        };
        dbg!(&order);

        // Buy price is in lamports (1e9 SOL).
        let buy_price = order.buy_now_price.clone().unwrap().0;
        let buy_price = buy_price.parse::<i64>().unwrap() as f64;
        dbg!(&buy_price);
        let mm_fee_bps = order.mm_fee_bps.unwrap() as f64;
        dbg!(&mm_fee_bps);
        let fee_rate_decimal = mm_fee_bps / 10_000.00;
        dbg!(&fee_rate_decimal);
        let gross_amount_lamports = buy_price / (1.0 - fee_rate_decimal);
        dbg!(&gross_amount_lamports);

        let (_, tx_v0) = self
            .0
            .tensorswap()
            .get_buy_order_tx(buyer, gross_amount_lamports.to_string(), token_mint)
            .await?
            .unwrap();
        let tx_v0 = tx_v0.data;

        let transaction: VersionedTransaction = bincode::deserialize(&tx_v0).unwrap();
        let signed_transaction = self.0.utils().sign_transaction(transaction.message).await?;

        let signature = self
            .0
            .utils()
            .execute_transaction(signed_transaction)
            .await?;
        dbg!(signature);

        Ok(true)
    }

    // Sell to a pool immediately.
    // It executes a delist transaction before it executes a sell now transaction.
    pub async fn sell_now(
        &self,
        seller: String,
        token_mint: String,
    ) -> Result<bool, anyhow::Error> {
        self.delist(&token_mint, &seller).await?;

        let (_, tx_v0) = self
            .0
            .tensorswap()
            .get_sell_now_tx(seller, token_mint)
            .await?
            .unwrap();
        let tx_v0 = tx_v0.data;

        let transaction: VersionedTransaction = bincode::deserialize(&tx_v0).unwrap();
        let signed_transaction = self.0.utils().sign_transaction(transaction.message).await?;

        let signature = self
            .0
            .utils()
            .execute_transaction(signed_transaction)
            .await?;
        dbg!(signature);

        Ok(true)
    }

    // IMPORTANT: Price is in Solana lamports (1e9 SOL).
    pub async fn list(
        &self,
        token_mint: &str,
        owner: &str,
        price: &str,
    ) -> Result<bool, anyhow::Error> {
        let (_, tx_v0) = self
            .0
            .tensorswap()
            .get_list_nft_tx(token_mint, owner, price)
            .await?
            .unwrap();
        let tx_v0 = tx_v0.data;

        let transaction: VersionedTransaction = bincode::deserialize(&tx_v0).unwrap();
        let signed_transaction = self.0.utils().sign_transaction(transaction.message).await?;

        let signature = self
            .0
            .utils()
            .execute_transaction(signed_transaction)
            .await?;
        dbg!(signature);

        Ok(true)
    }

    // IMPORTANT: Price is in Solana lamports (1e9 SOL).
    pub async fn edit_listing(
        &self,
        token_mint: &str,
        owner: &str,
        price: &str,
    ) -> Result<bool, anyhow::Error> {
        let (_, tx_v0) = self
            .0
            .tensorswap()
            .get_edit_listing_tx(token_mint, owner, price)
            .await?
            .unwrap();
        let tx_v0 = tx_v0.data;

        let transaction: VersionedTransaction = bincode::deserialize(&tx_v0).unwrap();
        let signed_transaction = self.0.utils().sign_transaction(transaction.message).await?;

        let signature = self
            .0
            .utils()
            .execute_transaction(signed_transaction)
            .await?;
        dbg!(signature);

        Ok(true)
    }

    pub async fn delist(&self, token_mint: &str, owner: &str) -> Result<bool, anyhow::Error> {
        let (_, tx_v0) = self
            .0
            .tensorswap()
            .get_delist_nft_tx(token_mint, owner)
            .await?
            .unwrap();
        let tx_v0 = tx_v0.data;

        let transaction: VersionedTransaction = bincode::deserialize(&tx_v0).unwrap();
        let signed_transaction = self.0.utils().sign_transaction(transaction.message).await?;

        let signature = self
            .0
            .utils()
            .execute_transaction(signed_transaction)
            .await?;
        dbg!(signature);

        Ok(true)
    }
}
