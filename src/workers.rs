use actix::prelude::*;
use actix_web::error::Error;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use endpoints::Endpoint;
use http::{Request, Response};

/// request processing actor. We are going to run N of them in parallel.
pub struct Worker {
  pub db_pool: Pool<ConnectionManager<PgConnection>>
}
impl Actor for Worker {
  // actix actors have their own "context", this is confusing but reuse the name below
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

/// "unwrapped" context struct will be passed into endpoint methods
pub struct Context<'a> {
  pub db: &'a PgConnection
}

/// process a request
impl<'a> Handler<RequestMessage<'a>> for Worker {
  type Result = Result<Response, Error>;

  fn handle(&mut self, msg: RequestMessage, _: &mut Self::Context) -> Self::Result {
    let context = Context { db: &self.db_pool.get().unwrap() };
    return msg.endpoint.handle(&context, msg.request);
  }
}

// actix app state with pool of workers
pub struct AppState { pub workers: Addr<Worker> }
