use webframework::prelude::*;
use crate::views;
use crate::database::DB;
use crate::models::{Task, NewTask};

#[controller]
pub fn root(db: DB) -> WebResponse {
    let tasks = Task::find_all(&db)?;

    Ok(
        views::root(tasks)
    )
}

#[controller]
pub fn not_found(req: &Request) -> WebResponse {
    Ok(
        views::not_found(req.path())
    )
}

#[controller]
pub fn create_task(new_task: Form<NewTask>) -> WebResponse {
    Ok( "" )
}
