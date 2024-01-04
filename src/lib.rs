use reqwest::header;

pub(crate) mod collection;
pub(crate) mod tensorswap;
pub(crate) mod user;

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
}

impl TensorTradeClient {
    pub fn new(api_key: &str) -> anyhow::Result<Self> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );
        headers.insert("X-TENSOR-API-KEY", header::HeaderValue::from_str(&api_key)?);

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self { client })
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
}
