#![feature(extern_prelude)]
#[macro_use]
extern crate juniper;
extern crate relative_path;

pub mod model;
mod schema;
mod traits;
pub mod util;

use std::path::PathBuf;

///
/// Return the juniper Schema containing the queries and mutations.
///
pub fn schema() -> model::Schema {
    model::Schema::new(model::Query {}, model::Mutation {})
}

///
/// Create a new context. Only the files in the directory given by
/// `root_path` will be accessible.
///
pub fn context(root_path: PathBuf) -> model::Context {
    model::Context {
        root_path: root_path,
    }
}
