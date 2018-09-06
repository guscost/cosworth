//! Db executor actor
use actix::prelude::*;
use actix_web::*;
use diesel;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
//use uuid;

use models::todo::*;
use schema;

/// This is db executor actor. We are going to run 3 of them in parallel.
pub struct DbExecutor(pub Pool<ConnectionManager<PgConnection>>);

/// This is only message that this actor can handle, but it is easy to extend
/// number of messages.
pub struct CreateUser {
    pub name: String,
}

impl Message for CreateUser {
    type Result = Result<Todo, Error>;
}

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

impl Handler<CreateUser> for DbExecutor {
    type Result = Result<Todo, Error>;

    fn handle(&mut self, msg: CreateUser, _: &mut Self::Context) -> Self::Result {
        use self::schema::todos::dsl::*;

        //let uuid = format!("{}", uuid::Uuid::new_v4());
        let new_todo = NewTodo {
            name: &msg.name,
        };

        let conn: &PgConnection = &self.0.get().unwrap();

        diesel::insert_into(todos)
            .values(&new_todo)
            .execute(conn)
            .map_err(|_| error::ErrorInternalServerError("Error inserting person"))?;

        let mut items = todos
            .filter(name.eq(&msg.name))
            .load::<Todo>(conn)
            .map_err(|_| error::ErrorInternalServerError("Error loading person"))?;

        Ok(items.pop().unwrap())
    }
}