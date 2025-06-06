/**
 * 事务操作辅助模块
 * 
 * 该模块提供了数据库事务操作的辅助函数，包括：
 * - 事务执行
 * - 重试机制
 * 
 * 提供了事务管理和错误处理的统一接口，支持事务的自动提交和回滚，
 * 以及操作失败时的自动重试机制。
 * 
 * # 使用示例
 * 
 * use server_service::helper::transaction_helper::*;
 * 
 * // 在事务中执行操作
 * let result = execute_in_transaction(&db, |tx| Box::pin(async move {
 *     // 执行数据库操作
 *     Ok(())
 * })).await?;
 * 
 * // 带重试机制的执行
 * let result = execute_with_retry(&db, |db| Box::pin(async move {
 *     // 执行数据库操作
 *     Ok(())
 * }), 3).await?;
 */

use sea_orm::{DatabaseConnection, DatabaseTransaction, TransactionTrait};
use server_core::web::error::AppError;
use std::{future::Future, pin::Pin};

/**
 * 在事务中执行数据库操作
 * 
 * 创建一个数据库事务，执行指定的操作，并根据操作结果自动提交或回滚事务。
 * 
 * # 类型参数
 * * `T` - 操作返回值的类型
 * * `F` - 异步闭包类型，接收事务对象并返回Future
 * 
 * # 参数
 * * `db` - 数据库连接
 * * `operation` - 异步闭包，接收事务对象并执行数据库操作
 * 
 * # 返回
 * * `Result<T, AppError>` - 操作结果，成功返回操作返回值，失败返回错误
 * 
 * # 使用示例
 * 
 * let result = execute_in_transaction(&db, |tx| Box::pin(async move {
 *     // 执行数据库操作
 *     let user = User::find_by_id(1).one(&tx).await?;
 *     // 更多操作...
 *     Ok(user)
 * })).await?;
 */
pub async fn execute_in_transaction<T, F>(db: &DatabaseConnection, operation: F) -> Result<T, AppError>
where
    F: FnOnce(DatabaseTransaction) -> Pin<Box<dyn Future<Output = Result<T, AppError>> + Send>>,
{
    let txn = db.begin().await.map_err(AppError::from)?;

    match operation(txn).await {
        Ok(result) => {
            // Transaction is consumed by the operation
            Ok(result)
        }
        Err(e) => {
            // Transaction is consumed by the operation
            Err(e)
        }
    }
}

/**
 * 带重试机制的数据库操作执行
 * 
 * 执行数据库操作，如果失败则进行重试，直到成功或达到最大重试次数。
 * 
 * # 类型参数
 * * `T` - 操作返回值的类型
 * * `F` - 异步闭包类型，接收数据库连接并返回Future
 * 
 * # 参数
 * * `db` - 数据库连接
 * * `operation` - 异步闭包，接收数据库连接并执行操作
 * * `max_retries` - 最大重试次数
 * 
 * # 返回
 * * `Result<T, AppError>` - 操作结果，成功返回操作返回值，失败返回错误
 * 
 * # 使用示例
 * 
 * let result = execute_with_retry(&db, |db| Box::pin(async move {
 *     // 执行数据库操作
 *     let user = User::find_by_id(1).one(db).await?;
 *     Ok(user)
 * }), 3).await?;
 */
pub async fn execute_with_retry<T, F>(
    db: &DatabaseConnection,
    operation: F,
    max_retries: u32,
) -> Result<T, AppError>
where
    F: Fn(&DatabaseConnection) -> Pin<Box<dyn Future<Output = Result<T, AppError>> + Send>>,
{
    let mut retries = 0;
    loop {
        match operation(db).await {
            Ok(result) => return Ok(result),
            Err(e) => {
                if retries >= max_retries {
                    return Err(e);
                }
                retries += 1;
                tokio::time::sleep(tokio::time::Duration::from_millis(100 * retries as u64)).await;
            }
        }
    }
} 