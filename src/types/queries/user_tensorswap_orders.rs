use graphql_client::GraphQLQuery;

use crate::types::queries::general::{BigInt, Decimal, Timestamp};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/user_tensorswap_orders.graphql",
    response_derives = "Debug"
)]
pub struct UserTensorSwapOrders;

pub enum CollectionMintsSortBy {
    RankHrttAsc,
}

impl CollectionMintsSortBy {
    pub fn default() -> Self {
        CollectionMintsSortBy::RankHrttAsc
    }
}

pub struct CollectionMintsFilters {}
