use actix_web::Error;
use helpers::{RawRequest, RawResponse};

trait Endpoint {
  fn handle (&self, request: RawRequest) -> Result<RawResponse, Error> {
    match &request.method[..] {
      "GET" => self.get(request),
      "POST" => self.post(request),
      "PUT" => self.put(request),
      "DELETE" => self.delete(request),
      "OPTIONS" => self.options(request),
      _ => Ok(RawResponse { status: 405, ..Default::default() }),
    }
  }
  #[allow(unused_variables)]
  fn get(&self, request: RawRequest) -> Result<RawResponse, Error> {
    return Ok(RawResponse { status: 405, ..Default::default() });
  }
  #[allow(unused_variables)]
  fn post(&self, request: RawRequest) -> Result<RawResponse, Error> {
    return Ok(RawResponse { status: 405, ..Default::default() });
  }
  #[allow(unused_variables)]
  fn put(&self, request: RawRequest) -> Result<RawResponse, Error> {
    return Ok(RawResponse { status: 405, ..Default::default() });
  }
  #[allow(unused_variables)]
  fn delete(&self, request: RawRequest) -> Result<RawResponse, Error> {
    return Ok(RawResponse { status: 405, ..Default::default() });
  }
  #[allow(unused_variables)]
  fn options(&self, request: RawRequest) -> Result<RawResponse, Error> {
    return Ok(RawResponse { status: 405, ..Default::default() });
  }
}
