use anyhow::Result;
use graphql_client::{GraphQLQuery, Response};

use crate::types::queries_scalar::{Byte, Decimal};

use super::{constants::TENSOR_TRADE_API_URL, TensorTradeClient};

mod queries;

pub(crate) use queries::{
    take_bid_tx as take_bid_tx_query, tensor_swap_active_orders as tensorswap_active_orders_query,
    tswap_buy_nft_tx as tswap_buy_nft_tx_query,
    tswap_buy_single_listing_tx as tswap_buy_single_listing_tx_query,
    tswap_delist_nft_tx as tswap_delist_nft_tx_query,
    tswap_edit_single_listing_tx as tswap_edit_single_listing_tx_query,
    tswap_list_nft_tx as tswap_list_nft_tx_query, tswap_sell_nft_tx as tswap_sell_nft_tx_query,
    TakeBidTx as TakeBidTxQuery, TensorSwapActiveOrders as TensorswapActiveOrdersQuery,
    TswapBuyNftTx as TswapBuyNftTxQuery, TswapBuySingleListingTx as TswapBuySingleListingTxQuery,
    TswapDelistNftTx as TswapDelistNftTxQuery,
    TswapEditSingleListingTx as TswapEditSingleListingTxQuery,
    TswapListNftTx as TswapListNftTxQuery, TswapSellNftTx as TswapSellNftTxQuery,
};

pub struct Tensorswap<'a>(pub(crate) &'a TensorTradeClient);

impl<'a> Tensorswap<'a> {
    /// TensorSwap Active Orders
    pub async fn get_active_orders(
        &self,
        slug: String,
    ) -> Result<Vec<tensorswap_active_orders_query::TensorSwapActiveOrdersTswapOrders>, anyhow::Error>
    {
        let query =
            TensorswapActiveOrdersQuery::build_query(tensorswap_active_orders_query::Variables {
                slug: slug.clone(),
            });

        let response = self
            .0
            .client
            .post(TENSOR_TRADE_API_URL)
            .json(&query)
            .send()
            .await?;
        // .map(|response| response.error_for_status())??;

        let response_body: Response<tensorswap_active_orders_query::ResponseData> =
            response.json().await?; // This error should be because of deserialization, not because of the HTTP request.

        dbg!(&response_body);

        if let Some(data) = response_body.data {
            Ok(data.tswap_orders)
            // TODO: Warn user if `data.tswap_orders` is empty.
        } else {
            // Err(TensorTradeError::NoResponseData);
            eprintln!("no response data");
            todo!()
        }
    }

    pub async fn get_buy_listing_tx(
        &self,
        buyer: String,
        max_price: String,
        mint: String,
        owner: String,
    ) -> anyhow::Result<Option<(Option<Byte>, Byte)>> {
        let query = TswapBuySingleListingTxQuery::build_query(
            tswap_buy_single_listing_tx_query::Variables {
                buyer,
                max_price: Decimal::new(max_price),
                mint,
                owner,
            },
        );

        let response = self
            .0
            .client
            .post(TENSOR_TRADE_API_URL)
            .json(&query)
            .send()
            .await?;
        // .map(|response| response.error_for_status())??;

        let response_body: Response<tswap_buy_single_listing_tx_query::ResponseData> =
            response.json().await?; // This error should be because of deserialization, not because of the HTTP request.

        dbg!(&response_body);

        if let Some(data) = response_body.data {
            let txs = data.tswap_buy_single_listing_tx.txs[0].clone();
            let tx = txs.tx;
            let tx_v0 = txs.tx_v0;
            Ok(Some((tx, tx_v0)))
        } else {
            // Err(TensorTradeError::NoResponseData);
            eprintln!("no response data");
            todo!()
        }
    }

