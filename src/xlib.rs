use anyhow::Ok;
use graphql_client::{GraphQLQuery, Response};
use reqwest::{header, Client};

pub mod constants;
pub mod helpers;
pub mod types;

use constants::TENSOR_TRADE_API_URL;
use types::{
    queries::{
        collection_mints::{
            collection_mints::{self as collection_mints_query, CollectionMintsCollectionMintsV2},
            CollectionMints as CollectionMintsQuery,
        },
        collection_stats::{
            collection_stats::{self as collection_stats_query, CollectionStatsInstrumentTv2},
            CollectionStats as CollectionStatsQuery,
        },
        general::Decimal,
        mint_list::{
            mint_list::{self as mint_list_query},
            MintList as MintListQuery,
        },
        mints::{
            mints::{self as mints_query, MintsMints},
            Mints as MintsQuery,
        },
        tcomp_bids::{
            tcomp_bids::{self as tcomp_bids_query, TcompBidsTcompBids},
            TcompBids as TcompBidsQuery,
        },
        tensorswap_active_orders::{
            tensor_swap_active_orders::{
                self as tensorswap_active_orders_query, TensorSwapActiveOrdersTswapOrders,
            },
            TensorSwapActiveOrders as TensorSwapActiveOrdersQuery,
        },
        transactions::{
            tswap_buy_nft_tx::{
                tswap_buy_nft_tx::{self as tswap_buy_nft_tx_query, TswapBuyNftTxTswapBuyNftTx},
                TswapBuyNftTx as TswapBuyNftTxQuery,
            },
            tswap_buy_single_listing_tx::{
                tswap_buy_single_listing_tx::{
                    self as tswap_buy_single_listing_tx_query,
                    TswapBuySingleListingTxTswapBuySingleListingTx,
                },
                TswapBuySingleListingTx as TswapBuySingleListingTxQuery,
            },
        },
        user_active_listings::{
            user_active_listings_v2::{
                self as user_active_listings_query, UserActiveListingsV2UserActiveListingsV2,
            },
            UserActiveListingsV2 as UserActiveListingsQuery,
        },
        user_tcomp_bids::{
            user_tcomp_bids::{self as user_tcomp_bids_query, UserTcompBidsUserTcompBids},
            UserTcompBids as UserTcompBidsQuery,
        },
        user_tensorswap_orders::{
            user_tensor_swap_orders::{
                self as user_tensorswap_orders_query, UserTensorSwapOrdersUserTswapOrders,
            },
            UserTensorSwapOrders as UserTensorSwapOrdersQuery,
        },
    },
    responses,
};

#[derive(Debug, Clone)]
pub struct TensorTradeClient {
    client: Client,
}

impl TensorTradeClient {
    pub fn new(api_key: String) -> Self {
        let client = Client::builder()
            .default_headers({
                let mut headers = header::HeaderMap::new();
                headers.insert(
                    "X-TENSOR-API-KEY",
                    header::HeaderValue::from_str(&api_key).unwrap(),
                );
                headers
            })
            .build()
            .unwrap();

        Self { client }
    }
}

#[async_trait::async_trait]
pub trait TensorTrade {
    // async fn get_active_listings(
    //     &self,
    //     slug: String,
    //     sort_by: SortBy,
    //     filters: Option<Filters>,
    //     limit: Option<Limit>,
    //     cursor: Option<Cursor>,
    // );

    async fn get_active_orders(
        &self,
        slug: String,
    ) -> Result<Vec<TensorSwapActiveOrdersTswapOrders>, anyhow::Error>;

    async fn get_tcomp_active_bids(
        &self,
        slug: String,
    ) -> Result<Vec<TcompBidsTcompBids>, anyhow::Error>;

    async fn get_token_mints_slugs(
        &self,
        token_mints: Vec<String>,
    ) -> Result<Vec<MintsMints>, anyhow::Error>;

    // ### Get Mint Active Listings, Bids AND/OR Active Orders
    // async fn get_token_mint_active_listings(&self, token_mint: String);

    // async fn get_token_mint_active_bids(&self, token_mint: String);

    // async fn get_token_mint_active_orders(&self, token_mint: String);

    // query MintList($slug: String!, $limit: Int, $after: String) {
    //     mintList(slug: $slug, limit: $limit, after: $after)
    //   }
    async fn get_mint_list(
        &self,
        slug: String,
        limit: Option<i64>,
        after: Option<String>,
    ) -> Result<Vec<String>, anyhow::Error>;

