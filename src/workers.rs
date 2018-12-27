use actix::prelude::*;
use actix_web::error::Error;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use endpoints::Endpoint;
use http::{Request, Response};


/// request processing actor. We are going to run N of them in parallel.
pub struct Context {
  pub db: Pool<ConnectionManager<PgConnection>>
}
impl Actor for Context {
  // actix actors have their own "context", this is confusing but reuse the name anyway
  type Context = SyncContext<Self>;
}

/// message for processing requests
pub struct RequestMessage<'a> {
  pub request: Request,
  pub endpoint: &'a Endpoint
}
impl<'a> Message for RequestMessage<'a> {
  type Result = Result<Response, Error>;
}

/// process a request
impl<'a> Handler<RequestMessage<'a>> for Context {
  type Result = Result<Response, Error>;

  fn handle(&mut self, msg: RequestMessage, _: &mut Self::Context) -> Self::Result {
    return msg.endpoint.handle(&self, msg.request);
  }
}

// actix app state with pool of workers
pub struct AppState { pub workers: Addr<Context> }
