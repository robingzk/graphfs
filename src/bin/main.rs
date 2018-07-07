#![feature(plugin)]
#![feature(custom_attribute)]
#![plugin(rocket_codegen)]

extern crate graphfs;
extern crate juniper;
extern crate juniper_rocket;
extern crate rocket;

use std::path::PathBuf;

use rocket::response::content;
use rocket::State;

use graphfs::model::{Context, Schema};

#[get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[get("/graphql?<request>")]
fn get_graphql_handler(
    context: State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

#[post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

//
// Return the root path. Currently use the current directory
// but should be changed later.
//
fn root_path() -> PathBuf {
    let mut path = PathBuf::new();
    path.push("./src");
    path
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .manage(graphfs::context(root_path()))
        .manage(graphfs::schema())
        .mount(
            "/",
            routes![graphiql, get_graphql_handler, post_graphql_handler],
        )
}

fn main() {
    rocket().launch();
}
