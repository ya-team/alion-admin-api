/**
 * 应用程序入口点
 * 
 * 该模块负责应用程序的启动和初始化，包括：
 * 1. 初始化应用程序配置
 * 2. 设置日志和追踪
 * 3. 初始化数据库连接
 * 4. 初始化Redis连接
 * 5. 设置路由和中间件
 * 6. 启动HTTP服务器
 * 
 * 初始化流程：
 * 1. 根据运行环境选择配置文件
 * 2. 初始化日志和追踪系统
 * 3. 加载应用程序配置
 * 4. 初始化数据库连接池
 * 5. 初始化JWT和访问密钥
 * 6. 初始化Redis连接池
 * 7. 构建应用程序路由
 * 8. 启动HTTP服务器
 * 
 * 错误处理：
 * - 所有初始化步骤都有适当的错误处理
 * - 关键初始化失败会导致程序退出
 * - 错误信息会被记录到日志系统
 */
use std::net::SocketAddr;

use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // 根据运行环境选择配置文件路径
    let config_path = if cfg!(debug_assertions) {
        "server/resources/application-test.yaml"
    } else {
        "server/resources/application.yaml"
    };

    // 初始化日志和追踪系统
    server_initialize::initialize_log_tracing().await;
    
    // 从配置文件初始化应用程序配置
    server_initialize::initialize_config(config_path).await;
    
    // 初始化数据库连接
    if let Err(e) = server_initialize::init_xdb().await {
        eprintln!("Failed to initialize XDB: {}", e);
        return;
    }
    if let Err(e) = server_initialize::init_primary_connection().await {
        eprintln!("Failed to initialize primary database connection: {}", e);
        return;
    }
    server_initialize::init_db_pools().await;
    
    // 初始化密钥和验证器
    if let Err(e) = server_initialize::init_jwt().await {
        eprintln!("Failed to initialize JWT: {}", e);
        return;
    }
    server_initialize::initialize_access_key().await;
    server_initialize::initialize_event_channel().await;

    // 初始化Redis连接
    server_initialize::init_primary_redis().await;
    server_initialize::init_redis_pools().await;

    // 构建应用程序路由
    let app = server_initialize::initialize_admin_router().await;

    // 获取服务器地址
    let addr = match server_initialize::get_server_address().await {
        Ok(addr) => addr,
        Err(e) => {
            eprintln!("Failed to get server address: {}", e);
            return;
        },
    };

    // 启动HTTP服务器
    let listener = TcpListener::bind(&addr).await.unwrap();
    // tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
