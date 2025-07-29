mod errors;
mod handlers;
pub mod models;
mod routes;
mod server;
pub mod services;
pub mod settings;
mod utils;

use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use tracing_actix_web::TracingLogger;
use utils::logger::init_logger;
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    init_logger();
    tracing::info!("🚀 Logger initialized");

    // Setup database
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("❌ Failed to connect to PostgreSQL");

    tracing::info!("✅ Connected to PostgreSQL database");

    // Setup listener
    let listener = server::get_tcp_listener().expect("❌ Failed to bind TCP listener");

    // Swagger only for dev
    let is_dev = std::env::var("APP_ENV")
        .map(|val| val == "development")
        .unwrap_or(false);
    let openapi = if is_dev {
        Some(routes::build_openapi())
    } else {
        None
    };

    // Start HTTP server
    HttpServer::new(move || {
        let mut app = App::new()
            .wrap(TracingLogger::default())
            .app_data(web::Data::new(pool.clone())) // ✅ inject PgPool
            .configure(routes::configure);

        if let Some(ref doc) = openapi {
            app =
                app.service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/openapi.json", doc.clone()));
        }

        app
    })
    .listen(listener)?
    .run()
    .await
}
