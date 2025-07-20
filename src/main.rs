mod api;
mod middleware;

use std::sync::Mutex;

use actix_web::{App, HttpServer, web};
use actix_web::middleware::from_fn;
use tracing_actix_web::{TracingLogger};

use api::{app_state::AppState, hello::hello};
use middleware::{DomainRootSpanBuilder, RequestID};

fn init_tracing() {
    tracing_subscriber::fmt().with_target(false).init();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_tracing();

    HttpServer::new(|| {
        App::new()
            .wrap(TracingLogger::<DomainRootSpanBuilder>::new())            
            //.wrap(from_fn(add_request_id))
            .wrap(RequestID)
            /*.wrap_fn(|req, srv| {
                //let uri = req.uri().to_string();
                let span = tracing::span!(tracing::Level::INFO, "my span");
                let _guard = span.enter();
                srv.call(req)
            })*/
            .app_data(web::Data::new(AppState {
                app_name: String::from("particular message"),
                counter: Mutex::new(0),
            }))
            .service(hello)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
