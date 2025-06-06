/// 服务器核心功能模块
/// 
/// 该模块提供了服务器运行所需的核心功能，包括：
/// - 签名验证：支持简单和复杂的API密钥验证机制
/// - Web服务：提供完整的Web服务功能，包括认证、JWT、分页等
/// - 宏定义：提供简化代码编写的通用宏
/// 
/// # 模块结构
/// 
/// ## sign 模块
/// 提供API密钥验证和签名验证功能：
/// - 简单API密钥验证
/// - 复杂签名验证（支持多种算法）
/// - Nonce存储管理
/// - API密钥中间件
/// 
/// ## web 模块
/// 提供Web服务相关功能：
/// - auth: 用户认证和授权
/// - jwt: JWT令牌的生成和验证
/// - page: 分页请求的处理
/// - res: 统一响应格式处理
/// - util: 通用工具函数
/// - validator: 请求输入验证
/// - operation_log: 操作日志记录
/// - request_id: 请求ID生成和追踪
/// 
/// ## macros 模块
/// 提供简化代码编写的宏：
/// - validated_struct: 定义带验证规则的结构体
/// - page_request: 定义分页请求结构体
/// - create_input: 定义创建操作的输入类型
/// - update_input: 定义更新操作的输入类型

/// 签名模块
/// 
/// 提供API密钥验证和签名验证功能，支持：
/// - 简单API密钥验证
/// - 复杂签名验证（支持MD5、SHA1、SHA256、HMAC-SHA256）
/// - 内存和Redis的Nonce存储
/// - API密钥中间件
pub mod sign;

/// Web服务模块
/// 
/// 提供完整的Web服务功能，包括：
/// - auth: 用户认证和授权管理
/// - jwt: JWT令牌的生成、验证和刷新
/// - page: 分页请求的处理和响应
/// - res: 统一响应格式的定义和处理
/// - util: 通用工具函数集合
/// - validator: 请求输入数据的验证
/// - operation_log: 操作日志的记录和查询
/// - request_id: 请求ID的生成和追踪
pub mod web;

/// 通用宏定义模块
/// 
/// 提供简化代码编写的宏，包括：
/// - validated_struct: 快速定义带验证规则的结构体
/// - page_request: 快速定义分页请求结构体
/// - create_input: 快速定义创建操作的输入类型
/// - update_input: 快速定义更新操作的输入类型
pub mod macros;

/// 输入验证trait
/// 
/// 用于验证请求输入数据的trait，提供：
/// - 字段级别的验证规则
/// - 自定义验证逻辑
/// - 验证错误处理
pub use web::validator::ValidateInput;

/// 分页请求结构体
/// 
/// 用于处理分页查询请求，包含：
/// - 页码和每页大小
/// - 排序字段和方向
/// - 关键词搜索
pub use web::page::PageRequest;
