extern crate actix_web;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

#[macro_use]
extern crate cosworth;


use actix_web::{http, server, App, Path, Responder};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
struct Widget<'a> {
    id: u32,
    name: &'a str,
}

fn to_json<T>(obj: &T) -> String
where
    T: Serialize
{
    return serde_json::to_string(&obj).unwrap();
}

fn index(info: Path<(u32, String)>) -> impl Responder {
    let widget = Widget { id: info.0, name: &info.1 };
    return to_json(&widget);
}

fn main() {
    println!("{}", hello!());
    server::new(
        || App::new()
            .route("/{id}/{name}/", http::Method::GET, index))
        .bind("127.0.0.1:8080").unwrap()
        .run();
}
