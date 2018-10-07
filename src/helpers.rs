//use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use bytes::Bytes;
use actix_web::http::HeaderMap;

/// important parts of an HTTP request
pub struct RawRequest {
  pub method: String,
  //pub cookies: HashMap<String, String>,
  pub headers: HeaderMap,
  pub body: Bytes,
}

/// important parts of an HTTP response
pub struct RawResponse {
  pub status: u16,
  //pub cookies: HashMap<String, String>,
  pub headers: HeaderMap,
  pub body: Bytes,
}

/// default values for an HTTP response
impl Default for RawResponse {
    fn default() -> RawResponse {
      return RawResponse {
        status: 404,
        //cookies: HashMap::new(),
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
