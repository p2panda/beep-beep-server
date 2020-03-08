use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use log::info;

pub type ConnectionPool = Pool<ConnectionManager<PgConnection>>;

pub fn get_connection_pool(postgres_url: String, pool_size: u32) -> ConnectionPool {
    let manager = ConnectionManager::new(postgres_url);

    let pool = Pool::builder()
        .max_size(pool_size)
        .build(manager)
        .unwrap();

    info!("Connected to Postgres");

    pool
}
