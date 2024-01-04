use graphql_client::GraphQLQuery;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Decimal(pub String);

#[derive(Debug, Deserialize)]
pub struct Timestamp(pub i64);

#[derive(Debug, Deserialize)]
pub struct BigInt(pub String);

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/user/user_active_listings.graphql",
    response_derives = "Debug"
)]
pub struct UserActiveListingsV2;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/user/user_tcomp_bids.graphql",
    response_derives = "Debug"
)]
pub struct UserTcompBids;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/user/user_tensorswap_orders.graphql",
    response_derives = "Debug"
)]
pub struct UserTensorSwapOrders;
