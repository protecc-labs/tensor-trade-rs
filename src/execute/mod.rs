use solana_sdk::transaction::VersionedTransaction;

use super::TensorTradeClient;

pub struct Execute<'a>(pub(crate) &'a TensorTradeClient);

impl<'a> Execute<'a> {
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
