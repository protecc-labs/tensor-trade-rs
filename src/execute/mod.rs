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
