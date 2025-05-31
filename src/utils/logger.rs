use tracing_subscriber::{fmt, EnvFilter};
use tracing::Subscriber;
use std::boxed::Box;

pub fn init_logger() {
    let is_dev = std::env::var("APP_ENV")
        .map(|val| val == "development")
        .unwrap_or(false);

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    let subscriber: Box<dyn Subscriber + Send + Sync> = if is_dev {
        Box::new(
            fmt::Subscriber::builder()
                .with_env_filter(env_filter)
                .pretty()
                .with_level(true)
                .finish()
        )
    } else {
        Box::new(
            fmt::Subscriber::builder()
                .with_env_filter(env_filter)
                .compact()
                .with_level(true)
                .finish()
        )
    };

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global logger");
}
