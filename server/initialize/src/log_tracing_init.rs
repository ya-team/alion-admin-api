/**
 * 日志追踪初始化模块
 * 
 * 本模块负责初始化系统的日志追踪功能，包括：
 * - 配置日志级别
 * - 设置日志格式
 * - 初始化日志追踪器
 * - 配置错误追踪层
 */

use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

use crate::{project_error, project_info};

/**
 * 初始化日志追踪系统
 * 
 * 配置并初始化系统的日志追踪功能，包括：
 * - 设置日志级别（debug/release模式不同）
 * - 配置日志格式（包含目标、文件、行号等信息）
 * - 初始化日志追踪器
 * - 添加错误追踪层
 * 
 * # 处理流程
 * 1. 初始化日志追踪器
 * 2. 配置环境过滤器
 * 3. 设置日志格式
 * 4. 配置错误追踪层
 * 5. 设置全局订阅者
 */
pub async fn initialize_log_tracing() {
    if let Err(e) = LogTracer::init() {
        project_error!("Failed to set logger: {}", e);
        return;
    }

    let env_filter = if cfg!(debug_assertions) {
        EnvFilter::new("debug,sea_orm=debug,sqlx=debug")
    } else {
        EnvFilter::new("info,sea_orm=info,sqlx=info")
    };

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_target(true)
        .with_ansi(true)
        .with_file(true)
        .with_line_number(true);

    let subscriber = Registry::default()
        .with(env_filter)
        .with(fmt_layer)
        .with(tracing_error::ErrorLayer::default());

    if let Err(e) = tracing::subscriber::set_global_default(subscriber) {
        project_error!("Failed to set subscriber: {}", e);
        return;
    }

    if cfg!(debug_assertions) {
        project_info!("Log tracing initialized successfully in debug mode");
    } else {
        project_info!("Log tracing initialized successfully in release mode");
    }
}
