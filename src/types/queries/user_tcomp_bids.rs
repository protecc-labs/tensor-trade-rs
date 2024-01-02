use graphql_client::GraphQLQuery;

use crate::types::queries::general::Decimal;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/user_tcomp_bids.graphql",
    response_derives = "Debug"
)]
pub struct UserTcompBids;

pub enum CollectionMintsSortBy {
    RankHrttAsc,
}

impl CollectionMintsSortBy {
    pub fn default() -> Self {
        CollectionMintsSortBy::RankHrttAsc
    }
}

pub struct CollectionMintsFilters {}