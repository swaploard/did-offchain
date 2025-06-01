use actix_web::{web};
use utoipa::{OpenApi};
use crate::models::user::User;
use crate::handlers::user_handler;

#[utoipa::path(
    get,
    path = "/user",
    responses(
        (status = 200, description = "List users", body = [User])
    )
)]

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/user", web::get().to(user_handler::get_users));
}

#[derive(OpenApi)]
#[openapi(
    paths(user_handler::get_users),
    components(schemas(User)),
    tags((name = "User", description = "User-related endpoints"))
)]
pub struct UserApiDoc;

pub fn get_openapi() -> utoipa::openapi::OpenApi {
    UserApiDoc::openapi()
}
