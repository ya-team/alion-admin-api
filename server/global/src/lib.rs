/**
 * 全局模块
 * 
 * 该模块提供了应用程序的全局状态管理功能，包括：
 * - global: 全局状态管理，包括：
 *   - 配置管理：支持多种配置类型的存储和访问
 *   - 数据库连接池：支持主数据库和多个数据库连接的管理
 *   - Redis连接池：支持单实例和集群模式的Redis连接管理
 *   - MongoDB连接池：支持主MongoDB和多个MongoDB连接的管理
 *   - S3客户端池：支持主S3客户端和多个S3客户端的管理
 *   - JWT密钥管理：提供JWT令牌的签名和验证功能
 *   - 事件通道：支持字符串和动态类型的事件通信
 *   - 路由信息收集：记录和管理API路由信息
 *   - 操作日志上下文：记录和管理操作日志信息
 * 
 * # 主要功能
 * 
 * ## 配置管理
 * 支持存储和访问任意类型的配置对象，使用TypeId作为键进行类型安全的访问。
 * 
 * ## 连接池管理
 * 提供统一的连接池管理接口，支持多种数据库和服务的连接管理。
 * 
 * ## 事件系统
 * 提供基于通道的事件通信机制，支持字符串和动态类型的事件处理。
 * 
 * ## 日志系统
 * 提供两个重要的日志宏：
 * - project_info!: 用于记录信息级别的日志
 * - project_error!: 用于记录错误级别的日志
 * 
 * 这些宏会自动包含模块路径、文件名和行号信息，便于调试和问题追踪。
 */

/// 重新导出JWT验证器
pub use jsonwebtoken::Validation;

/// 全局状态管理模块
pub mod global;

/**
 * 项目信息日志宏
 * 
 * 用于记录信息级别的日志，自动包含模块路径、文件名和行号信息。
 * 
 * # 参数
 * * `$($arg:tt)+` - 格式化字符串和参数，与println!宏格式相同
 */
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

/**
 * 项目错误日志宏
 * 
 * 用于记录错误级别的日志，自动包含模块路径、文件名和行号信息。
 * 
 * # 参数
 * * `$($arg:tt)+` - 格式化字符串和参数，与println!宏格式相同
 */
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
