use actix_web::{HttpResponse, Responder, get, web};
use tracing::{Level, event};

use super::app_state::AppState;

#[get("/")]
async fn hello(data: web::Data<AppState>) -> impl Responder {
    tracing::info!("call hello");
    event!(Level::INFO, "something happened inside my_span");
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    let msg = format!("message: {} with counter {}", data.app_name, counter);
    HttpResponse::Ok().body(msg)
}
