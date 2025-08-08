mod api;
mod error;
mod handlers;
mod middleware;
mod model;
mod state;
mod store;
mod repo;

use actix_web::{App, HttpServer, web};
use api::user::user_register;
use middleware::{DomainRootSpanBuilder, RequestID};
use state::app_state::AppState;
use tracing_actix_web::TracingLogger;
// use crate::error::ServiceError;

fn init_tracing() {
    tracing_subscriber::fmt().with_target(false).init();
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
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await?;

    Ok(())
}
