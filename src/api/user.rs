use actix_web::{HttpMessage, HttpRequest, HttpResponse, web};
use futures_util::StreamExt as _;
use tracing::{error, info};
use uuid::Uuid;

use crate::handlers::handler::Handler;
use crate::handlers::{UserGet, UserLogin, UserRegister};
use crate::model::{UserGetRequestBody, UserLoginRequestBody, UserRegisterRequestBody};
use crate::state::app_state::AppState;

const MAX_SIZE: usize = 262_144; // max payload size is 256k

#[actix_web::post("/user/register")]
async fn user_register(
    _request: HttpRequest,
    mut payload: web::Payload,
    state: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        if body.len() + chunk.len() > MAX_SIZE {
            return Err(actix_web::error::ErrorBadRequest("overflow"));
        }
        body.extend(chunk);
    }

    let obj = serde_json::from_slice::<UserRegisterRequestBody>(&body)?;
    info!("model={:?}", obj);

    UserRegister::process(state.ctx.clone(), obj)
        .await
        .map_err(|e| {
            error!("Failed with error: {}", e);
            actix_web::error::ErrorInternalServerError("internal server error")
        })?;

    Ok(HttpResponse::Ok().body("Ok"))
}

#[actix_web::post("/user/login")]
async fn user_login(
    _request: HttpRequest,
    mut payload: web::Payload,
    state: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        if body.len() + chunk.len() > MAX_SIZE {
            return Err(actix_web::error::ErrorBadRequest("overflow"));
        }
        body.extend(chunk);
    }

    let obj = serde_json::from_slice::<UserLoginRequestBody>(&body)?;
    info!("model={:?}", obj);

    let response = UserLogin::process(state.ctx.clone(), obj)
        .await
        .map_err(|e| {
            error!("Failed with error: {}", e);
            actix_web::error::ErrorInternalServerError("internal server error")
        })?;

    Ok(HttpResponse::Ok().json(response))
}

pub async fn user_get(
    _request: HttpRequest,
    state: web::Data<AppState>,
    id: web::Path<String>,
) -> Result<HttpResponse, actix_web::Error> {
    let obj = UserGetRequestBody {
        id: id.into_inner(),
    };

    let response = UserGet::process(state.ctx.clone(), obj)
        .await
        .map_err(|e| {
            error!("Failed with error: {}", e);
            actix_web::error::ErrorInternalServerError("internal server error")
        })?;

    Ok(HttpResponse::Ok().json(response))
}
