/// 服务器核心功能模块
/// 
/// 该模块包含以下核心功能：
/// - sign: 签名相关功能
/// - web: Web服务相关功能，包括认证、JWT、分页、响应处理等
/// - macros: 通用宏定义

/// 签名模块，用于处理数据签名和验证
pub mod sign;

/// Web服务模块，包含以下子模块：
/// - auth: 认证相关功能
/// - jwt: JWT令牌处理
/// - page: 分页请求处理
/// - res: 响应处理
/// - util: 工具函数
/// - validator: 输入验证
/// - operation_log: 操作日志
/// - request_id: 请求ID处理
pub mod web;

/// 通用宏定义模块
pub mod macros;

/// 输入验证trait，用于验证请求输入数据
pub use web::validator::ValidateInput;

/// 分页请求结构体，用于处理分页查询请求
pub use web::page::PageRequest;
