#[macro_use]
extern crate diesel;
extern crate dotenv;

mod db;
pub mod models;
pub mod schema;

pub use self::db::{get_connection_pool, ConnectionPool};