    // query CollectionMints(
    //     $slug: String!
    //     $sortBy: CollectionMintsSortBy!
    //     $filters: CollectionMintsFilters
    //     $cursor: String
    //     $limit: Int
    //   ) {
    //     collectionMintsV2(
    //       slug: $slug
    //       sortBy: $sortBy
    //       filters: $filters
    //       cursor: $cursor
    //       limit: $limit
    //     ) {
    //       mints {
    //         mint {
    //           onchainId
    //           rarityRankHR # HowRare score
    //           rarityRankTT # Tensor-computed rarity score (similar to HowRare)
    //         }
    //       }
    //       page {
    //         endCursor
    //         hasMore
    //       }
    //     }
    //   }
    // N0TE: Mints are tokens within a collection.
    async fn get_collection_mints(
        &self,
        slug: String,
        sort_by: collection_mints_query::CollectionMintsSortBy,
        filters: Option<collection_mints_query::CollectionMintsFilters>,
        cursor: Option<String>,
        limit: Option<i64>, // TODO: If this is `None`, it'll throw an error when sending the request.
    ) -> Result<CollectionMintsCollectionMintsV2, anyhow::Error>;

    async fn get_users_active_listings(
        &self,
        wallets: Vec<String>,
        sort_by: user_active_listings_query::ActiveListingsSortBy,
        cursor: Option<user_active_listings_query::ActiveListingsCursorInputV2>,
        limit: Option<i64>, // TODO: If this is `None`, it'll throw an error when sending the request.
        slug: Option<String>,
    ) -> Result<(), anyhow::Error>;

    async fn get_tensorswap_buy_single_nft_from_listing(
        &self,
        buyer: String,
        max_price: Decimal,
        mint: String,
        owner: String,
    ) -> Result<(), anyhow::Error>;

    async fn get_tensorswap_buy_nft(
        &self,
        buyer: String,
        max_price_lamports: Decimal,
        mint: String,
    ) -> Result<(), anyhow::Error>;
}

#[async_trait::async_trait]
impl TensorTrade for TensorTradeClient {
    async fn get_tcomp_active_bids(
        &self,
        slug: String,
    ) -> Result<Vec<TcompBidsTcompBids>, anyhow::Error> {
        let query = TcompBidsQuery::build_query(tcomp_bids_query::Variables { slug: slug.clone() });

        let response = self
            .client
            .post(TENSOR_TRADE_API_URL)
            .json(&query)
            .send()
            .await?;
        // .map(|response| response.error_for_status())??;

        let response_body: Response<tcomp_bids_query::ResponseData> = response.json().await?;

        if let Some(data) = response_body.data {
            Ok(data.tcomp_bids)
            // TODO: Warn user if `data.tcomp_bids` is empty.
            // TODO: This endpoint also returns all the trait bids for both compressed and non-compressed NFTs.
            // Any bid that has a non-empty attributes array is a trait bid.
        } else {
            // Err(TensorTradeError::NoResponseData);
            eprintln!("no response data");
            todo!()
        }
    }

    async fn get_user_tensorswap_active_orders(&self, wallet: String) -> Result<(), anyhow::Error> {
        let query =
            UserTensorSwapOrdersQuery::build_query(user_tensorswap_orders_query::Variables {
                owner: wallet,
            });

        let response = self
            .client
            .post(TENSOR_TRADE_API_URL)
            .json(&query)
            .send()
            .await?;
        // .map(|response| response.error_for_status())??;

        let response_body: Response<user_tensorswap_orders_query::ResponseData> =
            response.json().await?; // This error should be because of deserialization, not because of the HTTP request.

        dbg!(&response_body);

        if let Some(data) = response_body.data {
            dbg!(&data);
            Ok(())
        } else {
            // Err(TensorTradeError::NoResponseData);
            eprintln!("no response data");
            todo!()
        }
    }

    async fn get_user_tcomp_active_bids(&self, wallet: String) -> Result<(), anyhow::Error> {
        let query =
            UserTcompBidsQuery::build_query(user_tcomp_bids_query::Variables { owner: wallet });

        let response = self
            .client
            .post(TENSOR_TRADE_API_URL)
            .json(&query)
            .send()
            .await?;
        // .map(|response| response.error_for_status())??;

        let response_body: Response<user_tcomp_bids_query::ResponseData> = response.json().await?; // This error should be because of deserialization, not because of the HTTP request.

        dbg!(&response_body);

        if let Some(data) = response_body.data {
            dbg!(&data);
            Ok(())
        } else {
            // Err(TensorTradeError::NoResponseData);
            eprintln!("no response data");
            todo!()
        }
    }
}
