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
use std::collections::HashMap;
use std::env;

// diesel
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;

// actix-web
use futures::Future;
use actix::prelude::*;
use actix_web::{
  http, middleware, pred, server, App, AsyncResponder, Error,
  HttpMessage, HttpRequest, HttpResponse
};

// cosworth
use cosworth::helpers::RawRequest;
use cosworth::processor::{Processor, ProcessRequest};

// example project modules
mod endpoints;
mod models;
mod schema;

use endpoints::test::IndexEndpoint;
use endpoints::todos::TodosEndpoint;

// state with connection pool
struct AppState { processors: Addr<Processor> }

// macros to register endpoints
endpoint!(index, IndexEndpoint);
endpoint!(create_todo, TodosEndpoint);

// app setup
fn main() {
  println!("{}", hello!());

  // DB connection pool
  let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not found.");
  let db_manager = ConnectionManager::<PgConnection>::new(db_url);
  let db_pool = Pool::builder().build(db_manager).expect("Failed to create pool.");

  // actix stuff
  ::std::env::set_var("RUST_LOG", "actix_web=info");
  env_logger::init();
  let _sys = actix::System::new("cosworth-example");
  let addr = SyncArbiter::start(3, move || Processor(db_pool.clone()));

  server::new(move || {
    App::with_state(AppState{processors: addr.clone()})
      .middleware(middleware::Logger::default())
      .resource("/create", |r| {
        r.route()
         .filter(pred::Header("content-type", "application/json"))
         .f(create_todo)
      })
      .resource("/{id}/{name}", |r| {
        r.route()
         .filter(pred::Get())
         .f(index)
      })
  })
    .bind("0.0.0.0:8080").unwrap()
    .run();
}
