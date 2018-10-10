extern crate actix;
extern crate actix_web;
extern crate bytes;
extern crate diesel;
extern crate futures;
extern crate serde_json;
extern crate serde;

pub mod endpoints;
pub mod helpers;
pub mod macros;
pub mod prelude;
pub mod processor;
pub mod response;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

// state with processor pool
pub struct AppState { pub processors: actix::Addr<processor::Processor> }

pub fn hello() -> String {
    return format!("Hello, cosworth v{}!", VERSION);
}
