use actix_web::{get, post, web, HttpResponse, Responder};
use crate::models::user::{User, CreateUserRequest};
use crate::services::user_service;
use sqlx::PgPool;

#[utoipa::path(
    get,
    path = "/users",
    responses((status = 200, body = [User]))
)]

#[get("/users")]
pub async fn get_users() -> impl Responder {
    let users = user_service::fetch_users().await;
    println!("Fetched users: {:?}", users);
    HttpResponse::Ok().json(users)
}

#[utoipa::path(
    post,
    path = "/users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created successfully", body = User)
    )
)]

#[post("/users")]
pub async fn create_user(
    pool: web::Data<PgPool>,
    user: web::Json<CreateUserRequest>
) -> impl Responder {
    match user_service::create_user(pool.get_ref(), user.into_inner()).await {
        Ok(user) => HttpResponse::Created().json(user),
        Err(e) => {
            eprintln!("❌ Failed to create user: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to create user")
        }
    }
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
