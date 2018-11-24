extern crate webframework as wfw;
extern crate failure;
extern crate lazy_static;
extern crate regex;

use crate::wfw::prelude::*;

#[controller]
fn handle404(req: &Request) -> WebResponse {
    Ok(Response::from_string(format!("Could not find path {}", req.uri().path()))
        .with_status(StatusCode::NOT_FOUND))
}

#[controller]
fn root() -> WebResponse {
    Ok(Response::from_string("Hello World!"))
}

#[controller]
fn about() -> WebResponse {
    Ok(Response::from_string("About Webframework"))
}

#[controller]
fn create_task() -> WebResponse {
    Ok(Response::from_string("You can create a task here"))
}

#[controller]
fn new_task() -> WebResponse {
    Ok(Response::from_string("You can find a new task here"))
}

#[controller]
fn tasks() -> WebResponse {
    Ok(Response::from_string("List of tasks!"))
}

#[controller]
fn tasks_json() -> WebResponse {
    Ok(Response::from_string("{msg: \"List of tasks\"}"))
}

#[controller(params = "test")]
fn dynamic_path(test: String) -> WebResponse {
    Ok(Response::from_string(format!("Dynamic segment was: {}", test)))
}

routing! {
    TaskRouter => {
        POST "/create" => create_task;
        GET "/new" => new_task;
        GET "/" => {
            html => tasks;
            json => tasks_json;
        };
    }
}

routing! {
    RootRouter => {
        delegate "/tasks" => TaskRouter;
        GET "/about" => about;
        GET "/:test" => dynamic_path;
        GET "/" => root;
        >> NotFound => handle404;
    }
}

fn main() -> Result<(), failure::Error> {
    let server = wfw::server::load();

    let map = RootRouter.router_map();

    println!("{:#?}", map);

    server.handle_with(RootRouter)
}

