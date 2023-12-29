use graphql_client::GraphQLQuery;

use crate::types::queries::general::{Decimal, Timestamp};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/collection_stats.graphql",
    response_derives = "Debug"
)]
pub struct CollectionStats;
