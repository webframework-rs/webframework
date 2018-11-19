extern crate webframework as wfw;
extern crate failure;

use crate::wfw::prelude::*;

#[meta_controller]
fn handle404(req: Request) -> WebResponse {
    Ok(Response::from_string(format!("Could not find path {}", req.uri().path()))
        .with_status(StatusCode::NOT_FOUND))
}

#[controller]
fn root(_req: Request) -> WebResponse {
    Ok(Response::from_string("Hello World!"))
}

#[controller]
fn about(_req: Request) -> WebResponse {
    Ok(Response::from_string("About Webframework"))
}

#[controller]
fn create_task(req: Request) -> WebResponse {
    Ok(Response::from_string("Hello World!"))
}

#[controller]
fn tasks(_req: Request) -> WebResponse {
    Ok(Response::from_string("List of tasks!"))
}

#[controller]
fn tasks_json(_req: Request) -> WebResponse {
    Ok(Response::from_string("{msg: \"List of tasks\"}"))
}

routing! {
    TaskRouter => {
        POST "/create" => create_task;
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

