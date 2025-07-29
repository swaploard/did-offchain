use crate::handlers::user_handler;
use crate::models::user::User;
use actix_web::web;
use utoipa::OpenApi;

#[utoipa::path(
    get,
    path = "/user",
    responses(
        (status = 200, description = "List users", body = [User])
    )
)]

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(user_handler::get_users)
        .service(user_handler::create_user);
}

#[derive(OpenApi)]
#[openapi(
    paths(user_handler::get_users, user_handler::create_user),
    components(schemas(User))
)]
pub struct UserApiDoc;

pub fn get_openapi() -> utoipa::openapi::OpenApi {
    UserApiDoc::openapi()
}
