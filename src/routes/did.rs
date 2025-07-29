use crate::handlers::did_handler;
use crate::models::did::DID;
use actix_web::web;
use utoipa::OpenApi;

#[utoipa::path(
    get,
    path = "/did",
    responses(
        (status = 200, description = "did", body = [DID])
    )
)]

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(did_handler::create_did);
}

#[derive(OpenApi)]
#[openapi(paths(did_handler::create_did), components(schemas(DID)))]
pub struct DIDApiDoc;

pub fn get_openapi() -> utoipa::openapi::OpenApi {
    DIDApiDoc::openapi()
}
