use std::net::TcpListener;

use actix_web::{
    dev::Server,
    http::StatusCode,
    web::{self, Form},
    App, HttpResponse, HttpServer, Responder,
};
use serde::Deserialize;

mod db;
mod models;
mod schema;

async fn health_check() -> HttpResponse<&'static str> {
    HttpResponse::with_body(StatusCode::OK, "OK")
}

#[derive(Deserialize)]
struct SubscriptionsForm {
    name: String,
    email: String,
}

async fn subscriptions(form: Form<SubscriptionsForm>) -> impl Responder {
    println!("name: {}, email: {}", form.name, form.email);
    "OK"
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/healthz", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscriptions))
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
