use std::collections::HashMap;
use actix_web::http::HeaderMap;
use bytes::Bytes;


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