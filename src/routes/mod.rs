pub mod auth;
pub mod did;
pub mod user;

use actix_web::web::ServiceConfig;
use utoipa::openapi::{security, ComponentsBuilder, OpenApi, OpenApiBuilder};

pub fn configure(cfg: &mut ServiceConfig) {
    user::configure(cfg);
    auth::configure(cfg);
    did::configure(cfg);
}

pub fn build_openapi() -> OpenApi {
    let mut specs = vec![user::get_openapi(), auth::get_openapi(), did::get_openapi()];

    let mut merged = specs.remove(0);
    for spec in specs {
        merged.merge(spec);
    }

    let mut components_builder = ComponentsBuilder::new();

    if let Some(components) = merged.components.clone() {
        for (schema_name, schema) in components.schemas {
            components_builder = components_builder.schema(schema_name, schema);
        }
    }

    components_builder = components_builder.security_scheme(
        "bearer_auth",
        security::SecurityScheme::Http(
            security::HttpBuilder::new()
                .scheme(security::HttpAuthScheme::Bearer)
                .bearer_format("JWT")
                .build(),
        ),
    );

    let components = components_builder.build();

    let security = Some(vec![security::SecurityRequirement::new(
        "bearer_auth",
        Vec::<String>::new(),
    )]);

    OpenApiBuilder::from(merged)
        .components(Some(components))
        .security(security)
        .build()
}
