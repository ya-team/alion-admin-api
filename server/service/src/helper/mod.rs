/**
 * 辅助服务模块
 *
 * 该模块提供了各种数据库和事务操作的辅助功能，包括：
 * - db_helper: 关系型数据库操作辅助函数
 * - mongo_helper: MongoDB数据库操作辅助函数
 * - redis_helper: Redis缓存操作辅助函数
 * - transaction_helper: 事务处理辅助函数
 *
 * 这些辅助函数封装了常用的数据库操作，提供了更简洁和统一的接口，
 * 同时处理了错误、连接管理和事务等底层细节。
 *
 * 使用示例
 * --------
 *
 * use server_service::helper::*;
 *
 * // 使用数据库辅助函数
 * let result = db_helper::query_one::<User>(&db, "SELECT * FROM users WHERE id = ?", &[1]).await?;
 *
 * // 使用事务辅助函数
 * let result = execute_in_transaction(&db, |tx| async move {
 *     // 事务操作
 *     Ok(())
 * }).await?;
 */

pub mod db_helper;
pub mod mongo_helper;
pub mod redis_helper;
pub mod transaction_helper;

// Remove unused imports
// pub use db_helper::*;
// pub use db_pool::*;

pub use db_helper::*;
pub use transaction_helper::{execute_in_transaction, execute_with_retry};
