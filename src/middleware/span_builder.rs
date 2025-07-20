use actix_web::Error;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use tracing::Span;
use tracing_actix_web::{DefaultRootSpanBuilder, RootSpanBuilder};

pub struct DomainRootSpanBuilder;

impl RootSpanBuilder for DomainRootSpanBuilder {
    fn on_request_start(request: &ServiceRequest) -> Span {
        let request_id = request
            .headers()
            .get("x-request-id")
            .unwrap()
            .to_str()
            .unwrap();

        let method = request.method().to_string();
        let uri = request.uri().to_string();
        tracing::info_span!("http_request", request_id, method, uri)
        // tracing_actix_web::root_span!(request, request_id)
    }
    fn on_request_end<B: MessageBody>(span: Span, outcome: &Result<ServiceResponse<B>, Error>) {
        DefaultRootSpanBuilder::on_request_end(span, outcome);
    }
}
