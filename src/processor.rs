use actix::prelude::*;
use actix_web::error::Error;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use endpoints::Endpoint;
use utilities::{Request, Response};


/// request processing actor. We are going to run N of them in parallel.
pub struct Processor {
  pub db: Pool<ConnectionManager<PgConnection>>
}
impl Actor for Processor {
  type Context = SyncContext<Self>;
}

/// message for processing requests
pub struct ProcessRequest<'a> {
  pub request: Request,
  pub endpoint: &'a Endpoint
}
impl<'a> Message for ProcessRequest<'a> {
  type Result = Result<Response, Error>;
}

/// process a request
impl<'a> Handler<ProcessRequest<'a>> for Processor {
  type Result = Result<Response, Error>;

  fn handle(&mut self, msg: ProcessRequest, _: &mut Self::Context) -> Self::Result {
    return msg.endpoint.handle(&self, msg.request);
  }
}
