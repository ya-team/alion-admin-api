/**
 * Axum middleware for Casbin authorization
 * 
 * This module provides middleware components for integrating Casbin authorization
 * with Axum web applications. It includes the necessary structures and implementations
 * for enforcing access control policies on HTTP requests.
 */

use std::{
    convert::Infallible,
    ops::{Deref, DerefMut},
    sync::Arc,
    task::{Context, Poll},
};

#[cfg(feature = "runtime-async-std")]
use async_std::sync::RwLock;
use axum::{body, response::Response, BoxError};
use bytes::Bytes;
use casbin::{
    prelude::{TryIntoAdapter, TryIntoModel},
    CachedEnforcer, CoreApi, Result as CasbinResult,
};
use futures::future::BoxFuture;
use http::{Request, StatusCode};
use http_body::Body as HttpBody;
use http_body_util::Full;
#[cfg(feature = "runtime-tokio")]
use tokio::sync::RwLock;
use tower::{Layer, Service};

/**
 * Values used for Casbin policy enforcement
 * 
 * This struct contains the subject (user) and optional domain information
 * that will be used to evaluate access control policies.
 */
#[derive(Clone)]
pub struct CasbinVals {
    /** The subject(s) to check permissions for */
    pub subject: Vec<String>,
    /** Optional domain for domain-specific policies */
    pub domain: Option<String>,
}

/**
 * Layer for Casbin authorization in Axum
 * 
 * This struct implements the Tower Layer trait to provide Casbin authorization
 * as middleware in Axum applications.
 */
#[derive(Clone)]
pub struct CasbinAxumLayer {
    /** The Casbin enforcer wrapped in a thread-safe reference */
    enforcer: Arc<RwLock<CachedEnforcer>>,
}

impl CasbinAxumLayer {
    /**
     * Creates a new CasbinAxumLayer
     * 
     * # Arguments
     * * `m` - The Casbin model
     * * `a` - The Casbin adapter
     * 
     * # Returns
     * * `CasbinResult<Self>` - The created layer or an error
     */
    pub async fn new<M: TryIntoModel, A: TryIntoAdapter>(m: M, a: A) -> CasbinResult<Self> {
        let enforcer: CachedEnforcer = CachedEnforcer::new(m, a).await?;
        Ok(CasbinAxumLayer {
            enforcer: Arc::new(RwLock::new(enforcer)),
        })
    }

    /**
     * Gets a clone of the enforcer
     * 
     * # Returns
     * * `Arc<RwLock<CachedEnforcer>>` - A clone of the enforcer
     */
    pub fn get_enforcer(&mut self) -> Arc<RwLock<CachedEnforcer>> {
        self.enforcer.clone()
    }

    /**
     * Creates a new layer with the specified enforcer
     * 
     * # Arguments
     * * `e` - The enforcer to use
     * 
     * # Returns
     * * `CasbinAxumLayer` - A new layer with the specified enforcer
     */
    pub fn set_enforcer(e: Arc<RwLock<CachedEnforcer>>) -> CasbinAxumLayer {
        CasbinAxumLayer { enforcer: e }
    }
}

impl<S> Layer<S> for CasbinAxumLayer {
    type Service = CasbinAxumMiddleware<S>;

    /**
     * Creates a new middleware service
     * 
     * # Arguments
     * * `inner` - The inner service to wrap
     * 
     * # Returns
     * * `Self::Service` - The wrapped service
     */
    fn layer(&self, inner: S) -> Self::Service {
        CasbinAxumMiddleware {
            enforcer: self.enforcer.clone(),
            inner,
        }
    }
}

impl Deref for CasbinAxumLayer {
    type Target = Arc<RwLock<CachedEnforcer>>;

    fn deref(&self) -> &Self::Target {
        &self.enforcer
    }
}

impl DerefMut for CasbinAxumLayer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.enforcer
    }
}

/**
 * Middleware for Casbin authorization in Axum
 * 
 * This struct implements the Tower Service trait to provide Casbin authorization
 * as middleware in Axum applications.
 */
