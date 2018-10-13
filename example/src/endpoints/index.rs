use bytes::Bytes;
use cosworth;
use cosworth::prelude::*;


pub struct IndexEndpoint {}
endpoint!(IndexEndpoint, index);

impl Endpoint for IndexEndpoint {
  fn get(&self, _context: &Processor, _request: Request) -> Result<Response, Error> {
    return Ok(Response {
      status: 200,
      headers: HeaderMap::new(),
      body: Bytes::from(hello!())
    });
  }
}
