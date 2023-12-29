use graphql_client::GraphQLQuery;

use crate::types::queries::general::{Decimal, Timestamp};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/tcomp_bids.graphql",
    response_derives = "Debug"
)]
pub struct TcompBids;
