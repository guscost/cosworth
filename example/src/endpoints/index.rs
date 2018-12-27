use bytes::Bytes;
use cosworth;
use cosworth::prelude::*;


pub struct IndexEndpoint {}

impl Endpoint for IndexEndpoint {
  fn get(&self, _context: &Context, _request: Request) -> Result<Response, Error> {
    return Ok(Response {
      status: 200,
      headers: HeaderMap::new(),
      body: Bytes::from(hello!())
    });
  }
}
