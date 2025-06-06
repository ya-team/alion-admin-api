use std::{error::Error, fmt};

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Header, TokenData};
use server_config::JwtConfig;
use server_global::global;
use ulid::Ulid;

use crate::web::auth::Claims;

// pub static KEYS: Lazy<Arc<Mutex<Keys>>> = Lazy::new(|| {
//     let config = global::get_config::<JwtConfig>()
//         .expect("[alion-admin] >>>>>> [server-core] Failed to load JWT
// config");     Arc::new(Mutex::new(Keys::new(config.jwt_secret.as_bytes())))
// });
//
// pub static VALIDATION: Lazy<Arc<Mutex<Validation>>> = Lazy::new(|| {
//     let config = global::get_config::<JwtConfig>()
//         .expect("[alion-admin] >>>>>> [server-core] Failed to load JWT
// config");     let mut validation = Validation::default();
//     validation.leeway = 60;
//     validation.set_issuer(&[config.issuer.clone()]);
//     Arc::new(Mutex::new(validation))
// });

#[derive(Debug)]
pub enum JwtError {
    KeysNotInitialized,
    ValidationNotInitialized,
    TokenCreationError(String),
    TokenValidationError(String),
}

impl fmt::Display for JwtError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JwtError::KeysNotInitialized => write!(f, "Keys not initialized"),
            JwtError::ValidationNotInitialized => write!(f, "Validation not initialized"),
            JwtError::TokenCreationError(err) => write!(f, "Token creation error: {}", err),
            JwtError::TokenValidationError(err) => write!(f, "Token validation error: {}", err),
        }
    }
}

impl Error for JwtError {}

pub struct JwtUtils;

impl JwtUtils {
    pub async fn generate_token(claims: &Claims) -> Result<String, JwtError> {
        let keys_arc = global::KEYS.get().ok_or(JwtError::KeysNotInitialized)?;

        let keys = keys_arc.lock().await;

        let mut claims_clone = claims.clone();

        let now = Utc::now();
        let timestamp = now.timestamp() as usize;
        let jwt_config = global::get_config::<JwtConfig>().await.unwrap();
        claims_clone.set_exp((now + Duration::seconds(jwt_config.expire)).timestamp() as usize);
        claims_clone.set_iss(jwt_config.issuer.to_string());
        claims_clone.set_iat(timestamp);
        claims_clone.set_nbf(timestamp);
        claims_clone.set_jti(Ulid::new().to_string());

        let token = encode(&Header::default(), &claims_clone, &keys.encoding)
            .map_err(|e| JwtError::TokenCreationError(e.to_string()));

        if let Ok(ref tok) = token {
            global::send_string_event(tok.clone());
        }

        token
    }

    pub async fn validate_token(
        token: &str,
        audience: &str,
    ) -> Result<TokenData<Claims>, JwtError> {
        let keys_arc = global::KEYS.get().ok_or(JwtError::KeysNotInitialized)?;

        let keys = keys_arc.lock().await;
        let validation_arc = global::VALIDATION
            .get()
            .ok_or(JwtError::ValidationNotInitialized)?;
        let validation = validation_arc.lock().await;

        let mut validation_clone = validation.clone();
        validation_clone.set_audience(&[audience.to_string()]);
        decode::<Claims>(token, &keys.decoding, &validation_clone)
            .map_err(|e| JwtError::TokenValidationError(e.to_string()))
    }
}
