extern crate bytes;
extern crate actix;
extern crate actix_web;
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

// diesel
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;

// actix-web
use actix::prelude::*;
use actix_web::{middleware, server, App};

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
  let _db_url = env::var("DATABASE_URL").expect("DATABASE_URL not found.");
  let _db_manager = ConnectionManager::<PgConnection>::new(_db_url);
  let db_pool = Pool::builder().build(_db_manager).expect("Failed to create pool.");

  // actix stuff
  ::std::env::set_var("RUST_LOG", "actix_web=info");
  env_logger::init();
  let _sys = actix::System::new("cosworth-example");
  let addr = SyncArbiter::start(3, move || Processor{db: db_pool.clone()});

  server::new(move || {
    let app = App::with_state(AppState{processors: addr.clone()})
      .middleware(middleware::Logger::default());
    let app = app.resource("/create", |r| r.route().f(create_todo));
    let app = app.resource("/{id}/{name}", |r| r.route().f(index));
    return app;
  })
    .bind("0.0.0.0:8080").unwrap()
    .run();
}
