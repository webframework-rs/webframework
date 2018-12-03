#![allow(proc_macro_derive_resolution_fallback)]

use crate::database::schema::tasks;
use crate::database::DB;

use webframework::prelude::*;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub done: bool,
}

impl Task {
    pub fn find_all(db: &DB) -> WebResult<Vec<Task>> {
        Ok(tasks::table.load(&**db)?)
    }
}

#[derive(Insertable, Deserialize)]
#[table_name = "tasks"]
pub struct NewTask {
    pub name: String,
}
