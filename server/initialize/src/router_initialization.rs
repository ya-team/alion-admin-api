/**
 * 路由初始化模块
 * 
 * 本模块负责初始化和管理HTTP路由，包括：
 * - 初始化管理后台路由
 * - 配置路由中间件
 * - 管理路由权限
 * - 处理API密钥验证
 */

use std::sync::Arc;
use std::error::Error;
use tokio::sync::mpsc;
use std::time::Duration;

use axum::{body::Body, http::StatusCode, response::IntoResponse, Extension, Router};
use axum_casbin::CasbinAxumLayer;
use chrono::Local;
use http::Request;
use sea_orm::DatabaseConnection;
use server_config::Config;
use server_constant::definition::Audience;
use server_core::sign::{
    api_key_middleware, protect_route, ApiKeySource, ApiKeyValidation, ComplexApiKeyConfig,
    SimpleApiKeyConfig, ValidatorType,
};
use server_core::web::{RequestId, RequestIdLayer};
use server_global::global::{clear_routes, get_collected_routes, get_config};
use server_middleware::jwt_auth_middleware;
use server_router::admin::{
    SysAccessKeyRouter, SysAuthenticationRouter, SysDomainRouter, SysEndpointRouter,
    SysLoginLogRouter, SysMenuRouter, SysOperationLogRouter, SysOrganizationRouter, SysRoleRouter,
    SysSandboxRouter, SysUserRouter,
};
use server_service::{
    admin::{
        SysAccessKeyService, SysAuthService, SysAuthorizationService, SysDomainService,
        SysEndpointService, SysLoginLogService, SysMenuService, SysOperationLogService,
        SysOrganizationService, SysRoleService, SysUserService, TEndpointService,
    },
    SysEndpoint,
};
use tower_http::trace::TraceLayer;
use tracing::info_span;

use crate::{initialize_casbin, project_error, project_info, db_initialization};

/**
 * 服务类型枚举
 * 
 * 用于表示路由服务类型：
 * - None: 无服务
 * - Single: 单个服务实例
 */
#[derive(Clone)]
pub enum Services<T: Send + Sync + 'static> {
    None(std::marker::PhantomData<T>),
    Single(Arc<T>),
}

/**
 * 应用路由中间件
 * 
 * # 参数
 * - router: 原始路由
 * - services: 服务实例
 * - need_casbin: 是否需要Casbin权限控制
 * - need_auth: 是否需要JWT认证
 * - api_validation: API密钥验证配置
 * - casbin: Casbin层配置
 * - audience: 认证受众
 * 
 * # 返回
 * 返回配置了所有必要中间件的路由
 */
async fn apply_layers<T: Send + Sync + 'static>(
    router: Router,
    services: Services<T>,
    need_casbin: bool,
    need_auth: bool,
    api_validation: Option<ApiKeyValidation>,
    casbin: Option<CasbinAxumLayer>,
    audience: Audience,
) -> Router {
    let mut router = match services {
        Services::None(_) => router,
        Services::Single(service) => router.layer(Extension(service)),
    };

    router = router
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
                let request_id = request
                    .extensions()
                    .get::<RequestId>()
                    .map(ToString::to_string)
                    .unwrap_or_else(|| "unknown".into());
                info_span!(
                    "[alion-admin] >>>>>> request",
                    id = %request_id,
                    method = %request.method(),
                    uri = %request.uri(),
                )
            }),
        )
        .layer(RequestIdLayer);

    if need_casbin {
        if let Some(casbin) = casbin {
            router = router.layer(Extension(casbin.clone())).layer(casbin);
        }
    }

    if let Some(validation) = api_validation {
        router = router.layer(axum::middleware::from_fn(move |req, next| {
            api_key_middleware(validation.clone(), req, next)
        }));
    }

    if need_auth {
        router = router.layer(axum::middleware::from_fn(move |req, next| {
            jwt_auth_middleware(req, next, audience.as_str())
        }));
    }

    router
}

/**
 * 初始化管理后台路由
 * 
 * 配置并初始化所有管理后台相关的路由，
 * 包括认证、授权、用户管理等功能。
 * 
 * # 返回
 * 返回配置完整的路由实例
 */
