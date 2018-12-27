pub use std::collections::HashMap;
pub use futures::Future;
pub use num_cpus::{get as num_cpus};

pub use actix::{SyncArbiter as ActixSyncArbiter, System as ActixSystem};
pub use actix_web::error::*;
pub use actix_web::middleware::*;
pub use actix_web::http::HeaderMap;
pub use actix_web::{http, server, App, AsyncResponder, HttpMessage,
  HttpRequest, HttpResponse};

pub use super::endpoints::Endpoint;
pub use super::utilities::{get_millis, AppState, Request, Response};
pub use super::processor::{Context, RequestMessage};
