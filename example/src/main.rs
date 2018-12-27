extern crate bytes;
extern crate env_logger;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

#[macro_use]
extern crate cosworth;

mod endpoints;
mod models;
mod schema;

use cosworth::prelude::*;
use endpoints::index::IndexEndpoint;
use endpoints::todos::TodoListEndpoint;
use endpoints::todo::TodoDetailEndpoint;


fn main() {
  println!("{}", hello!());

  // start logging
  std::env::set_var("RUST_LOG", "actix_web=info");
  env_logger::init();

  // create DB connection pool and address for request processor actors
  let db_url = std::env::var("COSWORTH_DATABASE_URL").expect("COSWORTH_DATABASE_URL not found.");
  let db_pool = postgres!(db_url);

  // start server
  cosworth!(
    context!(db_pool),
    middleware!(Logger),
    route!("/hello", IndexEndpoint),
    route!("/todos", TodoListEndpoint),
    route!("/todos/{id}", TodoDetailEndpoint)
  );
}
