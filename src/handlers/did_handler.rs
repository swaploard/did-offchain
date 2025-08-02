use crate::models::did::{CreateDidRequest, DidDocumentRecord};
use crate::services::did_service;
use crate::utils::auth_guard::AuthGuard;
use actix_web::{get, post, web, Error, HttpResponse, Responder};
use deadpool_redis::{redis::AsyncCommands, Pool as RedisPool};
use serde::Deserialize;
use sqlx::PgPool;
use utoipa::ToSchema;

#[utoipa::path(
    post,
    path = "/did",
    request_body = CreateDidRequest,
     responses(
        (status = 200, description = "DID document created successfully", body = DidDocumentRecord),
        (status = 500, description = "Failed to create DID document")
    ),
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

#[derive(Debug, Deserialize, ToSchema)]
pub struct VerifyJwtRequest {
    pub jwt: String,
}

#[utoipa::path(
    post,
    path = "/did/verify",
    request_body = VerifyJwtRequest,
    responses(
        (status = 200, description = "JWT verified successfully"),
        (status = 401, description = "Invalid JWT"),
        (status = 500, description = "Internal server error")
    )
)]
#[post("/did/verify")]
pub async fn verify_did_auth(
    payload: web::Json<VerifyJwtRequest>,
    pg_pool: web::Data<PgPool>,
    redis_pool: web::Data<RedisPool>,
) -> Result<HttpResponse, Error> {
    println!("🔍 Raw JWT from request: {:?}", payload.jwt);
    match did_service::verify_did_jwt(&payload.jwt, pg_pool.get_ref(), redis_pool.get_ref()).await {
        Ok(()) => Ok(HttpResponse::Ok().finish()),
        Err(err) => {
            eprintln!("❌ JWT verification error: {err}");
            Ok(HttpResponse::Unauthorized().body("Invalid or expired token"))
        }
    }
}
