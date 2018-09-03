extern crate actix_web;

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
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;

// actix-web 
use actix_web::{http, pred, server, App, Error, HttpRequest, HttpResponse};

// cosworth
use cosworth::response::json;

mod schema;
mod models;

// state with connection pool
struct AppState {
    pool: Pool<ConnectionManager<PgConnection>>,
}

// index handler
fn index(req: &HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    let query_id: String = req.match_info().query("id")?;
    let query_name = req.match_info().query("name")?;

    // get some data from the real database
    use schema::todos::dsl::*;
    use models::todo::*;
    let connection = req.state().pool.get().expect("Error loading connection");
    let mut results = todos.filter(done.eq(false))
        .limit(5)
        .load::<Todo>(&connection)
        .expect("Error loading posts");

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

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not found.");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = diesel::r2d2::Pool::builder().build(manager).expect("Failed to create pool.");

    server::new(move || {
        App::with_state(AppState{pool: pool.clone()})
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
