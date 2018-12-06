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

    pub fn create(db: &DB, new: NewTask) -> WebResult<()> {
        Ok(diesel::insert_into(tasks::table)
            .values(&new)
            .execute(&**db)
            .map(|_| ())?)
    }
}

#[derive(Insertable, Deserialize, Debug)]
#[table_name = "tasks"]
pub struct NewTask {
    pub name: String,
}
