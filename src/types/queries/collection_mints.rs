use graphql_client::GraphQLQuery;
use serde::de;

use crate::types::queries::general::Decimal;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/collection_mints.graphql",
    response_derives = "Debug"
)]
pub struct CollectionMints;

pub enum CollectionMintsSortBy {
    RankHrttAsc,
}

impl CollectionMintsSortBy {
    pub fn default() -> Self {
        CollectionMintsSortBy::RankHrttAsc
    }
}

pub struct CollectionMintsFilters {}
