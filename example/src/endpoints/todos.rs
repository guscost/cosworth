use bytes::Bytes;
use serde_json;

use diesel;
use diesel::prelude::*;
use actix_web::error::*;
use actix_web::http::HeaderMap;

use cosworth::prelude::*;

use models::todo::*;
use schema;


pub struct TodosEndpoint{}
endpoint!(TodosEndpoint, create_todo);

impl Endpoint for TodosEndpoint {
  fn post(&self, context: &Processor, request: Request) -> Result<Response, Error> {
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
                ErrorInternalServerError("Error inserting todo")
            })?;

        let mut items = todos
            .filter(name.eq(&obj.name))
            .load::<Todo>(conn)
            .map_err(|_| ErrorInternalServerError("Error loading todos"))?;

        let queried_todo = items.pop().unwrap();

        return Ok(Response {
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
        return Err(ErrorBadRequest(e));
      }
    }
  }
}