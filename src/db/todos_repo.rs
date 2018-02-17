use diesel::MysqlConnection as MyConn;
use result::{Result, Error};
use chrono::

use super::{Todo, NewTodo};
use super::schema;

pub struct TodosRepo;

impl TodosRepo {
    pub fn insert(conn: &MyConn, new_todo: &NewTodo) -> Result<Todo> {
        insert_into(todos::table)
    }
}
