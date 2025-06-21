pub mod config;
pub mod models;
pub mod pool;
mod schema;

use std::net::TcpListener;

use actix_web::{
    App, HttpResponse, HttpServer,
    dev::Server,
    error::{Error, ErrorInternalServerError},
    http::StatusCode,
    web::{self, Data, Form},
};

use pool::Pool;

use crate::schema::subscriptions;
use diesel::prelude::*;
use models::{NewSubscription, Subscription};
use schema::subscriptions::dsl::*;

async fn health_check() -> HttpResponse<&'static str> {
    HttpResponse::with_body(StatusCode::OK, "OK")
}

async fn subscribe(form: Form<NewSubscription>, pool: Data<Pool>) -> HttpResponse {
    let res = pool
        .get()
        .await
        .expect("Could not acquire connection from pool")
        .interact(|conn| {
            diesel::insert_into(subscriptions::table)
                .values(&form.into_inner())
                .returning(Subscription::as_returning())
                .get_result(conn)
        })
        .await
        .unwrap()
        .unwrap();

    HttpResponse::Ok().json(res)
}

async fn get_subscriptions(
    _form: Form<NewSubscription>,
    pool: Data<Pool>,
) -> Result<HttpResponse, Error> {
    if let Ok(Ok(response)) = pool
        .get()
        .await
        .expect("Could not acquire connection from pool")
        .interact(|conn| subscriptions.select(Subscription::as_select()).first(conn))
        .await
    {
        Ok(HttpResponse::Ok().json(response))
    } else {
        Err(ErrorInternalServerError("Could not retrieve subscription"))
    }
}

pub fn run(listener: TcpListener, settings: &config::Settings) -> Result<Server, std::io::Error> {
    let pool = Data::new(pool::create_pool(&settings.database));

    let server = HttpServer::new(move || {
        App::new()
            .route("/healthz", web::get().to(health_check))
            .route("/subscriptions", web::post().to(get_subscriptions))
            .route("/subscribe", web::post().to(subscribe))
            .app_data(pool.clone())
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
