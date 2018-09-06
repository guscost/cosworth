use schema::todos;

#[derive(Insertable)]
#[table_name="todos"]
pub struct NewTodo<'a> {
    pub id: &'a i64,
    pub name: &'a str,
    pub done: &'a bool,
}

#[derive(Queryable)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub id: i64,
    pub name: String,
    pub done: bool,
}
