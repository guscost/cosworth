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

// std
use std::env;

// cosworth
use cosworth::prelude::*;

// example project modules
mod endpoints;
mod models;
mod schema;

// import endpoints
use endpoints::test::index;
use endpoints::todos::create_todo;

// app setup
fn main() {
  println!("{}", hello!());

  // start logging
  env::set_var("RUST_LOG", "actix_web=info");
  env_logger::init();

  // DB connection pool, and address for "request processor" actors
  let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not found.");
  let db_pool = postgres!(db_url);
  let processors = processors!(3, db_pool);

  server::new(move || {
    let context = Context{processors: processors.clone()};
    let app = app!(context);

    middleware!(app, Logger);

    route!(app, "/create", create_todo); 
    route!(app, "/{id}/{name}", index);

    return app;
  })
    .bind("0.0.0.0:8080").unwrap()
    .run();
}
