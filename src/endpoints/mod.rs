use std::collections::HashMap;
use actix_web::Error;
use helpers::{RawRequest, RawResponse};
use processor::Processor;

pub trait Endpoint: Sync {
  fn handle (&self, context: &Processor, request: RawRequest) -> Result<RawResponse, Error> {
    match &request.method[..] {
      "GET" => self.get(context, request),
      "POST" => self.post(context, request),
      "PUT" => self.put(context, request),
      "DELETE" => self.delete(context, request),
      "OPTIONS" => self.options(context, request),
      _ => Ok(RawResponse { status: 405, ..Default::default() }),
    }
  }
  #[allow(unused_variables)]
  fn get(&self, context: &Processor, request: RawRequest) -> Result<RawResponse, Error> {
    return Ok(RawResponse { status: 405, ..Default::default() });
  }
  #[allow(unused_variables)]
  fn post(&self, context: &Processor, request: RawRequest) -> Result<RawResponse, Error> {
    return Ok(RawResponse { status: 405, ..Default::default() });
  }
  #[allow(unused_variables)]
  fn put(&self, context: &Processor, request: RawRequest) -> Result<RawResponse, Error> {
    return Ok(RawResponse { status: 405, ..Default::default() });
  }
  #[allow(unused_variables)]
  fn delete(&self, context: &Processor, request: RawRequest) -> Result<RawResponse, Error> {
    return Ok(RawResponse { status: 405, ..Default::default() });
  }
  #[allow(unused_variables)]
  fn options(&self, context: &Processor, request: RawRequest) -> Result<RawResponse, Error> {
    // Tell CORS to go away
    let mut headers = HashMap::new();
    headers.insert(
      "Access-Control-Allow-Headers".to_owned(),
      match request.headers.get("Access-Control-Request-Headers") {
        Some(x) => x.to_owned(),
        None => "Accept, Accept-Language, Content-Language, Content-Type".to_owned()
      }
    );
    headers.insert(
      "Access-Control-Allow-Methods".to_owned(),
      "GET, POST, PUT, DELETE, OPTIONS".to_owned()
    );
    headers.insert(
      "Access-Control-Allow-Origin".to_owned(),
      "*".to_owned()
    );
    return Ok(RawResponse {
      status: 200,
      headers: headers,
      ..Default::default()
    });
  }
}