#[derive(Clone)]
pub struct CasbinAxumMiddleware<S> {
    /** The inner service being wrapped */
    inner: S,
    /** The Casbin enforcer wrapped in a thread-safe reference */
    enforcer: Arc<RwLock<CachedEnforcer>>,
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for CasbinAxumMiddleware<S>
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
    type Error = Infallible;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;
    type Response = Response;

    /**
     * Checks if the service is ready to process requests
     * 
     * # Arguments
     * * `cx` - The task context
     * 
     * # Returns
     * * `Poll<Result<(), Self::Error>>` - Whether the service is ready
     */
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    /**
     * Processes a request through the middleware
     * 
     * This function:
     * 1. Extracts the path and method from the request
     * 2. Gets the subject and domain from the request extensions
     * 3. Enforces the Casbin policy
     * 4. Returns appropriate responses based on the policy decision
     * 
     * # Arguments
     * * `req` - The incoming request
     * 
     * # Returns
     * * `Self::Future` - A future that resolves to the response
     */
    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        let cloned_enforcer = self.enforcer.clone();
        let not_ready_inner = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, not_ready_inner);

        Box::pin(async move {
            let path = req.uri().path().to_string();
            let action = req.method().as_str().to_string();
            let option_vals = req.extensions().get::<CasbinVals>().map(|x| x.to_owned());
            let vals = match option_vals {
                Some(value) => value,
                None => {
                    return Ok(Response::builder()
                        .status(StatusCode::UNAUTHORIZED)
                        .body(body::Body::new(Full::from("No authentication token was provided. Please ensure your request includes a valid token.")))
                        .unwrap());
                },
            };

            let subject = vals.subject.clone();

            if !vals.subject.is_empty() {
                if let Some(domain) = vals.domain {
                    let mut lock = cloned_enforcer.write().await;
                    let mut authorized = false;
                    let mut enforcement_error = false;

                    for sub in subject.iter() {
                        match lock.enforce_mut(vec![
                            sub.clone(),
                            domain.clone(),
                            path.clone(),
                            action.clone(),
                        ]) {
                            Ok(true) => {
                                authorized = true;
                                break;
                            },
                            Ok(false) => continue,
                            Err(_) => {
                                enforcement_error = true;
                                break;
                            },
                        }
                    }

                    drop(lock);

                    if enforcement_error {
                        Ok(Response::builder()
                                .status(StatusCode::BAD_GATEWAY)
                                .body(body::Body::new(Full::from("We encountered an unexpected error while processing your request. Our team has been notified, and we are investigating the issue.")))
                                .unwrap())
                    } else if authorized {
                        Ok(inner.call(req).await?.map(body::Body::new))
                    } else {
                        Ok(Response::builder()
                        .status(StatusCode::FORBIDDEN)
                        .body(body::Body::new(Full::from("You do not have the necessary permissions to access this resource. Please contact support if you believe this is an error.")))
                        .unwrap())
                    }
                } else {
                    let mut lock = cloned_enforcer.write().await;
                    let mut authorized = false;
                    let mut enforcement_error = false;

                    for sub in subject.iter() {
                        match lock.enforce_mut(vec![sub.clone(), path.clone(), action.clone()]) {
                            Ok(true) => {
                                authorized = true;
                                break;
                            },
                            Ok(false) => continue,
                            Err(_) => {
                                enforcement_error = true;
                                break;
                            },
                        }
                    }

                    drop(lock);

                    if enforcement_error {
                        Ok(Response::builder()
                        .status(StatusCode::BAD_GATEWAY)
                        .body(body::Body::new(Full::from("We encountered an unexpected error while processing your request. Our team has been notified, and we are investigating the issue.")))
                        .unwrap())
                    } else if authorized {
                        Ok(inner.call(req).await?.map(body::Body::new))
                    } else {
                        Ok(Response::builder()
                                .status(StatusCode::FORBIDDEN)
                                .body(body::Body::new(Full::from("You do not have the necessary permissions to access this resource. Please contact support if you believe this is an error.")))
                                .unwrap())
                    }
                }
            } else {
                Ok(Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body(body::Body::new(Full::from(
                        "No token provided or invalid token type",
                    )))
                    .unwrap())
            }
        })
    }
}
