use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::Keypair;

pub async fn sign_transaction() {
    let keypair = Keypair::from_base58_string(
        "reduce balance lady suggest cup crumble diesel impact unfair stuff zoo fox",
    );

    dbg!(keypair);
}
