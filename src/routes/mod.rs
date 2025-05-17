pub mod user;

use utoipa::openapi::OpenApi;

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    user::configure(cfg);
}

pub fn build_openapi() -> OpenApi {
    let mut specs = vec![
        user::get_openapi(),
    ];

    let mut merged = specs.remove(0);
    for spec in specs {
        merged.merge(spec);
    }

    merged
}
