use cosworth::prelude::*;
use schema::todos;


#[table_name="todos"]
#[derive(Insertable, Queryable)]
pub struct Todo {
  pub id: i64,
  pub name: String,
  pub done: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoJson {
  pub id: Option<u64>,
  pub name: String,
  pub done: Option<bool>,
}

impl From<&Todo> for TodoJson {
  fn from(obj: &Todo) -> Self {
    return Self {
      id: Some(obj.id as u64),
      name: obj.name.clone(),
      done: Some(obj.done)
    };
  }
}

impl From<&TodoJson> for Todo {
  fn from(obj: &TodoJson) -> Self {
    return Self {
      id: match obj.id { Some(x) => x, None => get_millis() } as i64,
      name: obj.name.clone(),
      done: match obj.done { Some(x) => x, None => false }
    };
  }
}
