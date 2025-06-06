use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

use crate::{project_error, project_info};

pub async fn initialize_log_tracing() {
    if let Err(e) = LogTracer::init() {
        project_error!("Failed to set logger: {}", e);
        return;
    }

    let env_filter = if cfg!(debug_assertions) {
        EnvFilter::new("debug,sea_orm=debug")
    } else {
        EnvFilter::new("info,sea_orm=info")
    };

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_target(true)
        .with_ansi(true);

    let subscriber = Registry::default()
        .with(env_filter)
        .with(fmt_layer)
        .with(tracing_error::ErrorLayer::default());

    if let Err(e) = tracing::subscriber::set_global_default(subscriber) {
        project_error!("Failed to set subscriber: {}", e);
        return;
    }

    if cfg!(debug_assertions) {
        project_info!("Log tracing initialized successfully in debug mode");
    } else {
        project_info!("Log tracing initialized successfully in release mode");
    }
}
