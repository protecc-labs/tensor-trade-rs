use std::thread::AccessError;

use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    account::Account,
    message::VersionedMessage,
    signature::{Keypair, Signature, Signer},
    transaction::{self, VersionedTransaction},
};

use super::TensorTradeClient;

pub struct Utils<'a>(pub(crate) &'a TensorTradeClient);

impl<'a> Utils<'a> {
    pub async fn sign_transaction(
        &self,
        mut transaction_message: VersionedMessage,
    ) -> Result<VersionedTransaction, anyhow::Error> {
        let rpc_client = RpcClient::new(&self.0.rpc_url);
        let signer = Keypair::from_base58_string(&self.0.private_key);

        transaction_message.set_recent_blockhash(rpc_client.get_latest_blockhash()?);

        let signed_transaction = VersionedTransaction::try_new(transaction_message, &[&signer])?;

        Ok(signed_transaction)
    }

    pub async fn execute_transaction(
        &self,
        signed_transaction: VersionedTransaction,
    ) -> Result<Signature, anyhow::Error> {
        let rpc_client = RpcClient::new(&self.0.rpc_url);

        rpc_client.simulate_transaction(&signed_transaction)?;

        let sig = rpc_client.send_and_confirm_transaction_with_spinner(&signed_transaction)?;

        Ok(sig)
    }

    // pub async fn get_this_account(&self) -> Result<Account, anyhow::Error> {
    //     let keypair = Keypair::from_base58_string(&self.0.private_key);

    //     let pubkey = keypair.pubkey();

    //     let account = RpcClient::new(&self.0.rpc_url).get(&pubkey)?;

    //     dbg!(&account.);

    //     Ok(account)
    // }
}
