use actix_web::Error;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::middleware::Next;

use crate::auth::{SECRET_JWT_TOKEN, verify_jwt_token};

const BEARER_VALUE: &'static str = "Bearer ";

pub async fn jwt_verify(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let token = req
        .headers()
        .get(actix_web::http::header::AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_header| {
            if auth_header.starts_with(BEARER_VALUE) {
                Some(auth_header[BEARER_VALUE.len()..].to_owned())
            } else {
                None
            }
        })
        .ok_or(actix_web::error::ErrorUnauthorized("no token"))?;

    tracing::debug!("jwt_verify: get token: {token:?}");

    match verify_jwt_token(token.as_str(), SECRET_JWT_TOKEN) {
        Ok(claims) => {
            tracing::debug!("token for user_id={}", claims.sub);
        }
        Err(_) => {
            return Err(actix_web::error::ErrorUnauthorized("invalid token"));
        }
    }

    let res = next.call(req);
    let res = res.await?;

    Ok(res)
}
