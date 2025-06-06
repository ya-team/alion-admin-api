#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        routing::get,
        Router,
    };
    use axum_casbin::{
        casbin::{DefaultModel, FileAdapter},
        CasbinAxumLayer,
    };
    use chrono::{Duration, Utc};
    use jsonwebtoken::{encode, EncodingKey, Header};
    use server_constant::definition::Audience;
    use server_core::web::{
        auth::{Claims, User},
        res::Res,
    };
    use server_initialize::{initialize_config, initialize_keys_and_validation};
    use server_middleware::jwt_auth_middleware;
    use tower::{ServiceBuilder, ServiceExt};

    async fn user_info_handler(user: User) -> Res<User> {
        Res::new_data(user)
    }

    #[tokio::test]
    async fn test_user_info_endpoint() {
        let m = DefaultModel::from_file("../../axum-casbin/examples/rbac_with_domains_model.conf")
            .await
            .unwrap();
        let a = FileAdapter::new("../../axum-casbin/examples/rbac_with_domains_policy.csv");

        let casbin_middleware = CasbinAxumLayer::new(m, a).await.unwrap();

        initialize_config("../resources/application.yaml").await;
        initialize_keys_and_validation().await;

        let app = Router::new()
            .route("/pen/1", get(user_info_handler))
            .layer(casbin_middleware)
            .layer(axum::middleware::from_fn(move |req, next| {
                jwt_auth_middleware(req, next, Audience::ManagementPlatform.as_str())
            }));

        let service = ServiceBuilder::new().service(app);

        let token = generate_jwt();

        let request = Request::builder()
            .uri("/pen/1")
            .header("Authorization", format!("Bearer {}", token))
            .body(Body::empty())
            .unwrap();

        let response = service.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        println!("headers is {:?}", response.headers());
        let body_bytes = axum::body::to_bytes(response.into_body(), 1000)
            .await
            .unwrap();
        let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
        println!("body_str is {}", body_str);
    }

    fn generate_jwt() -> String {
        let mut claims = Claims::new(
            "admin".to_string(),
            Audience::ManagementPlatform.as_str().to_string(),
            "alice".to_string(),
            vec!["example_role".to_string()],
            "domain1".to_string(),
            Option::from("example_org".to_string()),
        );
        let now = Utc::now();
        claims.set_exp((now + Duration::seconds(7200)).timestamp() as usize);

        let encoding_key = EncodingKey::from_secret("alion-admin".as_ref());
        encode(&Header::default(), &claims, &encoding_key).unwrap()
    }
}
