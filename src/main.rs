use rocket::response::content;
use rocket::State;

use juniper::{graphql_object, EmptyMutation, EmptySubscription, RootNode};

struct Context;
impl juniper::Context for Context {}

struct Query;

#[graphql_object(context = Context)]
impl Query {
    fn hello() -> &'static str {
        "Hello, World!"
    }
}

type Schema = RootNode<Query, EmptyMutation<Context>, EmptySubscription<Context>>;

#[rocket::get("/")]
fn graphiql() -> content::RawHtml<String> {
    juniper_rocket::graphiql_source("/graphql", None)
}

#[rocket::get("/graphql?<request..>")]
async fn get_graphql_handler(
    context: &State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(schema, context).await
}

#[rocket::post("/graphql", data = "<request>")]
async fn post_graphql_handler(
    context: &State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(schema, context).await
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .manage(Context)
        .manage(Schema::new(
            Query,
            EmptyMutation::<Context>::new(),
            EmptySubscription::<Context>::new(),
        ))
        .mount(
            "/",
            rocket::routes![graphiql, get_graphql_handler, post_graphql_handler],
        )
        .launch()
        .await?;
    Ok(())
}