    pub async fn get_buy_order_tx(
        &self,
        buyer: String,
        max_price_lamports: String,
        mint: String,
    ) -> anyhow::Result<Option<(Option<Byte>, Byte)>> {
        let slug = self.0.collection().get_slug(mint.clone()).await?;

        if self.0.collection().is_compressed(slug.clone()).await? {
            eprintln!("cannot buy compressed NFTs - use tcomp buy instead");
            return Ok(None);
        }

        let active_orders = self.get_active_orders(slug).await?;

        dbg!(active_orders.len());
        // If there are no active orders, return an error.

        let address = &active_orders.get(0).unwrap().address;

        dbg!(&buyer);
        dbg!(&max_price_lamports);
        dbg!(&mint);
        dbg!(&address);

        let query = TswapBuyNftTxQuery::build_query(tswap_buy_nft_tx_query::Variables {
            buyer,
            max_price_lamports: Decimal::new(max_price_lamports),
            mint,
            pool: address.to_string(),
        });

        let response = self
            .0
            .client
            .post(TENSOR_TRADE_API_URL)
            .json(&query)
            .send()
            .await?;

        // .map(|response| response.error_for_status())??;
        // dbg!(&response.json().await?.data);
        let response_body: Response<tswap_buy_nft_tx_query::ResponseData> = response.json().await?;
        // This error should be because of deserialization, not because of the HTTP request.

        if let Some(data) = response_body.data {
            let txs = data.tswap_buy_nft_tx.txs[0].clone();
            let tx = txs.tx;
            let tx_v0 = txs.tx_v0;
            Ok(Some((tx, tx_v0)))
        } else {
            // Err(TensorTradeError::NoResponseData);
            eprintln!("no response data");
            todo!()
        }
    }

    // It automatically selects the first order/pool (highest price) to sell to.
    pub async fn get_sell_now_tx(
        &self,
        seller: String,
        mint: String,
    ) -> anyhow::Result<Option<(Option<Byte>, Byte)>> {
        let slug = self.0.collection().get_slug(mint.clone()).await?;

        if self.0.collection().is_compressed(slug.clone()).await? {
            eprintln!("cannot sell compressed NFTs - use tcomp buy instead");
            return Ok(None);
        }

        let active_orders = self.get_active_orders(slug).await?;
        // If there are no active orders, return an error.
        let highest_price_order = active_orders.get(0).unwrap().clone(); // TODO: Handle no orders.

        let min_price_lamports = highest_price_order.sell_now_price.unwrap().clone();
        let pool = highest_price_order.address.clone();
        let buyer = highest_price_order.owner_address.clone();

        dbg!(&min_price_lamports);
        dbg!(&pool);
        dbg!(&buyer);

        let query = TswapSellNftTxQuery::build_query(tswap_sell_nft_tx_query::Variables {
            min_price_lamports,
            mint,
            pool,
            seller,
            seller_token_account: None,
        });

        let response = self
            .0
            .client
            .post(TENSOR_TRADE_API_URL)
            .json(&query)
            .send()
            .await?;

        // .map(|response| response.error_for_status())??;
        // dbg!(&response.json().await?.data);
        let response_body: Response<tswap_sell_nft_tx_query::ResponseData> =
            response.json().await?;
        // This error should be because of deserialization, not because of the HTTP request.

        if let Some(data) = response_body.data {
            let txs = data.tswap_sell_nft_tx.txs[0].clone();
            let tx = txs.tx;
            let tx_v0 = txs.tx_v0;
            Ok(Some((tx, tx_v0)))
        } else {
            // Err(TensorTradeError::NoResponseData);
            eprintln!("no response data");
            todo!()
        }
    }

    // pub async fn get_sell_listing_tx(
    //     &self,
    //     seller: String,
    //     mint: String,
    //     price: Decimal,
    // ) -> anyhow::Result<()> {
    //     todo!()
    // }

    pub async fn get_list_nft_tx(
        &self,
        mint: &str,
        owner: &str,
        price: &str,
    ) -> anyhow::Result<Option<(Option<Byte>, Byte)>> {
        let slug = self.0.collection().get_slug(mint.into()).await?;

        if self.0.collection().is_compressed(slug.clone()).await? {
            eprintln!("cannot list compressed NFTs - use tcomp list instead");
            return Ok(None);
        }

        let query = TswapListNftTxQuery::build_query(tswap_list_nft_tx_query::Variables {
            mint: mint.into(),
            owner: owner.into(),
            price: Decimal::new(price.into()),
        });

        let response = self
            .0
            .client
            .post(TENSOR_TRADE_API_URL)
            .json(&query)
            .send()
            .await?;

        // .map(|response| response.error_for_status())??;
        // dbg!(&response.json().await?.data);
        let response_body: Response<tswap_list_nft_tx_query::ResponseData> =
            response.json().await?;
        // This error should be because of deserialization, not because of the HTTP request.

        if let Some(data) = response_body.data {
            let txs = data.tswap_list_nft_tx.txs[0].clone();
            let tx = txs.tx;
            let tx_v0 = txs.tx_v0;
            Ok(Some((tx, tx_v0)))
        } else {
            // Err(TensorTradeError::NoResponseData);
            eprintln!("no response data");
            todo!()
        }
    }

