use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(schema_path = "src/schema.graphql", query_path = "src/queries.graphql")]
pub struct CreateEventMutation;

fn main() {
    println!("Hello, world!");
}
