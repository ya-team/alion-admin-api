use std::sync::Arc;
use std::error::Error;

use server_config::JwtConfig;
use server_global::{global, Validation};
use tokio::sync::Mutex;

pub async fn init_jwt() -> Result<(), Box<dyn Error>> {
    let jwt_config = global::get_config::<JwtConfig>().await
        .ok_or_else(|| Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "JWT config not found")))?;
    
    let keys = global::Keys::new(jwt_config.secret.as_bytes());
    let mut validation = Validation::default();
    validation.leeway = 60;
    validation.set_issuer(&[jwt_config.issuer.clone()]);
    validation.set_audience(&[jwt_config.audience.clone()]);

    global::KEYS.set(Arc::new(Mutex::new(keys)))
        .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to set JWT keys: {}", e))))?;
    
    global::VALIDATION.set(Arc::new(Mutex::new(validation)))
        .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to set JWT validation: {}", e))))?;

    Ok(())
}
