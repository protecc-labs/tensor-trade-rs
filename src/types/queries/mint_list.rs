use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/mint_list.graphql",
    response_derives = "Debug"
)]
pub struct MintList;
