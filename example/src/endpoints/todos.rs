use bytes::Bytes;
use cosworth::prelude::*;
use diesel;
use diesel::prelude::*;
use serde_json;

use models::todo::*;
use schema;


pub struct TodoListEndpoint{}

impl Endpoint for TodoListEndpoint {

  fn get(&self, context: &Context, _request: &Request) -> Result<Response, Error> {
    use schema::todos::dsl::*;
    let db_results = todos.filter(done.eq(false))
      .limit(50)
      .load::<Todo>(context.db)
      .expect("Error loading todos");

    let results: Vec<TodoJson> = db_results.iter().map(|r| TodoJson::from(r)).collect();

    return Ok(Response {
      status: 200,
      headers: HeaderMap::new(),
      body: Bytes::from(serde_json::to_string(&results)?)
    });
  }

  fn post(&self, context: &Context, request: &Request) -> Result<Response, Error> {
    use self::schema::todos::dsl::*;

    match serde_json::from_slice::<TodoJson>(&request.body) {
      Ok(obj)  => {
        diesel::insert_into(todos)
          .values(&Todo::from(&obj))
          .execute(context.db)
          .map_err(|e| {
            println!("{:?}", e);
            ErrorInternalServerError("Error inserting todo")
          })?;

        let mut items = todos
          .filter(name.eq(&obj.name))
          .load::<Todo>(context.db)
          .map_err(|_| ErrorInternalServerError("Error loading todos"))?;

        let queried_todo = TodoJson::from(&items.pop().unwrap());

        return Ok(Response {
          status: 200,
          headers: HeaderMap::new(),
          body: Bytes::from(serde_json::to_string(&queried_todo)?)
        });
      },
      Err(e) => {
        return Err(ErrorBadRequest(e));
      }
    }
  }

}