pub async fn initialize_admin_router() -> Router {
    clear_routes().await;
    project_info!("Initializing admin router");

    let app_config = get_config::<Config>().await.unwrap();
    let db = db_initialization::init_primary_connection().await
        .expect("Failed to connect to database");
    
    let db_url = app_config.database.url.clone();
    
    let casbin_layer = initialize_casbin(
        "server/resources/rbac_model.conf",
        &db_url,
    )
    .await
    .unwrap();

    // 初始化验证器
    // 根据是否配置了 Redis 来选择 nonce 存储实现
    let nonce_store_factory =
        if let Some(_) = crate::redis_initialization::get_primary_redis().await {
            // 如果 Redis 可用，使用 Redis 作为 nonce 存储
            project_info!("Using Redis for nonce storage");
            match redis::Client::open("redis://127.0.0.1/") {
                Ok(redis_client) => {
                    server_core::sign::create_redis_nonce_store_factory(
                        redis_client,
                        Duration::from_secs(300) // 5 minutes nonce expiration
                    )
                },
                Err(e) => {
                    project_error!("Failed to create Redis client: {}", e);
                    project_info!("Falling back to memory storage");
                    server_core::sign::create_memory_nonce_store_factory()
                }
            }
        } else {
            // 否则使用内存存储
            project_info!("Using memory for nonce storage");
            server_core::sign::create_memory_nonce_store_factory()
        };

    server_core::sign::init_validators_with_nonce_store(None, nonce_store_factory.clone()).await;

    let simple_validation = {
        let validator = server_core::sign::get_simple_validator().await;
        server_core::sign::add_key(ValidatorType::Simple, "test-api-key", None).await;
        ApiKeyValidation::Simple(
            validator,
            SimpleApiKeyConfig {
                source: ApiKeySource::Header,
                key_name: "x-api-key".to_string(),
            },
        )
    };

    let complex_validation = {
        let validator = server_core::sign::get_complex_validator().await;
        server_core::sign::add_key(
            ValidatorType::Complex,
            "test-access-key",
            Some("test-secret-key"),
        )
        .await;
        ApiKeyValidation::Complex(
            validator,
            ComplexApiKeyConfig {
                key_name: "AccessKeyId".to_string(),
                timestamp_name: "t".to_string(),
                nonce_name: "n".to_string(),
                signature_name: "sign".to_string(),
            },
        )
    };

    // 保护路由
    protect_route("/sandbox/simple-api-key");
    protect_route("/sandbox/complex-api-key");

    let audience = Audience::ManagementPlatform;
    let casbin = Some(casbin_layer);
    let mut app = Router::new();

    macro_rules! merge_router {
        ($router:expr, None, $need_casbin:expr, $need_auth:expr, $api_validation:expr) => {
            app = app.merge(
                apply_layers(
                    $router,
                    Services::None(std::marker::PhantomData::<()>),
                    $need_casbin,
                    $need_auth,
                    $api_validation,
                    casbin.clone(),
                    audience,
                )
                .await,
            );
        };
        ($router:expr, $service:expr, $need_casbin:expr, $need_auth:expr, $api_validation:expr) => {
            app = app.merge(
                apply_layers(
                    $router,
                    Services::Single(Arc::new($service)),
                    $need_casbin,
                    $need_auth,
                    $api_validation,
                    casbin.clone(),
                    audience,
                )
                .await,
            );
        };
    }

    merge_router!(
        SysAuthenticationRouter::init_authentication_router().await,
        SysAuthService,
        false,
        false,
        None
    );

    let auth_service = Arc::new(SysAuthService);
    let auth_router = SysAuthenticationRouter::init_authorization_router()
        .await
        .layer(Extension(auth_service.clone()))
        .layer(Extension(
            Arc::new(SysAuthorizationService::new(db.clone())) as Arc<SysAuthorizationService>
        ));

    let auth_router = apply_layers(
        auth_router,
        Services::None(std::marker::PhantomData::<()>),
        true,
        true,
        None,
        casbin.clone(),
        audience,
    )
    .await;

    app = app.merge(auth_router);

    merge_router!(
        SysAuthenticationRouter::init_protected_router().await,
        SysAuthService,
        false,
        true,
        None
    );

    merge_router!(
        SysMenuRouter::init_menu_router().await,
        SysMenuService,
        false,
        false,
        None
    );

    merge_router!(
        SysMenuRouter::init_protected_menu_router().await,
        SysMenuService,
        true,
        true,
        None
    );

    merge_router!(
        SysUserRouter::init_user_router().await,
        SysUserService,
        true,
        true,
        None
    );
    merge_router!(
        SysDomainRouter::init_domain_router().await,
        SysDomainService,
        true,
        true,
        None
    );
    merge_router!(
        SysRoleRouter::init_role_router().await,
        SysRoleService,
        true,
        true,
        None
    );
    merge_router!(
        SysEndpointRouter::init_endpoint_router().await,
        SysEndpointService::new(db.clone()),
        true,
        true,
        None
    );
    merge_router!(
        SysAccessKeyRouter::init_access_key_router().await,
        SysAccessKeyService,
        true,
        true,
        None
    );
    merge_router!(
        SysLoginLogRouter::init_login_log_router().await,
        SysLoginLogService,
        true,
        true,
        None
    );
    merge_router!(
        SysOperationLogRouter::init_operation_log_router().await,
        SysOperationLogService,
        true,
        true,
        None
    );

    merge_router!(
        SysOrganizationRouter::init_organization_router().await,
        SysOrganizationService,
        false,
        false,
        None
    );

    // sandbox
    merge_router!(
        SysSandboxRouter::init_simple_sandbox_router().await,
        None,
        false,
        false,
        Some(simple_validation)
    );
    merge_router!(
        SysSandboxRouter::init_complex_sandbox_router().await,
        None,
        false,
        false,
        Some(complex_validation)
    );

    app = app.fallback(handler_404);

    process_collected_routes(db.clone()).await;
    project_info!("Admin router initialization completed");

    app
}

