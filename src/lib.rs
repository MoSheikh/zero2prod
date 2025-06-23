pub mod config;
pub mod models;
pub mod pool;
pub mod request;
mod schema;

use std::net::TcpListener;

use actix_web::{
    dev::Server,
    http::StatusCode,
    web::{self, Data, Form},
    App, HttpResponse, HttpServer,
};

use pool::{query_pool, Pool};
use request::RequestId;
use tracing::instrument;

use crate::schema::subscriptions;
use diesel::prelude::*;
use models::{NewSubscription, Subscription};
use schema::subscriptions::dsl::*;

#[instrument()]
async fn health_check(request_id: RequestId) -> HttpResponse<&'static str> {
    HttpResponse::with_body(StatusCode::OK, "OK")
}

#[instrument(skip(pool))]
async fn subscribe(
    form: Form<NewSubscription>,
    pool: Data<Pool>,
    request_id: RequestId,
) -> HttpResponse {
    tracing::info!("request {} - Saving new subscriber details...", request_id);
    let res = query_pool(&pool, |conn| {
        diesel::insert_into(subscriptions::table)
            .values(&form.into_inner())
            .returning(Subscription::as_returning())
            .get_result(conn)
    })
    .await;

    match res {
        Ok(subscription) => {
            tracing::info!(
                "request {} - New subscriber details have been saved",
                request_id
            );
            HttpResponse::Ok().json(subscription)
        }
        Err(e) => {
            tracing::error!("request {} - Failed to execute query: {:?}", request_id, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[instrument(skip(pool))]
async fn get_subscriptions(
    form: Form<NewSubscription>,
    pool: Data<Pool>,
    request_id: RequestId,
) -> HttpResponse {
    tracing::info!("request {} - Requesting subscriber details...", request_id);
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
            tracing::info!("request {} - Found subscription", request_id);
            HttpResponse::Ok().json(subscription)
        }
        Ok(None) => {
            tracing::info!(
                "request {} - Did not find the requested subscription",
                request_id
            );
            HttpResponse::NotFound().finish()
        }
        Err(e) => {
            tracing::error!("request {} - Failed to execute query: {:?}", request_id, e);
            HttpResponse::InternalServerError().body(e.to_string())
        }
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
    use crate::request::RequestId;

    #[tokio::test]
    async fn health_check_succeeds() {
        let response = health_check(RequestId(uuid::Uuid::new_v4())).await;
        assert!(response.status().is_success())
    }
}
