pub mod config;
pub mod models;
pub mod pool;
mod schema;

use std::net::TcpListener;

use actix_web::{
    dev::Server,
    http::StatusCode,
    web::{self, Data, Form},
    App, HttpResponse, HttpServer,
};

use pool::{query_pool, Pool};

use crate::schema::subscriptions;
use diesel::prelude::*;
use models::{NewSubscription, Subscription};
use schema::subscriptions::dsl::*;

async fn health_check() -> HttpResponse<&'static str> {
    HttpResponse::with_body(StatusCode::OK, "OK")
}

async fn subscribe(form: Form<NewSubscription>, pool: Data<Pool>) -> HttpResponse {
    let res = query_pool(&pool, |conn| {
        diesel::insert_into(subscriptions::table)
            .values(&form.into_inner())
            .returning(Subscription::as_returning())
            .get_result(conn)
    })
    .await;

    match res {
        Ok(subscription) => HttpResponse::Ok().json(subscription),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

async fn get_subscriptions(_form: Form<NewSubscription>, pool: Data<Pool>) -> HttpResponse {
    let res = query_pool(&pool, |conn| {
        subscriptions.select(Subscription::as_select()).first(conn)
    })
    .await;

    match res {
        Ok(subscription) => HttpResponse::Ok().json(subscription),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub fn run(listener: TcpListener, pool: Pool) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .route("/healthz", web::get().to(health_check))
            .route("/subscriptions", web::post().to(get_subscriptions))
            .route("/subscribe", web::post().to(subscribe))
            .app_data(Data::new(pool.clone()))
    })
    .listen(listener)?
    .run();

    Ok(server)
}

#[cfg(test)]
mod tests {
    use crate::health_check;

    #[tokio::test]
    async fn health_check_succeeds() {
        let response = health_check().await;
        assert!(response.status().is_success())
    }
}
