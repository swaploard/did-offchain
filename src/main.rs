mod handlers;
pub mod models;
mod routes;
mod server;
pub mod services;
mod utils;

use actix_web::{App, HttpServer};
use tracing_actix_web::TracingLogger;
use utils::logger::init_logger;
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load .env (optional but recommended)
    if let Err(err) = dotenv::dotenv() {
        eprintln!("Warning: failed to load .env file: {}", err);
    }

    init_logger();
    tracing::info!("🚀 Logger initialized");

    let listener = match server::get_tcp_listener() {
        Ok(l) => l,
        Err(e) => {
            eprintln!("❌ Failed to bind to address: {}", e);
            std::process::exit(1);
        }
    };

    // Read environment
    let is_dev = std::env::var("APP_ENV")
        .map(|val| val == "development")
        .unwrap_or(false);

    // Build OpenAPI only if needed
    let openapi = if is_dev {
        Some(routes::build_openapi())
    } else {
        None
    };

    // Launch server
    HttpServer::new(move || {
        let mut app = App::new()
            .wrap(TracingLogger::default())
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