/**
 * 404处理器
 * 
 * 处理未找到的路由请求
 * 
 * # 返回
 * 返回404状态码和提示信息
 */
async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}

/**
 * 处理收集的路由
 * 
 * 将收集到的路由信息同步到数据库
 * 
 * # 参数
 * - db: 数据库连接
 */
async fn process_collected_routes(db: DatabaseConnection) {
    let routes = get_collected_routes().await;
    let endpoints: Vec<SysEndpoint> = routes
        .into_iter()
        .map(|route| {
            let resource = route.path.split('/').nth(1).unwrap_or("").to_string();
            SysEndpoint {
                id: generate_id(&route.path, &route.method.to_string()),
                path: route.path.clone(),
                method: route.method.to_string(),
                action: "rw".to_string(),
                resource,
                controller: route.service_name,
                summary: Some(route.summary),
                created_at: Local::now().naive_local(),
                updated_at: None,
            }
        })
        .collect();

    let endpoint_service = SysEndpointService::new(db.clone());
    match endpoint_service.sync_endpoints(endpoints).await {
        Ok(_) => {
            project_info!("Endpoints synced successfully")
        },
        Err(e) => {
            project_error!("Failed to sync endpoints: {:?}", e)
        },
    }
}

/**
 * 生成路由ID
 * 
 * 根据路径和方法生成唯一的路由ID
 * 
 * # 参数
 * - path: 路由路径
 * - method: HTTP方法
 * 
 * # 返回
 * 返回生成的唯一ID
 */
fn generate_id(path: &str, method: &str) -> String {
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };

    let mut hasher = DefaultHasher::new();
    format!("{}{}", path, method).hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

/**
 * 初始化路由
 * 
 * 初始化基本路由配置
 * 
 * # 参数
 * - app_config: 应用配置
 * 
 * # 返回
 * - 成功：返回路由实例
 * - 失败：返回错误信息
 */
#[allow(dead_code)]
pub async fn init_router(_app_config: &Config) -> Result<Router, Box<dyn Error>> {
    #[allow(unused_variables)]
    let db = db_initialization::init_primary_connection().await?;
    #[allow(unused_variables)]
    let (event_sender, _event_receiver) = mpsc::unbounded_channel::<Box<dyn std::any::Any + Send>>();
    #[allow(unused_variables)]
    let auth_service = Arc::new(SysAuthService);

    // Initialize router
    let router = Router::new()
        .merge(SysAuthenticationRouter::init_authentication_router().await)
        .merge(SysAuthenticationRouter::init_authorization_router().await.layer(Extension(auth_service.clone())))
        .merge(SysAuthenticationRouter::init_protected_router().await)
        .merge(SysMenuRouter::init_menu_router().await)
        .merge(SysMenuRouter::init_protected_menu_router().await)
        .merge(SysUserRouter::init_user_router().await)
        .merge(SysDomainRouter::init_domain_router().await)
        .merge(SysRoleRouter::init_role_router().await)
        .merge(SysEndpointRouter::init_endpoint_router().await)
        .merge(SysAccessKeyRouter::init_access_key_router().await)
        .merge(SysLoginLogRouter::init_login_log_router().await)
        .merge(SysOperationLogRouter::init_operation_log_router().await)
        .merge(SysOrganizationRouter::init_organization_router().await);

    Ok(router)
}
