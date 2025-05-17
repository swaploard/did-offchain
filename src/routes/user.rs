use actix_web::{web, HttpResponse, Responder};
use utoipa::{OpenApi};
use crate::models::user::User;
use crate::services::user_service;

#[utoipa::path(
    get,
    path = "/user",
    responses(
        (status = 200, description = "List users", body = [User])
    )
)]
pub async fn get_users() -> impl Responder {
    let users = user_service::fetch_users().await;
    HttpResponse::Ok().json(users)
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/user", web::get().to(get_users));
}

#[derive(OpenApi)]
#[openapi(
    paths(get_users),
    components(schemas(User)),
    tags((name = "User", description = "User-related endpoints"))
)]
pub struct UserApiDoc;

pub fn get_openapi() -> utoipa::openapi::OpenApi {
    UserApiDoc::openapi()
}
