use graphql_client::GraphQLQuery;

use crate::types::queries::general::{Decimal, Timestamp};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/user_active_listings.graphql",
    response_derives = "Debug"
)]
pub struct UserActiveListingsV2;

pub enum CollectionMintsSortBy {
    RankHrttAsc,
}

impl CollectionMintsSortBy {
    pub fn default() -> Self {
        CollectionMintsSortBy::RankHrttAsc
    }
}

pub struct CollectionMintsFilters {}
