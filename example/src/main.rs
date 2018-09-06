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
use futures::Future;
use actix::prelude::*;
use actix_web::{http, middleware, pred, server, App, Path, State, Error,
                HttpRequest, HttpResponse, FutureResponse, AsyncResponder};

// cosworth
use cosworth::response::json;

mod db;
mod schema;
mod models;

use db::{CreateUser, DbExecutor};


// state with connection pool(s)
struct AppState {
    db_pool: Pool<ConnectionManager<PgConnection>>,
    db_addr: Addr<DbExecutor>,
}

// async request gets sent to actix "DbExecutor" actor
fn create(
    (name, state): (Path<String>, State<AppState>),
) -> FutureResponse<HttpResponse> {
    state
        .db_addr
        .send(CreateUser {
            name: name.into_inner(),
        })
        .from_err()
        .and_then(|res| match res {
            Ok(user) => Ok(HttpResponse::Ok().json(user)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}

// basic index handler
fn index(req: &HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    let query_id: String = req.match_info().query("id")?;
    let query_name = req.match_info().query("name")?;

    // get some data from the real database
    use schema::todos::dsl::*;
    use models::todo::*;
    let connection = req.state().db_pool.get().expect("Error loading connection");
    let mut results = todos.filter(done.eq(false))
        .limit(5)
        .load::<Todo>(&connection)
        .expect("Error loading posts");

    // return possible responses
    match query_id.parse::<i64>() {
        Ok(n) => {
            let widget = models::todo::Todo { id: n, name: query_name, done: false };
            results.push(widget);
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

    // heh heh heh
    let db_pool_1 = db_pool.clone();

    // actix stuff
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix::System::new("cosworth-example");
    let addr = SyncArbiter::start(3, move || DbExecutor(db_pool_1.clone()));

    server::new(move || {
        App::with_state(AppState{db_pool: db_pool.clone(), db_addr: addr.clone()})
            .middleware(middleware::Logger::default())
            .resource("/create/{name}", |r| r.method(http::Method::POST).with(create))
            .resource("/{id}/{name}", |r| {
                r.route()
                 .filter(pred::Get())
                 .filter(pred::Header("content-type", "application/json"))
                 .f(index)
            })
    })
        .bind("0.0.0.0:8080").unwrap()
        .run();
}
