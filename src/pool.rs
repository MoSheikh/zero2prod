// use crate::config;

use std::ops::Deref;
use std::pin::Pin;

use actix_web::{web::Data, FromRequest};
use deadpool_diesel::postgres::Object;
use deadpool_diesel::{Error, InteractError, Pool as DieselPool};
use diesel::PgConnection;
use secrecy::ExposeSecret;

use crate::config::DbSettings;

pub type Pool = DieselPool<deadpool_diesel::postgres::Manager>;

pub fn create_pool(settings: &DbSettings) -> Pool {
    DieselPool::builder(deadpool_diesel::postgres::Manager::new(
        settings.to_url().expose_secret(),
        deadpool::Runtime::Tokio1,
    ))
    .build()
    .unwrap_or_else(|e| panic!("Could not create pool for database: {e}"))
}

impl Deref for DbConn {
    type Target = Object;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct DbConn(pub Object);

impl FromRequest for DbConn {
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let pool = req.app_data::<Data<Pool>>().unwrap().clone();

        Box::pin(async move {
            let conn = pool.get().await.unwrap();
            Ok(DbConn(conn))
        })
    }
}

#[derive(Debug)]
pub enum QueryError {
    PoolError(PoolError<Error>),
    InteractError(InteractError),
    DieselError(diesel::result::Error),
}

impl From<PoolError<Error>> for QueryError {
    fn from(value: PoolError<Error>) -> Self {
        QueryError::PoolError(value)
    }
}

impl From<InteractError> for QueryError {
    fn from(value: InteractError) -> Self {
        QueryError::InteractError(value)
    }
}

impl From<diesel::result::Error> for QueryError {
    fn from(value: diesel::result::Error) -> Self {
        QueryError::DieselError(value)
    }
}

impl std::fmt::Display for QueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QueryError::PoolError(pool_error) => pool_error.fmt(f),
            QueryError::InteractError(interact_error) => interact_error.fmt(f),
            QueryError::DieselError(diesel_error) => diesel_error.fmt(f),
        }
    }
}

use deadpool::managed::PoolError;

pub async fn query_pool<F, T>(pool: &Pool, f: F) -> Result<T, QueryError>
where
    F: FnOnce(&mut PgConnection) -> Result<T, diesel::result::Error> + Send + 'static,
    T: Send + 'static,
{
    let conn = pool.get().await?;
    let res = conn.interact(f).await??;
    Ok(res)
}
