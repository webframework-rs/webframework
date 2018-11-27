use webframework::prelude::*;
use diesel::r2d2;

type InnerSQLite = r2d2::PooledConnection<r2d2::ConnectionManager<diesel::SqliteConnection>>;

lazy_static::lazy_static! {
    static ref DB_POOL: r2d2::Pool<r2d2::ConnectionManager<diesel::SqliteConnection>> = created_db_pool();
}

fn created_db_pool() -> r2d2::Pool<r2d2::ConnectionManager<diesel::SqliteConnection>> {
    let database_url = "./database.sqlite";

    let manager = r2d2::ConnectionManager::new(database_url);
    r2d2::Pool::new(manager).expect("Failed to create database pool")
}

pub struct DB(InnerSQLite);

impl std::ops::Deref for DB {
    type Target = InnerSQLite;

    fn deref(&self) -> &InnerSQLite {
        &self.0
    }
}

impl<'a> FromRequest<'a> for DB {
    fn from_request(_req: &'a Request) -> WebResult<DB> {
        Ok(DB(DB_POOL.get()?))
    }
}

pub mod schema {
    #![allow(proc_macro_derive_resolution_fallback)]
    table! {
        tasks (id) {
            id -> Integer,
            name -> Text,
            done -> Bool,
        }
    }
}
