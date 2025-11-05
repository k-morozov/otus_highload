mod api;
mod auth;
mod dao;
mod error;
mod handlers;
mod layer;
mod model;
mod repo;
mod state;
mod store;
mod utils;

use actix_web::middleware::from_fn;
use actix_web::{App, HttpServer, web};
use api::user::{user_get, user_login, user_register};
use layer::{DomainRootSpanBuilder, RequestID, jwt_verify};
use state::app_state::AppState;
use tracing::Level;
use tracing_actix_web::TracingLogger;

fn init_tracing() {
    tracing_subscriber::fmt()
        .with_target(true)
        .with_max_level(Level::DEBUG)
        .init();
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();

    let state = web::Data::new(AppState::new().await?);

    HttpServer::new(move || {
        let tracing_logger = TracingLogger::<DomainRootSpanBuilder>::new();

        App::new()
            .wrap(tracing_logger)
            .wrap(RequestID)
            .app_data(state.clone())
            .service(user_register)
            .service(user_login)
            .service(
                web::resource("user/get/{id}")
                    .wrap(from_fn(jwt_verify))
                    .route(web::get().to(user_get)),
            )
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await?;

    Ok(())
}
