//! request processing actor
use bytes::Bytes;
use serde_json;

use actix::prelude::*;
use actix_web::*;
use actix_web::http::HeaderMap;

use diesel;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
//use uuid;

use cosworth::helpers::{get_millis, RawRequest, RawResponse};
use models::todo::*;
use schema;


/// request processing actor. We are going to run 3 of them in parallel.
pub struct Processor(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for Processor {
  type Context = SyncContext<Self>;
}


/// message for creating a new todo
pub struct CreateTodo {
  pub request: RawRequest,
}
impl Message for CreateTodo {
  type Result = Result<RawResponse, Error>;
}

impl Handler<CreateTodo> for Processor {
  type Result = Result<RawResponse, Error>;

  fn handle(&mut self, msg: CreateTodo, _: &mut Self::Context) -> Self::Result {
    use self::schema::todos::dsl::*;

    match serde_json::from_slice::<TodoJson>(&msg.request.body) {
      Ok(obj)  => {

        let new_id: u64;
        match obj.id {
            Some(x) => new_id = x,
            None => new_id = get_millis()
        }

        let new_done: bool;
        match obj.done {
            Some(x) => new_done = x,
            None => new_done = false
        }

        //let uuid = format!("{}", uuid::Uuid::new_v4());
        let new_todo = Todo {
            id: new_id as i64,
            name: obj.name.clone(),
            done: new_done
        };

        let conn: &PgConnection = &self.0.get().unwrap();

        diesel::insert_into(todos)
            .values(&new_todo)
            .execute(conn)
            .map_err(|e| {
                println!("{:?}", e);
                error::ErrorInternalServerError("Error inserting person")
            })?;

        let mut items = todos
            .filter(name.eq(&obj.name))
            .load::<Todo>(conn)
            .map_err(|_| error::ErrorInternalServerError("Error loading person"))?;

        let queried_todo = items.pop().unwrap();

        return Ok(RawResponse {
          status: 200,
          headers: HeaderMap::new(),
          body: Bytes::from(serde_json::to_string(&TodoJson {
            id: Some(queried_todo.id as u64),
            name: queried_todo.name,
            done: Some(queried_todo.done)
          })?)
        });
      },
      Err(e) => {
        return Ok(RawResponse {
          status: 200,
          headers: HeaderMap::new(),
          body: Bytes::from(format!("{{\"error\": \"{}\"}}", e))
        });
      }
    }
  }
}