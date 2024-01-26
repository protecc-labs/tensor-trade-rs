use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    message::VersionedMessage,
    signature::{Keypair, Signature},
    transaction::VersionedTransaction,
};

// NOTE: Not working.
// pub async fn to_valid_account(account: &str) -> Result<Pubkey> {
//     // Convert to bytes.
//     let bytes = hex::decode(account)?;
//     // Verify that the bytes length is 32.
//     if bytes.len() != 32 {
//         return Err(anyhow::anyhow!("invalid account"));
//     }
//     // Convert to [u8; 32].
//     let bytes = bytes.as_slice();
//     let bytes = <[u8; 32]>::try_from(bytes)?;

//     Ok(Pubkey::from(bytes))
// }

pub async fn sign_transaction(
    rpc_url: &str,
    private_key: &str,
    mut transaction_message: VersionedMessage,
) -> Result<VersionedTransaction> {
    let rpc_client = RpcClient::new(rpc_url);
    let signer = Keypair::from_base58_string(private_key);

    transaction_message.set_recent_blockhash(rpc_client.get_latest_blockhash()?);

    let signed_transaction = VersionedTransaction::try_new(transaction_message, &[&signer])?;

    Ok(signed_transaction)
}

pub async fn execute_transaction(
    rpc_url: &str,
    signed_transaction: VersionedTransaction,
) -> Result<Signature> {
    let rpc_client = RpcClient::new(rpc_url);

    rpc_client.simulate_transaction(&signed_transaction)?;

    let signature = rpc_client.send_and_confirm_transaction_with_spinner(&signed_transaction)?;

    Ok(signature)
}

pub async fn sing_and_execute_transaction(
    rpc_url: &str,
    private_key: &str,
    transaction_message: VersionedMessage,
) -> Result<Signature> {
    let signed_transaction = sign_transaction(private_key, rpc_url, transaction_message).await?;
    let signature = execute_transaction(rpc_url, signed_transaction).await?;

    Ok(signature)
}
