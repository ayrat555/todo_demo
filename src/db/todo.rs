use chrono::NaiveDateTime;

#[derive(Queryable, Debug, Clone)]
pub struct Todo {
    pub id: i64,
    pub text: String,
    pub done: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}


#[derive(Debug, Clone)]
pub struct NewTodo {
    pub text: String,
    pub done: bool,
    pub created_at: NaiveDateTime

}
