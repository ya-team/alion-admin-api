use std::{
    convert::Infallible,
    task::{Context, Poll},
};

use axum::{body::Body, response::Response, routing::get, BoxError, Router};
use axum_casbin::{CasbinAxumLayer, CasbinVals};
use bytes::Bytes;
use casbin::{DefaultModel, FileAdapter};
use futures::future::BoxFuture;
use http::{Request, StatusCode};
use http_body::Body as HttpBody;
use tower::{Layer, Service, ServiceExt};

#[derive(Clone)]
struct FakeAuthLayer;

impl<S> Layer<S> for FakeAuthLayer {
    type Service = FakeAuthMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        FakeAuthMiddleware { inner }
    }
}

#[derive(Clone)]
struct FakeAuthMiddleware<S> {
    inner: S,
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for FakeAuthMiddleware<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>, Error = Infallible>
        + Clone
        + Send
        + 'static,
    S::Future: Send + 'static,
    ReqBody: Send + 'static,
    Infallible: From<<S as Service<Request<ReqBody>>>::Error>,
    ResBody: HttpBody<Data = Bytes> + Send + 'static,
    ResBody::Error: Into<BoxError>,
{
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;
    type Response = S::Response;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<ReqBody>) -> Self::Future {
        let not_ready_inner = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, not_ready_inner);

        Box::pin(async move {
            let vals = CasbinVals {
                subject: vec!["alice".to_string()],
                domain: Option::from(String::from("domain1")),
            };
            req.extensions_mut().insert(vals);
            inner.call(req).await
        })
    }
}

// Handler that immediately returns an empty `200 OK` response.
async fn handler() {}

#[cfg_attr(feature = "runtime-tokio", tokio::test)]
#[cfg_attr(feature = "runtime-async-std", async_std::test)]
async fn test_middleware_domain() {
    let m = DefaultModel::from_file("examples/rbac_with_domains_model.conf")
        .await
        .unwrap();
    let a = FileAdapter::new("examples/rbac_with_domains_policy.csv");

    let casbin_middleware = CasbinAxumLayer::new(m, a).await.unwrap();

    let app = Router::new()
        .route("/pen/1", get(handler))
        .route("/book/1", get(handler))
        .layer(casbin_middleware)
        .layer(FakeAuthLayer);

    let service = tower::ServiceBuilder::new().service(app);

    let resp_pen = service
        .clone()
        .oneshot(Request::builder().uri("/pen/1").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(resp_pen.status(), StatusCode::OK);

    let resp_book = service
        .clone()
        .oneshot(Request::builder().uri("/book/1").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(resp_book.status(), StatusCode::FORBIDDEN);
}
