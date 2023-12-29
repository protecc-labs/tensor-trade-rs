use graphql_client::GraphQLQuery;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Decimal(String);

#[derive(Debug, Deserialize)]
struct Timestamp(i64);

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/collection_stats.graphql",
    response_derives = "Debug"
)]
pub struct CollectionStats;
