use graphql_client::GraphQLQuery;

use crate::types::queries::general::{Byte, Decimal};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/transactions/tswap_buy_single_listing_tx.graphql",
    response_derives = "Debug"
)]
pub struct TswapBuySingleListingTx;
