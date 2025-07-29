use crate::models::did::{CreateDidRequest, DidDocumentRecord};
use crate::services::did_service;
use crate::utils::auth_guard::AuthGuard;
use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::PgPool;

#[utoipa::path(
    post,
    path = "/did",
    request_body = CreateDidRequest,
    responses((status = 200, body = DidDocumentRecord)),
    security(("bearer_auth" = []))
)]
#[post("/did")]
pub async fn create_did(
    pool: web::Data<PgPool>,
    payload: web::Json<CreateDidRequest>,
) -> impl Responder {
    match did_service::create_did_document(&pool, payload.into_inner()).await {
        Ok(record) => HttpResponse::Ok().json(record),
        Err(e) => {
            eprintln!("❌ Error creating DID document: {e}");
            HttpResponse::InternalServerError().body("Failed to create DID document")
        }
    }
}
