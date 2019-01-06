use cosworth;
use cosworth::prelude::*;


pub struct IndexEndpoint {}

impl Endpoint for IndexEndpoint {
  fn get(&self, _context: &Context, _request: &Request) -> Result<Response, Error> {
    Response::new(200, hello!())
  }
}
