extern crate actix_web;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

#[macro_use]
extern crate cosworth;

use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use actix_web::{http, pred, server, App, Error, HttpRequest, HttpResponse};
use cosworth::response::json;

mod schema;
mod models;

fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn index(req: &HttpRequest) -> Result<HttpResponse, Error> {
    let id: String = req.match_info().query("id")?;
    let name = req.match_info().query("name")?;

    // TODO: do something with the database
    let _connection = establish_connection();

    match id.parse::<i32>() {
        Ok(n) => {
            let widget = models::todo::Todo { id: n, name: name, done: false };
            return Ok(json(&req, widget, http::StatusCode::OK)?);
        },
        Err(_e) => {
            return Ok(req.build_response(http::StatusCode::BAD_REQUEST)
                .content_type("text/plain")
                .body(hello!()));
        }
    }
}

fn main() {
    println!("{}", hello!());
    server::new(|| {
        App::new()
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
