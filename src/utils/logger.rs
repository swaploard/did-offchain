use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter, Layer, Registry};

pub fn init_logger() {
    let is_dev = std::env::var("APP_ENV")
        .map(|val| val == "development")
        .unwrap_or(false);

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let fmt_layer: Box<dyn Layer<_> + Send + Sync> = if is_dev {
        Box::new(
            fmt::layer()
                .pretty()
                .with_level(true)
                .with_target(true)
                .with_span_events(fmt::format::FmtSpan::ENTER | fmt::format::FmtSpan::EXIT),
        )
    } else {
        Box::new(
            fmt::layer()
                .compact()
                .with_level(true)
                .with_target(true)
                .with_span_events(fmt::format::FmtSpan::ENTER | fmt::format::FmtSpan::EXIT),
        )
    };

    let subscriber = Registry::default().with(env_filter).with(fmt_layer);

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set global logger");
}
