extern crate bytes;
extern crate env_logger;
extern crate futures;

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

  // DB connection pool
  let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not found.");
  let db_pool = postgres!(db_url);

  // actix stuff
  env::set_var("RUST_LOG", "actix_web=info");
  env_logger::init();
  let _sys = ActixSystem::new("cosworth-example");
  let addr = ActixSyncArbiter::start(3, move || Processor{db: db_pool.clone()});

  server::new(move || {
    let context = Context{processors: addr.clone()};
    let app = app!(context);

    middleware!(app, Logger);

    route!(app, "/create", create_todo); 
    route!(app, "/{id}/{name}", index);

    return app;
  })
    .bind("0.0.0.0:8080").unwrap()
    .run();
}
