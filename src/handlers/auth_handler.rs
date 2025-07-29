use crate::errors::user_errors::UserServiceError;
use crate::models::auth::{LoginRequest, LogoutRequest, SignupRequest, TokenResponse};
use crate::models::user::UserRole;
use crate::services::auth_service::{authenticate_user, register_user};
use crate::settings::jwt::JWT_CONFIG;
use crate::utils::jwt::{decode_token, issue_tokens};
use actix_web::{post, web, HttpResponse, Responder};
use sqlx::PgPool;
use utoipa::path;
use validator::Validate;

#[utoipa::path(
    post,
    path = "/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = TokenResponse),
        (status = 401, description = "Invalid credentials")
    )
)]
#[post("/login")]
pub async fn login(
    pool: web::Data<PgPool>,
    payload: web::Json<LoginRequest>,
) -> Result<impl Responder, UserServiceError> {
    let data = payload.into_inner();
    let user = authenticate_user(pool.get_ref(), &data.username, &data.password).await?;
    let (access, refresh) =
        issue_tokens(&user.id.to_string(), user.role).map_err(UserServiceError::JwtError)?;

    Ok(HttpResponse::Ok().json(TokenResponse {
        access_token: access,
        refresh_token: refresh,
    }))
}

#[utoipa::path(
    post,
    path = "/auth/signup",
    request_body = SignupRequest,
    responses(
        (status = 201, description = "Signup successful", body = TokenResponse),
        (status = 409, description = "User already exists")
    )
)]
#[post("/signup")]
pub async fn signup(
    pool: web::Data<PgPool>,
    payload: web::Json<SignupRequest>,
) -> Result<impl Responder, UserServiceError> {
    let role = UserRole::User;
    payload
        .validate()
        .map_err(|e| UserServiceError::ValidationError(e.to_string()))?;
    let data = payload.into_inner();
    let user_id = register_user(pool.get_ref(), data).await?;
    let (access, refresh) = issue_tokens(&user_id, role).map_err(UserServiceError::JwtError)?;

    tracing::info!("New user {} signed up", user_id);

    Ok(HttpResponse::Created().json(TokenResponse {
        access_token: access,
        refresh_token: refresh,
    }))
}

#[utoipa::path(
    post,
    path = "/auth/logout",
    request_body = LogoutRequest,
    responses(
        (status = 200, description = "Logged out successfully")
    )
)]
#[post("/logout")]
pub async fn logout(
    _pool: web::Data<PgPool>,
    _payload: web::Json<LogoutRequest>,
) -> impl Responder {
    HttpResponse::Ok().body("Logged out successfully")
}

#[utoipa::path(
    post,
    path = "/auth/refresh",
    request_body = String,
    responses(
        (status = 200, description = "Token refreshed", body = TokenResponse),
        (status = 401, description = "Invalid or expired refresh token")
    )
)]
#[post("/refresh")]
pub async fn refresh_token(body: String) -> impl Responder {
    let token_str = body.trim();
    match decode_token(token_str, &JWT_CONFIG.refresh_secret) {
        Ok(token_data) if token_data.claims.token_type == "refresh" => {
            let subject = token_data.claims.sub;
            let role = token_data.claims.role.clone();
            let (access, refresh) =
                issue_tokens(&subject, role).unwrap_or_else(|_| ("".into(), "".into()));
            if access.is_empty() {
                HttpResponse::InternalServerError().body("Failed to issue new tokens")
            } else {
                HttpResponse::Ok().json(TokenResponse {
                    access_token: access,
                    refresh_token: refresh,
                })
            }
        }
        _ => HttpResponse::Unauthorized().body("Invalid or expired refresh token"),
    }
}
