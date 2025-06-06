use std::sync::Arc;

use server_config::JwtConfig;
use server_global::{global, Validation};
use tokio::sync::Mutex;

use crate::{project_error, project_info};

pub async fn initialize_keys_and_validation() {
    let jwt_config = match global::get_config::<JwtConfig>().await {
        Some(cfg) => cfg,
        None => {
            project_error!("Failed to load JWT config");
            return;
        },
    };

    let keys = global::Keys::new(jwt_config.jwt_secret.as_bytes());
    if global::KEYS.set(Arc::new(Mutex::new(keys))).is_err() {
        project_error!("Failed to set KEYS");
    }

    let mut validation = Validation::default();
    validation.leeway = 60;
    validation.set_issuer(&[&jwt_config.issuer]);
    if global::VALIDATION
        .set(Arc::new(Mutex::new(validation)))
        .is_err()
    {
        project_error!("Failed to set VALIDATION");
    }

    project_info!("JWT keys and validation initialized");
}
