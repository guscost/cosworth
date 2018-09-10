//! request processing actor
use std::time::{SystemTime, UNIX_EPOCH};

use actix::prelude::*;
use actix_web::*;

use diesel;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
//use uuid;

use models::todo::*;
use schema;

/// timestamp snowflake ID thing
fn get_millis() -> u64 {
  let start = SystemTime::now();
  let since_the_epoch = start.duration_since(UNIX_EPOCH)
    .expect("Time went backwards");
  return since_the_epoch.as_secs() * 1000 +
         since_the_epoch.subsec_nanos() as u64 / 1_000_000 << 22;
}

/// request processing actor. We are going to run 3 of them in parallel.
pub struct Processor(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for Processor {
  type Context = SyncContext<Self>;
}


/// message for creating a new todo
pub struct CreateTodo {
  pub body: String,
}
impl Message for CreateTodo {
  type Result = Result<HttpResponse, Error>;
}

impl Handler<CreateTodo> for Processor {
  type Result = Result<HttpResponse, Error>;

  fn handle(&mut self, msg: CreateTodo, _: &mut Self::Context) -> Self::Result {
    use self::schema::todos::dsl::*;

    match serde_json::from_slice::<TodoJson>(&body) {
      Ok(obj)  => {
        
        let new_id: u64;
        match msg.id {
            Some(x) => new_id = x,
            None => new_id = get_millis()
        }

        let new_done: bool;
        match msg.done {
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
            .filter(name.eq(&msg.name))
            .load::<Todo>(conn)
            .map_err(|_| error::ErrorInternalServerError("Error loading person"))?;

        let queried_todo = items.pop().unwrap();

        return Ok(HttpResponse::Ok().json(TodoJson {
          id: Some(queried_todo.id as u64),
          name: queried_todo.name,
          done: Some(queried_todo.done)
        }));
      },
      Err(_) => {
        let response = HttpResponse::BadRequest()
          .header("Content-Type", "text/javascript")
          .body(format!("{{\"error\": \"{}\"}}", e));
        return Box::new(futures::future::err(req));
      }
    }
  }
}