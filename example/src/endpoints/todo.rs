use bytes::Bytes;
use cosworth::prelude::*;
use diesel::prelude::*;

use models::todo::*;


pub struct TodoDetailEndpoint {}

impl Endpoint for TodoDetailEndpoint {
  fn get(&self, context: &Context, request: &Request) -> Result<Response, Error> {
    let path_id = request.path_params.get("id").unwrap();
    match path_id.parse::<i64>() {
      Ok(n) => {
        use schema::todos::dsl::*;
        let db_result = todos.find(n)
          .load::<Todo>(context.db)
          .expect("Error loading todo");

        match db_result.len() {
          1 => {
            Response::new(200, TodoJson::from(&db_result[0]))
          },
          _ => {
            Response::new(404, json!({"detail": "Not found"}))
          }
        }
      },
      Err(_e) => {
        return Ok(Response {
          status: 400,
          headers: HeaderMap::new(),
          body: Bytes::from(hello!())
        });
      }
    }
  }
}
