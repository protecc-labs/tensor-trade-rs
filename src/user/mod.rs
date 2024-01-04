use anyhow::Result;
use graphql_client::{GraphQLQuery, Response};

use super::{constants, TensorTradeClient};

mod queries;

pub(crate) use queries::{
    user_active_listings_v2 as user_active_listings_query,
    UserActiveListingsV2 as UserActiveListingsQuery,
};

pub struct User<'a>(pub(crate) &'a TensorTradeClient);

impl<'a> User<'a> {
    /// Get TensorSwap and TComp active listings for a user.
    /// If limit is not provided, it defaults to 50.
    pub async fn get_active_listings(
        &self,
        wallets: Vec<String>,
        sort_by: user_active_listings_query::ActiveListingsSortBy,
        cursor: Option<user_active_listings_query::ActiveListingsCursorInputV2>,
        mut limit: Option<i64>,
        slug: Option<String>,
    ) -> Result<(), anyhow::Error> {
        if limit.is_none() {
            limit = Some(50)
        };

        let query = UserActiveListingsQuery::build_query(user_active_listings_query::Variables {
            wallets,
            sort_by,
            cursor,
            limit,
            slug,
        });

        let response = self
            .0
            .client
            .post(constants::TENSOR_TRADE_API_URL)
            .json(&query)
            .send()
            .await?;
        // .map(|response| response.error_for_status())??;

        let response_body: Response<user_active_listings_query::ResponseData> =
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

    async fn get_user_tensorswap_active_orders(&self, wallet: String) -> Result<(), anyhow::Error> {
        todo!()
    }

    async fn get_user_tcomp_active_bids(&self, wallet: String) -> Result<(), anyhow::Error> {
        todo!()
    }
}
