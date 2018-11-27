use webframework::prelude::*;
use crate::views;
use crate::database::DB;
use crate::models::Task;

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
