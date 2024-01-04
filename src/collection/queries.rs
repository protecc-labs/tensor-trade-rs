use graphql_client::GraphQLQuery;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Decimal(pub String);

#[derive(Debug, Deserialize)]
pub struct Timestamp(pub i64);

#[derive(Debug, Deserialize)]
pub struct BigInt(pub String);

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/collection/collection_stats.graphql",
    response_derives = "Debug"
)]
pub struct CollectionStats;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/collection/collection_mints.graphql",
    response_derives = "Debug"
)]
pub struct CollectionMints;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/collection/mints.graphql",
    response_derives = "Debug"
)]
pub struct Mints;
