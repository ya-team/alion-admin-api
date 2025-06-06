/// 服务层模块
/// 
/// 该模块实现了应用程序的业务逻辑层，包括：
/// - admin: 管理后台相关的业务逻辑服务
/// - helper: 通用辅助服务
/// 
/// 服务层负责：
/// 1. 实现具体的业务逻辑
/// 2. 处理数据验证和转换
/// 3. 协调不同数据源的操作
/// 4. 实现业务规则和约束
/// 5. 处理事务和并发

/// 管理后台相关的业务逻辑服务
pub mod admin;

/// 通用辅助服务
pub mod helper;

/// 重新导出常量定义
pub use server_constant::definition::Audience;

/// 重新导出全局错误和信息处理
pub use server_global::{project_error, project_info};

/// 重新导出系统端点模型
pub use server_model::admin::entities::sys_endpoint::Model as SysEndpoint;
