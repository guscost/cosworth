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
use endpoints::index::index;
use endpoints::todos::todo_list;
use endpoints::todo::todo_detail;

// app setup
fn main() {
  println!("{}", hello!());

  // start logging
  std::env::set_var("RUST_LOG", "actix_web=info");
  env_logger::init();

  // DB connection pool, and address for "request processor" actors
  let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found.");
  let db_pool = postgres!(db_url);
  let processors = processors!(3, db_pool);

  server::new(move || {
    let context = Context{processors: processors.clone()};
    let app = app!(context);
    middleware!(app, Logger);

    route!(app, "/hello", index);
    route!(app, "/todos", todo_list);
    route!(app, "/todos/{id}", todo_detail);

    return app;
  })
    .bind("0.0.0.0:8080").unwrap()
    .run();
}
