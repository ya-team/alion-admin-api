/** 管理后台辅助模块
 * 
 * 该模块重新导出了通用的辅助函数，用于管理后台服务的实现。
 * 主要包括数据库操作和事务处理的辅助函数。
 * 
 * 主要组件
 * --------
 * 
 * 辅助函数
 * --------
 * * `db_helper`: 数据库操作辅助函数，提供数据库连接和基本操作
 * * `execute_in_transaction`: 事务执行辅助函数，用于在事务中执行数据库操作
 * * `execute_with_retry`: 带重试机制的执行辅助函数，用于处理临时性错误
 * 
 * 使用示例
 * --------
 * /* 在事务中执行操作
 *  * let result = execute_in_transaction(&db, |tx| Box::pin(async move {
 *  *     // 执行数据库操作
 *  *     Ok(())
 *  * })).await?;
 *  */
 * 
 * /* 使用重试机制执行操作
 *  * let result = execute_with_retry(|| async {
 *  *     // 执行可能失败的操作
 *  *     Ok(())
 *  * }).await?;
 *  */
 */

pub use crate::helper::{db_helper, execute_in_transaction, execute_with_retry}; 