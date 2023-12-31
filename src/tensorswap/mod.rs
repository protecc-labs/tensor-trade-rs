use anyhow::Result;
use graphql_client::{GraphQLQuery, Response};

use crate::tensorswap::queries::Decimal;

use super::{constants, TensorTradeClient};

mod queries;

pub(crate) use queries::{
    tensor_swap_active_orders as tensorswap_active_orders_query,
    tswap_buy_nft_tx as tswap_buy_nft_tx_query,
    tswap_buy_single_listing_tx as tswap_buy_single_listing_tx_query,
    tswap_delist_nft_tx as tswap_delist_nft_tx_query,
    tswap_edit_single_listing_tx as tswap_edit_single_listing_tx_query,
    tswap_list_nft_tx as tswap_list_nft_tx_query,
    TensorSwapActiveOrders as TensorswapActiveOrdersQuery, TswapBuyNftTx as TswapBuyNftTxQuery,
    TswapBuySingleListingTx as TswapBuySingleListingTxQuery,
    TswapDelistNftTx as TswapDelistNftTxQuery,
    TswapEditSingleListingTx as TswapEditSingleListingTxQuery,
    TswapListNftTx as TswapListNftTxQuery,
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
            .post(constants::TENSOR_TRADE_API_URL)
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
        max_price: Decimal,
        mint: String,
        owner: String,
    ) -> Result<(), anyhow::Error> {
        let query = TswapBuySingleListingTxQuery::build_query(
            tswap_buy_single_listing_tx_query::Variables {
                buyer,
                max_price,
                mint,
                owner,
            },
        );

        let response = self
            .0
            .client
            .post(constants::TENSOR_TRADE_API_URL)
            .json(&query)
            .send()
            .await?;
        // .map(|response| response.error_for_status())??;

        let response_body: Response<tswap_buy_single_listing_tx_query::ResponseData> =
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

    pub async fn get_buy_order_tx(
        &self,
        buyer: String,
        max_price_lamports: String,
        mint: String,
    ) -> Result<(), anyhow::Error> {
        let slug = self.0.collection().get_slug(mint.clone()).await?;

        if self.0.collection().is_compressed(slug.clone()).await? {
            eprintln!("cannot buy compressed NFTs - use tcomp buy instead");
            return Ok(());
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
            max_price_lamports: Decimal(max_price_lamports),
            mint,
            pool: address.to_string(),
        });

        let response = self
            .0
            .client
            .post(constants::TENSOR_TRADE_API_URL)
            .json(&query)
            .send()
            .await?;

        // .map(|response| response.error_for_status())??;
        // dbg!(&response.json().await?.data);
        let response_body: Response<tswap_buy_nft_tx_query::ResponseData> = response.json().await?;
        // This error should be because of deserialization, not because of the HTTP request.

        if let Some(data) = response_body.data {
            dbg!(&data);
            Ok(())
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
    ) -> anyhow::Result<()> {
        let slug = self.0.collection().get_slug(mint.into()).await?;

        if self.0.collection().is_compressed(slug.clone()).await? {
            eprintln!("cannot list compressed NFTs - use tcomp list instead");
            return Ok(());
        }

        let query = TswapListNftTxQuery::build_query(tswap_list_nft_tx_query::Variables {
            mint: mint.into(),
            owner: owner.into(),
            price: Decimal(price.into()),
        });

        let response = self
            .0
            .client
            .post(constants::TENSOR_TRADE_API_URL)
            .json(&query)
            .send()
            .await?;

        // .map(|response| response.error_for_status())??;
        // dbg!(&response.json().await?.data);
        let response_body: Response<tswap_list_nft_tx_query::ResponseData> =
            response.json().await?;
        // This error should be because of deserialization, not because of the HTTP request.

        if let Some(data) = response_body.data {
            dbg!(&data);
            Ok(())
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
    ) -> anyhow::Result<()> {
        let slug = self.0.collection().get_slug(mint.into()).await?;

        if self.0.collection().is_compressed(slug.clone()).await? {
            eprintln!("cannot edit compressed NFTs - use tcomp edit instead");
            return Ok(());
        }

        let query = TswapEditSingleListingTxQuery::build_query(
            tswap_edit_single_listing_tx_query::Variables {
                mint: mint.into(),
                owner: owner.into(),
                price: Decimal(price.into()),
            },
        );

        let response = self
            .0
            .client
            .post(constants::TENSOR_TRADE_API_URL)
            .json(&query)
            .send()
            .await?;

        // .map(|response| response.error_for_status())??;
        // dbg!(&response.json().await?.data);
        let response_body: Response<tswap_edit_single_listing_tx_query::ResponseData> =
            response.json().await?;
        // This error should be because of deserialization, not because of the HTTP request.

        if let Some(data) = response_body.data {
            dbg!(&data);
            Ok(())
        } else {
            // Err(TensorTradeError::NoResponseData);
            eprintln!("no response data");
            todo!()
        }
    }

    pub async fn get_delist_nft_tx(&self, mint: &str, owner: &str) -> anyhow::Result<()> {
        let slug = self.0.collection().get_slug(mint.into()).await?;

        if self.0.collection().is_compressed(slug.clone()).await? {
            eprintln!("cannot delist compressed NFTs - use tcomp delist instead");
            return Ok(());
        }

        let query = TswapDelistNftTxQuery::build_query(tswap_delist_nft_tx_query::Variables {
            mint: mint.into(),
            owner: owner.into(),
        });

        let response = self
            .0
            .client
            .post(constants::TENSOR_TRADE_API_URL)
            .json(&query)
            .send()
            .await?;
        // .map(|response| response.error_for_status())??;

        let response_body: Response<tswap_delist_nft_tx_query::ResponseData> =
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
}
