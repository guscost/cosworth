//! Db executor actor
use actix::prelude::*;
use actix_web::*;
use diesel;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
//use uuid;

use models::todo::*;
use schema;

// Timestamp snowflake ID thing
use std::time::{SystemTime, UNIX_EPOCH};
fn get_millis() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    return since_the_epoch.as_secs() * 1000 +
            since_the_epoch.subsec_nanos() as u64 / 1_000_000 << 22;
}

/// This is db executor actor. We are going to run 3 of them in parallel.
pub struct DbExecutor(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

/// This is only message that this actor can handle, but it is easy to extend
/// number of messages.
pub struct CreateTodo {
    pub id: Option<u64>,
    pub name: String,
    pub done: Option<bool>,
}

impl Message for CreateTodo {
    type Result = Result<Todo, Error>;
}

impl Handler<CreateTodo> for DbExecutor {
    type Result = Result<Todo, Error>;

    fn handle(&mut self, msg: CreateTodo, _: &mut Self::Context) -> Self::Result {
        use self::schema::todos::dsl::*;

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
            name: msg.name.clone(),
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

        Ok(items.pop().unwrap())
    }
}