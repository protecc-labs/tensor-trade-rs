use graphql_client::GraphQLQuery;

use crate::types::queries_scalar::{BigInt, Decimal, Timestamp};

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
