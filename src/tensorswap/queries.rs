use graphql_client::GraphQLQuery;

use crate::types::queries_scalar::{BigInt, Byte, Decimal, Timestamp};

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
    query_path = "graphql/tensorswap/tswap_sell_nft_tx.graphql",
    response_derives = "Debug, Clone"
)]
pub struct TswapSellNftTx;

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
