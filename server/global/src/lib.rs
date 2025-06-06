/// 全局模块
/// 
/// 该模块提供了应用程序的全局状态管理功能，包括：
/// - global: 全局状态管理，包括：
///   - 配置管理
///   - 数据库连接池
///   - Redis连接池
///   - MongoDB连接池
///   - S3客户端池
///   - JWT密钥管理
///   - 事件通道
///   - 路由信息收集
///   - 操作日志上下文
/// 
/// 该模块还提供了两个重要的日志宏：
/// - project_info!: 用于记录信息级别的日志
/// - project_error!: 用于记录错误级别的日志
/// 
/// 这些宏会自动包含模块路径、文件名和行号信息，便于调试和问题追踪。

/// 重新导出JWT验证器
pub use jsonwebtoken::Validation;

/// 全局状态管理模块
pub mod global;

/// 项目信息日志宏
/// 
/// 用法示例：
/// ```rust
/// project_info!("User {} logged in", username);
/// ```
#[macro_export]
macro_rules! project_info {
    ($($arg:tt)+) => {{
        let span = tracing::span!(
            tracing::Level::INFO,
            module_path!(),
            file = file!(),
            line = line!(),
        );
        let _enter = span.enter();
        tracing::info!(
            target: "[alion-admin]",
            $($arg)+
        );
    }}
}

/// 项目错误日志宏
/// 
/// 用法示例：
/// ```rust
/// project_error!("Failed to connect to database: {}", error);
/// ```
#[macro_export]
macro_rules! project_error {
    ($($arg:tt)+) => {{
        let span = tracing::span!(
            tracing::Level::ERROR,
            module_path!(),
            file = file!(),
            line = line!(),
        );
        let _enter = span.enter();
        tracing::error!(
            target: "[alion-admin]",
            $($arg)+
        );
    }}
}
