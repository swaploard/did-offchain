use actix_web::{web, HttpResponse, Responder};
use crate::models::user::User;
use crate::services::user_service;

#[utoipa::path(
    get,
    path = "/user",
    responses((status = 200, body = [User]))
)]
pub async fn get_users() -> impl Responder {
    let users = user_service::fetch_users().await;
    HttpResponse::Ok().json(users)
}

// #[utoipa::path(
//     post,
//     path = "/user",
//     request_body = CreateUserDto,
//     responses((status = 201, body = User))
// )]
// pub async fn create_user(user: web::Json<CreateUserDto>) -> impl Responder {
//     let created = user_service::create_user(user.into_inner()).await;
//     HttpResponse::Created().json(created)
// }

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
