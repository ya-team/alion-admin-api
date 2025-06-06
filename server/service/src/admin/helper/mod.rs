/// 管理后台辅助模块
/// 
/// 该模块重新导出了通用的辅助函数，用于管理后台服务的实现。
/// 主要包括数据库操作和事务处理的辅助函数。
/// 
/// # 导出的函数
/// * `db_helper`: 数据库操作辅助函数
/// * `execute_in_transaction`: 事务执行辅助函数
/// * `execute_with_retry`: 带重试机制的执行辅助函数
/// 
/// # 使用示例
/// 
/// use server_service::admin::helper::*;
/// 
/// // 在事务中执行操作
/// let result = execute_in_transaction(&db, |tx| Box::pin(async move {
///     // 执行数据库操作
///     Ok(())
/// })).await?;
/// 

pub use crate::helper::{db_helper, execute_in_transaction, execute_with_retry}; 