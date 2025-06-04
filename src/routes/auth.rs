use actix_web::{web};
use utoipa::{OpenApi};
use crate::handlers::auth_handler;
use crate::models::auth::{LoginRequest, TokenResponse, SignupRequest, LogoutRequest, UserRole};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(auth_handler::login)
            .service(auth_handler::refresh_token)
            .service(auth_handler::signup)
            .service(auth_handler::logout)
    );
}

#[derive(OpenApi)]
#[openapi(
    paths(
        auth_handler::login,
        auth_handler::signup,
        auth_handler::logout,
    ),
    components(
        schemas(LoginRequest, SignupRequest, LogoutRequest, TokenResponse, UserRole)
    ),
)]
pub struct AuthApiDoc;

pub fn get_openapi() -> utoipa::openapi::OpenApi {
    AuthApiDoc::openapi()
}
