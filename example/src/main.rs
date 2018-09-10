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
use std::collections::HashMap;

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
use cosworth::response::json;

// example project modules
mod helpers;
mod models;
mod processor;
mod schema;

use helpers::RawRequest;


/// state with connection pool(s)
struct AppState {
  raw_db_pool: Pool<ConnectionManager<PgConnection>>,
  processors: Addr<processor::Processor>,
}

/// async POST handler
fn create(req: &HttpRequest<AppState>) -> Box<Future<Item = HttpResponse, Error = Error>> {
  let req = req.clone();
  return req.body()
    .from_err()
    .and_then(move |body| {
      return req.state().processors
        .send(processor::CreateTodo {request: RawRequest {
          cookies: HashMap::new(),
          headers: HashMap::new(),
          body: body
        }})
        .from_err()
        .and_then(|res| match res {
          Ok(obj) => Ok(HttpResponse::Ok().body(obj.body)),
          Err(_) => Ok(HttpResponse::InternalServerError().into()),
        });
    })
    .responder();
}

// basic index handler
fn index(req: &HttpRequest<AppState>) -> Result<HttpResponse, Error> {
  let query_id: String = req.match_info().query("id")?;
  let query_name = req.match_info().query("name")?;

  // get some data from the real database
  use schema::todos::dsl::*;
  use models::todo::*;
  let connection = req.state().raw_db_pool.get().expect("Error loading connection");
  let db_results = todos.filter(done.eq(false))
    .limit(5)
    .load::<Todo>(&connection)
    .expect("Error loading todos");

  let mut results: Vec<TodoJson> = db_results.iter().map(|r| {
    TodoJson { id: Some(r.id as u64), name: r.name.clone(), done: Some(r.done) }
  }).collect();

  // return possible responses
  match query_id.parse::<u64>() {
    Ok(n) => {
      let todo = models::todo::TodoJson { 
        id: Some(n),
        name: query_name,
        done: Some(false)
      };
      results.push(todo);
      return Ok(json(&req, results, http::StatusCode::OK)?);
    },
    Err(_e) => {
      return Ok(req.build_response(http::StatusCode::BAD_REQUEST)
        .content_type("text/plain")
        .body(hello!()));
    }
  }
}

// app setup
fn main() {
  println!("{}", hello!());

  // DB connection pool
  let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not found.");
  let db_manager = ConnectionManager::<PgConnection>::new(db_url);
  let db_pool = Pool::builder().build(db_manager).expect("Failed to create pool.");

  // try using a raw db pool instance (not async)
  let raw_db_pool = db_pool.clone();

  // actix stuff
  ::std::env::set_var("RUST_LOG", "actix_web=info");
  env_logger::init();
  let _sys = actix::System::new("cosworth-example");
  let addr = SyncArbiter::start(3, move || processor::Processor(db_pool.clone()));

  server::new(move || {
    App::with_state(AppState{raw_db_pool: raw_db_pool.clone(), processors: addr.clone()})
      .middleware(middleware::Logger::default())
      //.resource("/create/{name}", |r| r.method(http::Method::POST).with(create))
      .resource("/create", |r| {
        r.route()
         .filter(pred::Post())
         .filter(pred::Header("content-type", "application/json"))
         .f(create)
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
