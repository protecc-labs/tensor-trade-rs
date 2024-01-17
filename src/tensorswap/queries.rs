use graphql_client::GraphQLQuery;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Decimal(pub String);

#[derive(Debug, Clone, Deserialize)]
pub struct Timestamp(pub i64);

#[derive(Debug, Clone, Deserialize)]
pub struct BigInt(pub String);

#[derive(Debug, Clone, Deserialize)]
pub struct Byte {
    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/tensorswap/tensorswap_active_orders.graphql",
    response_derives = "Debug, Clone"
)]
pub struct TensorSwapActiveOrders;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/tensorswap/tswap_buy_nft_tx.graphql",
    response_derives = "Debug, Clone"
)]
pub struct TswapBuyNftTx;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/tensorswap/tswap_buy_single_listing_tx.graphql",
    response_derives = "Debug, Clone"
)]
pub struct TswapBuySingleListingTx;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/tensorswap/tswap_list_nft_tx.graphql",
    response_derives = "Debug, Clone"
)]
pub struct TswapListNftTx;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/tensorswap/tswap_edit_single_listing_tx.graphql",
    response_derives = "Debug, Clone"
)]
pub struct TswapEditSingleListingTx;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/tensorswap/tswap_delist_nft_tx.graphql",
    response_derives = "Debug, Clone"
)]
pub struct TswapDelistNftTx;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/tensorswap/take_bid_tx.graphql",
    response_derives = "Debug, Clone"
)]
pub struct TakeBidTx;