    pub async fn get_edit_listing_tx(
        &self,
        mint: &str,
        owner: &str,
        price: &str,
    ) -> anyhow::Result<Option<(Option<Byte>, Byte)>> {
        let slug = self.0.collection().get_slug(mint.into()).await?;

        if self.0.collection().is_compressed(slug.clone()).await? {
            eprintln!("cannot edit compressed NFTs - use tcomp edit instead");
            return Ok(None);
        }

        let query = TswapEditSingleListingTxQuery::build_query(
            tswap_edit_single_listing_tx_query::Variables {
                mint: mint.into(),
                owner: owner.into(),
                price: Decimal::new(price.into()),
            },
        );

        let response = self
            .0
            .client
            .post(TENSOR_TRADE_API_URL)
            .json(&query)
            .send()
            .await?;

        // .map(|response| response.error_for_status())??;
        // dbg!(&response.json().await?.data);
        let response_body: Response<tswap_edit_single_listing_tx_query::ResponseData> =
            response.json().await?;
        // This error should be because of deserialization, not because of the HTTP request.

        if let Some(data) = response_body.data {
            let txs = data.tswap_edit_single_listing_tx.txs[0].clone();
            let tx = txs.tx;
            let tx_v0 = txs.tx_v0;
            Ok(Some((tx, tx_v0)))
        } else {
            // Err(TensorTradeError::NoResponseData);
            eprintln!("no response data");
            todo!()
        }
    }

    pub async fn get_delist_nft_tx(
        &self,
        mint: &str,
        owner: &str,
    ) -> anyhow::Result<Option<(Option<Byte>, Byte)>> {
        let slug = self.0.collection().get_slug(mint.into()).await?;

        if self.0.collection().is_compressed(slug.clone()).await? {
            eprintln!("cannot delist compressed NFTs - use tcomp delist instead");
            return Ok(None);
        }

        let query = TswapDelistNftTxQuery::build_query(tswap_delist_nft_tx_query::Variables {
            mint: mint.into(),
            owner: owner.into(),
        });

        let response = self
            .0
            .client
            .post(TENSOR_TRADE_API_URL)
            .json(&query)
            .send()
            .await?;
        // .map(|response| response.error_for_status())??;

        let response_body: Response<tswap_delist_nft_tx_query::ResponseData> =
            response.json().await?; // This error should be because of deserialization, not because of the HTTP request.

        dbg!(&response_body);

        if let Some(data) = response_body.data {
            let txs = data.tswap_delist_nft_tx.txs[0].clone();
            let tx = txs.tx;
            let tx_v0 = txs.tx_v0;
            Ok(Some((tx, tx_v0)))
        } else {
            // Err(TensorTradeError::NoResponseData);
            eprintln!("no response data");
            todo!()
        }
    }

    // pub async fn get_take_bid_tx(
    //     &self,
    //     bidder: String,
    //     mint: String,
    //     price: String,
    //     seller: String,
    // ) -> anyhow::Result<Option<(Option<Byte>, Byte)>> {
    //     let query = TakeBidTxQuery::build_query(take_bid_tx_query::Variables {
    //         bidder,
    //         mint,
    //         price: Decimal(price),
    //         seller,
    //     });

    //     let response = self
    //         .0
    //         .client
    //         .post(TENSOR_TRADE_API_URL)
    //         .json(&query)
    //         .send()
    //         .await?;
    //     // .map(|response| response.error_for_status())??;

    //     let response_body: Response<tswap_buy_single_listing_tx_query::ResponseData> =
    //         response.json().await?; // This error should be because of deserialization, not because of the HTTP request.
    //     dbg!(&response_body);

    //     if let Some(data) = response_body.data {
    //         let txs = data.tswap_buy_single_listing_tx.txs[0].clone();
    //         let tx = txs.tx;
    //         let tx_v0 = txs.tx_v0;
    //         Ok(Some((tx, tx_v0)))
    //     } else {
    //         // Err(TensorTradeError::NoResponseData);
    //         eprintln!("no response data");
    //         todo!()
    //     }
    // }
}
