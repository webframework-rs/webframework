extern crate webframework as wfw;
extern crate failure;

use crate::wfw::prelude::*;

#[controller]
fn handle404(req: Request) -> WebResponse {
    Ok(Response::from_string(format!("Could not find path {}", req.path()))
        .with_status(StatusCode::NOT_FOUND))
}

#[controller]
fn root(_req: Request) -> WebResponse {
    Ok(Response::from_string("Hello World!"))
}

#[controller]
fn about(_req: Request) -> WebResponse { unimplemented!() }

#[controller]
fn create_task(req: Request) -> WebResponse {
    Ok(Response::from_string("Hello World!"))
}

#[controller]
fn tasks(_req: Request) -> WebResponse { unimplemented!() }

#[controller]
fn tasks_json(_req: Request) -> WebResponse { unimplemented!() }

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
        DLG "/tasks" => TaskRouter;
        GET "/about" => about;
        GET "/" => root;
        NotFound => handle404;
    }
}

fn main() -> Result<(), failure::Error> {
    let server = wfw::server::load();

    server.handle_with(RootRouter)
}

