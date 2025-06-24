pub mod config;
pub mod models;
pub mod pool;
mod schema;
pub mod telemetry;

use std::net::TcpListener;

use actix_web::{
    dev::Server,
    http::StatusCode,
    web::{self, Data, Form},
    App, HttpResponse, HttpServer,
};
use diesel::prelude::*;
use tracing_actix_web::TracingLogger;

use models::{NewSubscription, Subscription};
use pool::{query_pool, Pool};
use schema::subscriptions;
use schema::subscriptions::dsl::*;
use tracing::instrument;

#[instrument]
async fn health_check() -> HttpResponse<&'static str> {
    HttpResponse::with_body(StatusCode::OK, "OK")
}

#[instrument(skip(pool))]
async fn subscribe(form: Form<NewSubscription>, pool: Data<Pool>) -> HttpResponse {
    tracing::info!("Saving new subscriber details...");
    let res = query_pool(&pool, |conn| {
        diesel::insert_into(subscriptions::table)
            .values(&form.into_inner())
            .returning(Subscription::as_returning())
            .get_result(conn)
    })
    .await;

    match res {
        Ok(subscription) => {
            tracing::info!("New subscriber details have been saved");
            HttpResponse::Ok().json(subscription)
        }
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[instrument(skip(pool))]
async fn get_subscriptions(form: Form<NewSubscription>, pool: Data<Pool>) -> HttpResponse {
    tracing::info!("Requesting subscriber details...");
    let query = query_pool(&pool, move |conn| {
        subscriptions
            .select(Subscription::as_select())
            .filter(subscriptions::email.eq(&form.email))
            .filter(subscriptions::name.eq(&form.name))
            .first(conn)
            .optional()
    })
    .await;

    match query {
        Ok(Some(subscription)) => {
            tracing::info!("Found subscription");
            HttpResponse::Ok().json(subscription)
        }
        Ok(None) => {
            tracing::info!("Did not find the requested subscription");
            HttpResponse::NotFound().finish()
        }
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

pub fn run(listener: TcpListener, pool: Pool) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
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
