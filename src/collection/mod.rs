use anyhow::Result;
use graphql_client::{GraphQLQuery, Response};

use super::{constants, TensorTradeClient};

mod queries;

pub(crate) use queries::{
    collection_mints as collection_mints_query, collection_stats as collection_stats_query,
    mint_list as mint_list_query, mints as mints_query, CollectionMints as CollectionMintsQuery,
    CollectionStats as CollectionStatsQuery, MintList as MintListQuery, Mints as MintsQuery,
};

pub struct Collection<'a>(pub(crate) &'a TensorTradeClient);

impl<'a> Collection<'a> {
    /// Get stats and metadata for a single NFT collection.
    pub async fn get_stats(
        &self,
        slug: String,
    ) -> Result<Option<collection_stats_query::CollectionStatsInstrumentTv2>, anyhow::Error> {
        let query = CollectionStatsQuery::build_query(collection_stats_query::Variables { slug });

        let response = self
            .0
            .client
            .post(constants::TENSOR_TRADE_API_URL)
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

    pub async fn get_token_mint_list(
        &self,
        slug: String,
        mut limit: Option<i64>,
        after: Option<String>,
    ) -> Result<Vec<String>, anyhow::Error> {
        if limit.is_none() {
            limit = Some(100)
        };

        let query = MintListQuery::build_query(mint_list_query::Variables {
            slug: slug.clone(),
            limit,
            after,
        });

        let response = self
            .0
            .client
            .post(constants::TENSOR_TRADE_API_URL)
            .json(&query)
            .send()
            .await?;
        // .map(|response| response.error_for_status())??;

        let response_body: Response<mint_list_query::ResponseData> = response.json().await?;

        if let Some(data) = response_body.data {
            let mint_list = data.mint_list;

            if !mint_list.is_empty() {
                Ok(mint_list)
            } else {
                // Err(TensorTradeError::NoMintList);
                eprintln!("no mint list");
                todo!()
            }
        } else {
            // Err(TensorTradeError::NoResponseData);
            eprintln!("no response data");
            todo!()
        }
    }

    /// Get token mints for a single NFT collection.
    /// If limit is not provided, it defaults to 100
    pub async fn get_token_mints(
        &self,
        slug: String,
        sort_by: collection_mints_query::CollectionMintsSortBy,
        filters: Option<collection_mints_query::CollectionMintsFilters>,
        cursor: Option<String>,
        mut limit: Option<i64>,
    ) -> Result<collection_mints_query::CollectionMintsCollectionMintsV2, anyhow::Error> {
        if limit.is_none() {
            limit = Some(100)
        };

        let query = CollectionMintsQuery::build_query(collection_mints_query::Variables {
            slug,
            sort_by,
            filters,
            cursor,
            limit,
        });

        let response = self
            .0
            .client
            .post(constants::TENSOR_TRADE_API_URL)
            .json(&query)
            .send()
            .await?;
        // .map(|response| response.error_for_status())??;

        let response_body: Response<collection_mints_query::ResponseData> = response.json().await?; // This error should be because of deserialization, not because of the HTTP request.

        dbg!(&response_body);
        if let Some(data) = response_body.data {
            dbg!(&data);
            Ok(data.collection_mints_v2)
        } else {
            // Err(TensorTradeError::NoResponseData);
            eprintln!("no response data");
            todo!()
        }
    }

    // Get the slug for a collection with any token mint in it.
    pub async fn get_slug(&self, token_mint: String) -> anyhow::Result<String> {
        let query = MintsQuery::build_query(mints_query::Variables {
            token_mints: vec![token_mint],
        });

        let response = self
            .0
            .client
            .post(constants::TENSOR_TRADE_API_URL)
            .json(&query)
            .send()
            .await?;
        // .map(|response| response.error_for_status())??;

        let response_body: Response<mints_query::ResponseData> = response.json().await?;

        if let Some(data) = response_body.data {
            let mints = data.mints;

            if let Some(mint) = mints.into_iter().next() {
                Ok(mint.slug)
            } else {
                // Err(TensorTradeError::NoMints);
                eprintln!("no mints");
                todo!()
            }
        } else {
            // Err(TensorTradeError::NoResponseData);
            eprintln!("no response data");
            todo!()
        }
    }

    pub async fn is_compressed(&self, slug: String) -> anyhow::Result<bool> {
        let stats = self.get_stats(slug).await?;

        if let Some(stats) = stats {
            Ok(stats.compressed)
        } else {
            Err(anyhow::anyhow!("no stats"))
        }
    }
}
