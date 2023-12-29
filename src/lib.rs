use graphql_client::{GraphQLQuery, Response};
use reqwest::{header, Client};

mod constants;
mod types;

use constants::TENSOR_TRADE_API_URL;
use types::{
    queries::collection_stats::{
        collection_stats::{self as collection_stats_query, CollectionStatsInstrumentTv2},
        CollectionStats as CollectionStatsQuery,
    },
    responses::collection_stats::CollectionStats,
};

#[derive(Debug, Clone)]
pub struct TensorTradeClient {
    client: Client,
}

impl TensorTradeClient {
    pub fn new(api_key: String) -> Self {
        let client = Client::builder()
            .default_headers({
                let mut headers = header::HeaderMap::new();
                headers.insert(
                    "X-TENSOR-API-KEY",
                    header::HeaderValue::from_str(&api_key).unwrap(),
                );
                headers
            })
            .build()
            .unwrap();

        Self { client }
    }
}

#[async_trait::async_trait]
pub trait TensorTradeGetters {
    async fn get_collection_stats(
        &self,
        slug: String,
    ) -> Result<Option<CollectionStatsInstrumentTv2>, reqwest::Error>;
}

#[async_trait::async_trait]
impl TensorTradeGetters for TensorTradeClient {
    async fn get_collection_stats(
        &self,
        slug: String,
    ) -> Result<Option<CollectionStatsInstrumentTv2>, reqwest::Error> {
        let query = CollectionStatsQuery::build_query(collection_stats_query::Variables {
            slug: slug.clone(),
        });

        let response = self
            .client
            .post(TENSOR_TRADE_API_URL)
            .json(&query)
            .send()
            .await?;
        // .map(|response| response.error_for_status())??;

        let response_body: Response<collection_stats_query::ResponseData> = response.json().await?; // This error should be because of deserialization, not because of the HTTP request.

        if let Some(data) = response_body.data {
            if let Some(instrument_tv2) = data.instrument_tv2 {
                Ok(Some(instrument_tv2))
            } else {
                // Err(TensorTradeError::NoInstrumentTV2);
                eprintln!("no collection stats");
                Ok(None)
            }
        } else {
            // Err(TensorTradeError::NoResponseData);
            eprintln!("no response data");
            Ok(None)
        }
    }
}
