use schema::todos;

#[derive(Insertable)]
#[table_name="todos"]
pub struct NewTodo<'a> {
    pub name: &'a str,
}

#[derive(Queryable)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub id: i64,
    pub name: String,
    pub done: bool,
}
