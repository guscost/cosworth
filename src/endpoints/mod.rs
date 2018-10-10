use actix_web::error::Error;
use actix_web::http::HeaderMap;
use helpers::{Request, Response};
use processor::Processor;

pub trait Endpoint: Sync {
  fn handle (&self, context: &Processor, request: Request) -> Result<Response, Error> {
    match &request.method[..] {
      "GET" => self.get(context, request),
      "POST" => self.post(context, request),
      "PUT" => self.put(context, request),
      "DELETE" => self.delete(context, request),
      "OPTIONS" => self.options(context, request),
      _ => Ok(Response { status: 405, ..Default::default() }),
    }
  }
  #[allow(unused_variables)]
  fn get(&self, context: &Processor, request: Request) -> Result<Response, Error> {
    return Ok(Response { status: 405, ..Default::default() });
  }
  #[allow(unused_variables)]
  fn post(&self, context: &Processor, request: Request) -> Result<Response, Error> {
    return Ok(Response { status: 405, ..Default::default() });
  }
  #[allow(unused_variables)]
  fn put(&self, context: &Processor, request: Request) -> Result<Response, Error> {
    return Ok(Response { status: 405, ..Default::default() });
  }
  #[allow(unused_variables)]
  fn delete(&self, context: &Processor, request: Request) -> Result<Response, Error> {
    return Ok(Response { status: 405, ..Default::default() });
  }
  #[allow(unused_variables)]
  fn options(&self, context: &Processor, request: Request) -> Result<Response, Error> {
    // Tell CORS to go away
    let mut headers = HeaderMap::new();
    headers.insert(
      "Access-Control-Allow-Headers",
      match request.headers.get("Access-Control-Request-Headers") {
        Some(x) => x.to_owned(),
        None => "Accept, Accept-Language, Content-Language, Content-Type".parse().unwrap()
      }
    );
    headers.insert(
      "Access-Control-Allow-Methods",
      "GET, POST, PUT, DELETE, OPTIONS".parse().unwrap()
    );
    headers.insert(
      "Access-Control-Allow-Origin",
      "*".parse().unwrap()
    );
    return Ok(Response {
      status: 200,
      headers: headers,
      ..Default::default()
    });
  }
}
