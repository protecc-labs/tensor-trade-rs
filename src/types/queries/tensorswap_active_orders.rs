use graphql_client::GraphQLQuery;

use crate::types::queries::general::{BigInt, Decimal, Timestamp};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/tensorswap_active_orders.graphql",
    response_derives = "Debug"
)]
pub struct TensorSwapActiveOrders;
