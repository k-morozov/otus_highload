use actix_web::{HttpRequest, HttpResponse, web};
use futures_util::StreamExt as _;
use tracing::{error, info};

use crate::handlers::UserRegister;
use crate::handlers::handler::Handler;
use crate::model::UserRegisterRequestBody;
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
    info!("obj={:?}", obj);

    let repo = state.ctx.user_repo();

    UserRegister::process(&obj, repo).await.map_err(|e| {
        error!("Failed with error: {}", e);
        actix_web::error::ErrorInternalServerError("internal server error")
    })?;

    Ok(HttpResponse::Ok().body("Ok"))
}
