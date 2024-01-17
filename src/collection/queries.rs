use graphql_client::GraphQLQuery;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Decimal(pub String);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Timestamp(pub i64);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BigInt(pub String);

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/collection/collection_stats.graphql",
    response_derives = "Debug, Clone"
)]
pub struct CollectionStats;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/collection/collection_mints.graphql",
    response_derives = "Debug, Clone"
)]
pub struct CollectionMints;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/collection/mints.graphql",
    response_derives = "Debug, Clone"
)]
pub struct Mints;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/collection/mint_list.graphql",
    response_derives = "Debug, Clone"
)]
pub struct MintList;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/collection/active_listings.graphql",
    response_derives = "Debug, Clone"
)]
pub struct ActiveListingsV2;
