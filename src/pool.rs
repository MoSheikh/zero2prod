// use crate::config;

use crate::config::DbSettings;
use deadpool_diesel::Pool as DieselPool;

pub type Pool = DieselPool<deadpool_diesel::postgres::Manager>;

pub fn create_pool(config: &DbSettings) -> Pool {
    DieselPool::builder(deadpool_diesel::postgres::Manager::new(
        config.url.clone(),
        deadpool::Runtime::Tokio1,
    ))
    .build()
    .unwrap_or_else(|e| panic!("Could not create pool for database: {e}"))
}
