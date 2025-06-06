/// 应用程序入口点
/// 
/// 该模块负责：
/// 1. 初始化应用程序配置
/// 2. 设置日志和追踪
/// 3. 初始化数据库连接
/// 4. 初始化Redis和MongoDB连接
/// 5. 设置路由和中间件
/// 6. 启动HTTP服务器
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
    let _ = server_initialize::init_xdb().await;
    server_initialize::init_primary_connection().await;
    server_initialize::init_db_pools().await;
    
    // 初始化密钥和验证器
    server_initialize::initialize_keys_and_validation().await;
    
    // 初始化事件通道
    server_initialize::initialize_event_channel().await;

    // 初始化Redis连接
    server_initialize::init_primary_redis().await;
    server_initialize::init_redis_pools().await;
    
    // 初始化MongoDB连接
    server_initialize::init_primary_mongo().await;
    server_initialize::init_mongo_pools().await;

    // 构建应用程序路由
    let app = server_initialize::initialize_admin_router().await;

    // 初始化访问密钥（需要在验证器初始化之后）
    server_initialize::initialize_access_key().await;

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
