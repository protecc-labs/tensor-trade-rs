use anyhow::Ok;
use graphql_client::{GraphQLQuery, Response};
use reqwest::{header, Client};

mod constants;
mod types;

use constants::TENSOR_TRADE_API_URL;
use types::{
    queries::{
        collection_stats::{
            collection_stats::{self as collection_stats_query, CollectionStatsInstrumentTv2},
            CollectionStats as CollectionStatsQuery,
        },
        tensorswap_active_orders::{
            tensor_swap_active_orders::{
                self as tensorswap_active_orders_query, TensorSwapActiveOrdersTswapOrders,
            },
            TensorSwapActiveOrders as TensorSwapActiveOrdersQuery,
        },
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
pub trait TensorTrade {
    async fn get_collection_stats(
        &self,
        slug: String,
    ) -> Result<Option<CollectionStatsInstrumentTv2>, anyhow::Error>;

    // async fn get_active_listings(
    //     &self,
    //     slug: String,
    //     sort_by: SortBy,
    //     filters: Option<Filters>,
    //     limit: Option<Limit>,
    //     cursor: Option<Cursor>,
    // );

    async fn get_active_orders(
        &self,
        slug: String,
    ) -> Result<Vec<TensorSwapActiveOrdersTswapOrders>, anyhow::Error>;
}

#[async_trait::async_trait]
impl TensorTrade for TensorTradeClient {
    /// Single Collection Stats & Metadata
    async fn get_collection_stats(
        &self,
        slug: String,
    ) -> Result<Option<CollectionStatsInstrumentTv2>, anyhow::Error> {
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
                todo!()
            }
        } else {
            // Err(TensorTradeError::NoResponseData);
            eprintln!("no response data");
            todo!()
        }
    }

    // async fn get_active_listings(
    //     &self,
    //     slug: String,
    //     sort_by: SortBy,
    //     filters: Option<Filters>,
    //     limit: Option<Limit>,
    //     cursor: Option<Cursor>,
    // ) {
    // }

    /// TensorSwap Active Orders
    async fn get_active_orders(
        &self,
        slug: String,
    ) -> Result<Vec<TensorSwapActiveOrdersTswapOrders>, anyhow::Error> {
        let query =
            TensorSwapActiveOrdersQuery::build_query(tensorswap_active_orders_query::Variables {
                slug: slug.clone(),
            });

        let response = self
            .client
            .post(TENSOR_TRADE_API_URL)
            .json(&query)
            .send()
            .await?;
        // .map(|response| response.error_for_status())??;

        let response_body: Response<tensorswap_active_orders_query::ResponseData> =
            response.json().await?; // This error should be because of deserialization, not because of the HTTP request.

        if let Some(data) = response_body.data {
            Ok(data.tswap_orders)
            // TODO: Warn user if `data.tswap_orders` is empty.
        } else {
            // Err(TensorTradeError::NoResponseData);
            eprintln!("no response data");
            todo!()
        }
    }
}
