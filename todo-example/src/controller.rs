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
pub fn create_task(new_task: Form<NewTask>, db: DB) -> WebResponse {
    let new_task = new_task.get("task")?;

    Task::create(&db, new_task)?;

    Ok(
        Redirect::from_str("/")?
    )
}
