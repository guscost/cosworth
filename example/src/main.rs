extern crate bytes;
extern crate env_logger;
extern crate num_cpus;

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


fn main() {
  println!("{}", hello!());

  // start logging
  std::env::set_var("RUST_LOG", "actix_web=info");
  env_logger::init();

  // read hostname param
  let host = match std::env::var("COSWORTH_HOST") {
    Ok(v) => v,
    Err(_e) => "0.0.0.0:8080".to_owned()
  };

  // determine number of request processors
  let processors_num = match std::env::var("COSWORTH_PROCESSORS_NUM") {
    Ok(v) => v.parse::<usize>().expect("COSWORTH_PROCESSORS_NUM must be an integer."),
    Err(_e) => num_cpus::get()
  };

  // create DB connection pool and address for request processor actors
  let db_url = std::env::var("COSWORTH_DATABASE_URL").expect("COSWORTH_DATABASE_URL not found.");
  let db_pool = postgres!(db_url);

  // init processor actors
  println!("Starting {} request processors...", processors_num);
  let processors = processors!(processors_num, db_pool);
  
  println!("Serving requests at http://{}...", host);
  server::new(move || {
    let context = Context{processors: processors.clone()};
    let app = app!(context);
    middleware!(app, Logger);

    route!(app, "/hello", index);
    route!(app, "/todos", todo_list);
    route!(app, "/todos/{id}", todo_detail);

    return app;
  })
    .maxconnrate(4096)
    .bind(host).unwrap()
    .run();
}
