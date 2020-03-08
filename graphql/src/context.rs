use beep_beep_db::ConnectionPool;

#[derive(Clone)]
pub struct Context {
    pub pool: ConnectionPool,
}

impl Context {
    pub fn new(pool: ConnectionPool) -> Self {
        Self {
            pool,
        }
    }
}

impl juniper::Context for Context {
    // To make our context usable by Juniper, we have to implement a marker trait
}
