use std::collections::HashMap;
use actix_web::http::HeaderMap;
use bytes::Bytes;

use serde_json;
use serde::Serialize;


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
impl Response {
  pub fn new(status: u16, body: impl Serialize) -> Result<Self, actix_web::Error> {
    Ok(Self {
      status: status,
      headers: HeaderMap::new(),
      body: Bytes::from(serde_json::to_string(&body)?)
    })
  }
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
