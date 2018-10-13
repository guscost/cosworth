use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use actix::Addr;
use actix_web::http::HeaderMap;
use bytes::Bytes;

use processor::Processor;


// context with processor pool
pub struct Context { pub processors: Addr<Processor> }

/// important parts of an HTTP request
pub struct Request {
  pub method: String,
  pub path_params: HashMap<String, String>,
  pub query_params: HashMap<String, String>,
  pub headers: HeaderMap,
  pub body: Bytes,
}

/// important parts of an HTTP response
pub struct Response {
  pub status: u16,
  pub headers: HeaderMap,
  pub body: Bytes,
}

/// default values for an HTTP response
impl Default for Response {
    fn default() -> Response {
      return Response {
        status: 404,
        headers: HeaderMap::new(),
        body: Bytes::new()
      };
    }
}


/// timestamp snowflake ID generator
/// TODO: use database instead of app server
pub fn get_millis() -> u64 {
  let start = SystemTime::now();
  let since_the_epoch = start.duration_since(UNIX_EPOCH)
    .expect("Time went backwards");
  return since_the_epoch.as_secs() * 1000 +
         since_the_epoch.subsec_nanos() as u64 / 1_000_000 << 22;
}
