use actix_web::{get, post, web, HttpResponse, Responder};
use crate::models::user::{User, CreateUserRequest};
use crate::services::user_service;
use crate::errors::user_errors::UserServiceError;
use crate::utils::auth_guard::AuthGuard;
use crate::models::user::UserRole;
use sqlx::PgPool;

#[utoipa::path(
    get,
    path = "/users",
    responses((status = 200, body = [User])),
    security(("bearer_auth" = []))
)]
#[get("/users")]
pub async fn get_users(
    pool: web::Data<PgPool>,
    auth: AuthGuard,
) -> Result<impl Responder, UserServiceError> {
    auth.require_role(UserRole::Admin)
        .map_err(|_| UserServiceError::Unauthorized)?;

    let users = user_service::fetch_users(&pool)
        .await
        .map_err(|e| UserServiceError::Internal(e.to_string()))?;

    Ok(HttpResponse::Ok().json(users))
}


#[utoipa::path(
    post,
    path = "/users",
    request_body = CreateUserRequest,
    responses((status = 201, description = "User created successfully", body = User)),
    security(("bearer_auth" = []))
)]
#[post("/users")]
pub async fn create_user(
    pool: web::Data<PgPool>,
    auth: AuthGuard, // <-- extract AuthGuard
    user: web::Json<CreateUserRequest>,
) -> Result<impl Responder, UserServiceError> {
    // Optional: restrict to admin only
    auth.require_role(UserRole::Admin)
    .map_err(|_| UserServiceError::Unauthorized)?;

    let created_user = user_service::create_user(pool.get_ref(), user.into_inner())
        .await
        .map_err(|e| UserServiceError::Internal(e.to_string()))?;

    Ok(HttpResponse::Created().json(created_user))
}


// #[utoipa::path(
//     put,
//     path = "/user/{id}",
//     request_body = UpdateUserDto,
//     responses((status = 200, body = User))
// )]
// pub async fn update_user(
//     path: web::Path<i32>,
//     user: web::Json<UpdateUserDto>,
// ) -> impl Responder {
//     let updated = user_service::update_user(path.into_inner(), user.into_inner()).await;
//     HttpResponse::Ok().json(updated)
// }

// #[utoipa::path(
//     delete,
//     path = "/user/{id}",
//     responses((status = 204))
// )]
// pub async fn delete_user(path: web::Path<i32>) -> impl Responder {
//     user_service::delete_user(path.into_inner()).await;
//     HttpResponse::NoContent().finish()
// }
