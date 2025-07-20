use std::future::{ready, Ready};
use std::pin::Pin;

use actix_web::dev::ServiceResponse;
use actix_web::Error;
use actix_web::middleware::{Next};
use actix_web::{body::MessageBody, http::header::{HeaderName, HeaderValue}};
use actix_web::dev::{Transform, forward_ready, Service, ServiceRequest};
use uuid::Uuid;

#[deprecated]
pub async fn add_request_id(
    mut req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
   let request_id = Uuid::new_v4().to_string();

    req.headers_mut().append(
        HeaderName::from_static("x-request-id"),
        HeaderValue::from_str(request_id.as_str()).unwrap(),
    );

    tracing::info!("get request: {:#?}", req.request());

    let res= next.call(req);
    let mut res = res.await?;

    res.headers_mut().insert(
        HeaderName::from_static("x-request-id"),
        HeaderValue::from_str(&request_id).unwrap(),
    );

    tracing::info!("prepared response: {:#?}", res.response());
    Ok(res)
}

pub struct RequestID;

impl<S, B> Transform<S, ServiceRequest> for RequestID 
where 
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error=Error>,
    S::Future: 'static,
    B: 'static + actix_web::body::MessageBody {
    
        type Response = ServiceResponse<B>;
        type Error = Error;
        type InitError = ();
        type Transform = RequestIDService<S>;
        type Future = Ready<Result<Self::Transform, Self::InitError>>;

        fn new_transform(&self, service: S) -> Self::Future {
            ready(Ok(RequestIDService { service }))
        }
}

pub struct RequestIDService<S> {
    service: S,
}

type LocalBoxFuture<T> = Pin<Box<dyn Future<Output = T> + 'static>>;

impl<S, B> Service<ServiceRequest> for RequestIDService<S> 
where 
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error=Error>,
    S::Future: 'static,
    B: 'static + actix_web::body::MessageBody {

    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let request_id = Uuid::new_v4().to_string();

        req.headers_mut().append(
            HeaderName::from_static("x-request-id"),
            HeaderValue::from_str(request_id.as_str()).unwrap(),
        );

        tracing::info!("get request: {:#?}", req.request());

        let fut = self.service.call(req);
        Box::pin(async move {
            let mut res = fut.await?;
            res.headers_mut().insert(
                HeaderName::from_static("x-request-id"),
                HeaderValue::from_str(&request_id).unwrap(),
            );

            tracing::info!("prepared response: {:#?}", res.response());
            Ok(res)
        })

    }
}
