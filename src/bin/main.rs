#![feature(plugin)]
#![feature(custom_attribute)]
#![plugin(rocket_codegen)]

extern crate clap;
extern crate graphfs;
extern crate juniper;
extern crate juniper_rocket;
extern crate rocket;

use std::path::PathBuf;

use rocket::response::content;
use rocket::State;

use clap::{App, Arg, ArgMatches};

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

/// Create the rocket app.
fn rocket(root_path: PathBuf) -> rocket::Rocket {
    rocket::ignite()
        .manage(graphfs::context(root_path))
        .manage(graphfs::schema())
        .mount(
            "/",
            routes![graphiql, get_graphql_handler, post_graphql_handler],
        )
}

/// Read the command line arguments.
fn get_matches<'a>() -> ArgMatches<'a> {
    App::new("graphfs")
        .version("1.0")
        .about("Access a directory as a GraphQL endpoint")
        .arg(Arg::with_name("root").help("Path to the directory used as root"))
        .get_matches()
}

fn main() {
    let matches = get_matches();
    let root = matches.value_of("root").unwrap_or("./");
    println!("Serving directory {}", root);
    rocket(PathBuf::from(root)).launch();
}
