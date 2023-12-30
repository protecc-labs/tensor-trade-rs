use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/mints.graphql",
    response_derives = "Debug"
)]
pub struct Mints;
