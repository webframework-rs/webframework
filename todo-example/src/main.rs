#[macro_use] extern crate horrorshow;
extern crate lazy_static;
#[macro_use] extern crate diesel;

use webframework::prelude::*;

mod controller;
mod views;
mod database;
mod models;

routing! {
    RootServer => {
        GET "/" => controller::root;
        >> NotFound => controller::not_found;
    }
}

fn main() -> WebResult<()> {
    let server = webframework::server::load();

    server.handle_with(RootServer)
}

