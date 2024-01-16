use anyhow::Ok;
use solana_sdk::transaction::VersionedTransaction;

use super::TensorTradeClient;

pub struct Execute<'a>(pub(crate) &'a TensorTradeClient);

impl<'a> Execute<'a> {
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
