use anyhow::Result;
use reqwest::header;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer};

pub(crate) mod collection;
pub(crate) mod execute;
pub(crate) mod tensorswap;
pub(crate) mod types;
pub(crate) mod user;
pub(crate) mod utils;

mod constants;

pub use crate::{
    collection::collection_mints_query::{CollectionMintsFilters, CollectionMintsSortBy},
    user::user_active_listings_query::{ActiveListingsCursorInputV2, ActiveListingsSortBy},
};

impl Default for CollectionMintsSortBy {
    fn default() -> Self {
        Self::RankHrttAsc
    }
}

#[derive(Debug, Clone)]
pub struct TensorTradeClient {
    pub(crate) client: reqwest::Client,
    pub(crate) private_key: String,
    pub(crate) rpc_url: String,
}

impl TensorTradeClient {
    pub fn new(
        api_key: String,
        private_key: String,
        rpc_url: Option<String>,
    ) -> anyhow::Result<Self> {
        if api_key.is_empty() {
            return Err(anyhow::anyhow!("api key cannot be empty"));
        };
        if private_key.is_empty() {
            return Err(anyhow::anyhow!("private key cannot be empty"));
        };

        let rpc_url =
            rpc_url.unwrap_or_else(|| String::from("https://api.mainnet-beta.solana.com/"));
        if rpc_url.is_empty() {
            return Err(anyhow::anyhow!("rpc_url cannot be empty"));
        }

        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );
        headers.insert("X-TENSOR-API-KEY", header::HeaderValue::from_str(&api_key)?);

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self {
            client,
            private_key,
            rpc_url,
        })
    }

    pub fn collection(&self) -> collection::Collection {
        collection::Collection(self)
    }

    pub fn user(&self) -> user::User {
        user::User(self)
    }

    pub fn tensorswap(&self) -> tensorswap::Tensorswap {
        tensorswap::Tensorswap(self)
    }

    pub fn execute(&self) -> execute::Execute {
        execute::Execute(self)
    }

    pub fn utils(&self) -> utils::Utils {
        utils::Utils(self)
    }
}

pub trait Settings {
    fn update_private_key(&mut self, private_key: String);

    fn update_rpc_url(&mut self, rpc_url: String);
}

impl Settings for TensorTradeClient {
    fn update_private_key(&mut self, private_key: String) {
        self.private_key = private_key;
    }

    fn update_rpc_url(&mut self, rpc_url: String) {
        self.rpc_url = rpc_url;
    }
}

pub trait Getters {
    fn this_account(&self) -> Result<Pubkey>;

    fn this_balance(&self) -> Result<u64>;
}
impl Getters for TensorTradeClient {
    fn this_account(&self) -> Result<Pubkey> {
        Ok(Keypair::from_base58_string(&self.private_key).pubkey())
    }

    fn this_balance(&self) -> Result<u64> {
        Ok(RpcClient::new(&self.rpc_url)
            .get_account(&self.this_account()?)?
            .lamports)
    }
}
