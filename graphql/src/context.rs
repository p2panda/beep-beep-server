use beep_beep_db::ConnectionPool;

#[derive(Clone)]
pub struct Context {
    pub pool: ConnectionPool,
    pub cddl_schema: String,
}

impl Context {
    pub fn new(pool: ConnectionPool, cddl_schema: String) -> Self {
        Self { pool, cddl_schema }
    }
}

impl juniper::Context for Context {
    // To make our context usable by Juniper, we have to implement a marker trait
}
