use graphql_client::GraphQLQuery;

use crate::types::queries_scalar::{BigInt, Decimal, Timestamp};

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

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/collection/mint.graphql",
    response_derives = "Debug, Clone"
)]
pub struct Mint;
