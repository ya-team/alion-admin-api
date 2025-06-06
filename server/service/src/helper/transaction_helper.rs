use sea_orm::{DatabaseConnection, DatabaseTransaction, TransactionTrait};
use server_core::web::error::AppError;
use std::{future::Future, pin::Pin};

/// Execute a database operation within a transaction
/// 
/// # Arguments
/// 
/// * `db` - The database connection
/// * `operation` - The async closure that performs the database operation
/// 
/// # Returns
/// 
/// * `Result<T, AppError>` - The result of the operation
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

/// Execute a database operation with retry logic
/// 
/// # Arguments
/// 
/// * `db` - The database connection
/// * `operation` - The async closure that performs the database operation
/// * `max_retries` - Maximum number of retry attempts
/// 
/// # Returns
/// 
/// * `Result<T, AppError>` - The result of the operation
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