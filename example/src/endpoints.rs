use std::collections::HashMap;
use bytes::Bytes;
use serde_json;

use diesel;
use diesel::prelude::*;
use actix_web::*;
use actix_web::error::ErrorBadRequest;

use cosworth::endpoints::Endpoint;
use cosworth::helpers::{get_millis, RawRequest, RawResponse};
use cosworth::processor::Processor;

use models::todo::*;
use schema;

pub struct TodoCreateEndpoint {}

impl Endpoint for TodoCreateEndpoint {
  fn post(&self, context: &Processor, request: RawRequest) -> Result<RawResponse, Error> {
    use self::schema::todos::dsl::*;

    match serde_json::from_slice::<TodoJson>(&request.body) {
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

        let conn: &PgConnection = &context.0.get().unwrap();

        diesel::insert_into(todos)
            .values(&new_todo)
            .execute(conn)
            .map_err(|e| {
                println!("{:?}", e);
                error::ErrorInternalServerError("Error inserting todo")
            })?;

        let mut items = todos
            .filter(name.eq(&obj.name))
            .load::<Todo>(conn)
            .map_err(|_| error::ErrorInternalServerError("Error loading todos"))?;

        let queried_todo = items.pop().unwrap();

        return Ok(RawResponse {
          status: 200,
          headers: HashMap::new(),
          body: Bytes::from(serde_json::to_string(&TodoJson {
            id: Some(queried_todo.id as u64),
            name: queried_todo.name,
            done: Some(queried_todo.done)
          })?)
        });
      },
      Err(e) => {
        return Err(ErrorBadRequest(e));
      }
    }
  }
}