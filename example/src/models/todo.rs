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
