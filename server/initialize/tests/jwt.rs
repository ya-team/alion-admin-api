use server_core::web::{auth::Claims, jwt::JwtUtils};

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use server_initialize::{initialize_config, init_jwt};
    use tokio::sync::Mutex;

    use super::*;

    fn create_claims(issuer: &str, audience: &str) -> Claims {
        let mut claims = Claims::new(
            "user123".to_string(),
            audience.to_string(),
            "account".to_string(),
            vec!["admin".to_string()],
            "example_domain".to_string(),
            Option::from("example_org".to_string()),
        );
        claims.set_iss(issuer.to_string());
        claims
    }

    static INITIALIZED: Mutex<Option<Arc<()>>> = Mutex::const_new(None);

    async fn init() {
        let mut initialized = INITIALIZED.lock().await;
        if initialized.is_none() {
            initialize_config("../resources/application.yaml").await;
            init_jwt().await.unwrap();
            *initialized = Some(Arc::new(()));
        }
    }

    #[tokio::test]
    async fn test_validate_token_success() {
        init().await;

        let claims = create_claims(
            "git@github.com:ya-team/alion-admin-api.git",
            "audience",
        );
        let token = JwtUtils::generate_token(&claims).await.unwrap();

        let result = JwtUtils::validate_token(&token, "audience").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_validate_token_invalid_audience() {
        init().await;

        let claims = create_claims(
            "git@github.com:ya-team/alion-admin-api.git",
            "invalid_audience",
        );
        let token = JwtUtils::generate_token(&claims).await.unwrap();

        let result = JwtUtils::validate_token(&token, "audience").await;
        assert!(result.is_err());
    }

    // #[tokio::test]
    // async fn test_validate_token_invalid_issuer() {
    //     init().await;
    //
    //     let claims = create_claims("invalid_issuer", "audience");
    //     let token = JwtUtils::generate_token(&claims).await.unwrap();
    //
    //     let result = JwtUtils::validate_token(&token, "audience").await;
    //     assert!(result.is_err());
    // }
    //
    // #[tokio::test]
    // async fn test_validate_token_expired() {
    //     init().await;
    //
    //     let claims =
    //         create_claims("git@github.com:ya-team/alion-admin-api.git", "audience");
    //     let token = JwtUtils::generate_token(&claims).await.unwrap();
    //
    //     let result = JwtUtils::validate_token(&token, "audience").await;
    //     assert!(result.is_err());
    // }

    #[tokio::test]
    async fn test_validate_token_invalid_signature() {
        init().await;

        let claims = create_claims(
            "git@github.com:ya-team/alion-admin-api.git",
            "audience",
        );
        let token = JwtUtils::generate_token(&claims).await.unwrap();

        let mut invalid_token = token.clone();
        let len = invalid_token.len();
        invalid_token.replace_range((len - 1)..len, "X");

        let result = JwtUtils::validate_token(&invalid_token, "audience").await;
        assert!(result.is_err());
    }
}
