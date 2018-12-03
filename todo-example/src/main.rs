#[macro_use] extern crate horrorshow;
extern crate lazy_static;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;

use webframework::prelude::*;

mod controller;
mod views;
mod database;
mod models;

routing! {
    RootServer => {
        GET "/" => controller::root;
        POST "/create" => controller::create_task;
        >> NotFound => controller::not_found;
    }
}

fn main() -> WebResult<()> {
    let server = webframework::server::load();

    server.handle_with(RootServer)
}

