use bytes::Bytes;
use serde_json;

use diesel::prelude::*;
use actix_web::error::*;
use actix_web::http::HeaderMap;

use cosworth;
use cosworth::prelude::*;


pub struct IndexEndpoint {}
endpoint!(IndexEndpoint, index);

impl Endpoint for IndexEndpoint {
  fn get(&self, context: &Processor, request: Request) -> Result<Response, Error> {
    let path_id = request.path_params.get("id").unwrap();
    let path_name = request.path_params.get("name").unwrap();

    // get some data from the real database
    use schema::todos::dsl::*;
    use models::todo::*;
    let conn: &PgConnection = &context.db.get().unwrap();
    let db_results = todos.filter(done.eq(false))
      .limit(50)
      .load::<Todo>(conn)
      .expect("Error loading todos");

    let mut results: Vec<TodoJson> = db_results.iter().map(|r| {
      TodoJson { id: Some(r.id as u64), name: r.name.clone(), done: Some(r.done) }
    }).collect();

    // return possible responses
    match path_id.parse::<u64>() {
      Ok(n) => {
        let todo = TodoJson { 
          id: Some(n),
          name: path_name.to_string(),
          done: Some(false)
        };
        results.push(todo);
        return Ok(Response {
          status: 200,
          headers: HeaderMap::new(),
          body: Bytes::from(serde_json::to_string(&results)?)
        });
      },
      Err(_e) => {
        return Ok(Response {
          status: 200,
          headers: HeaderMap::new(),
          body: Bytes::from(hello!())
        });
      }
    }
  }
}
