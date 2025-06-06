/// 数据库操作辅助模块
/// 
/// 该模块提供了关系型数据库操作的辅助函数，包括：
/// - 数据库连接池管理
/// - 连接获取和验证
/// - 错误处理和日志记录
/// 
/// 所有函数都提供了详细的错误处理和日志记录，便于问题诊断和监控。
/// 
/// # 使用示例
/// 
/// use server_service::helper::db_helper::*;
/// 
/// // 初始化数据库连接池
/// init_db_pool("postgres://user:pass@localhost/db").await?;
/// 
/// // 获取数据库连接
/// let conn = get_connection().await?;
/// 
/// // 验证连接
/// validate_connection(&conn).await?;
/// 

#[allow(dead_code)]
use sea_orm::{DatabaseConnection, DbErr, ConnAcquireErr, Database};
use server_global::global::GLOBAL_DB_POOL;
use std::sync::Arc;
use tracing::{info, error, warn};

/// 从连接池获取数据库连接
/// 
/// 尝试从默认连接池获取数据库连接，并提供详细的错误处理和日志记录。
/// 
/// # 返回
/// * `Result<Arc<DatabaseConnection>, sea_orm::DbErr>` - 成功返回数据库连接的Arc包装，失败返回错误
/// 
/// # 错误
/// * 如果默认连接池不存在，返回 `ConnectionAcquire(ConnAcquireErr::Timeout)`
/// 
/// # 使用示例
/// 
/// let conn = get_db_connection().await?;
/// 
pub async fn get_db_connection() -> Result<Arc<DatabaseConnection>, sea_orm::DbErr> {
    info!("Attempting to get default database connection");
    let pools = GLOBAL_DB_POOL.read().await;
    match pools.get("default") {
        Some(conn) => {
            info!("Successfully acquired database connection");
            Ok(conn.clone())
        },
        None => {
            let error_msg = "Failed to get database connection: Default pool not found";
            error!("{}", error_msg);
            warn!("Connection pool not initialized, please check database configuration");
            Err(sea_orm::DbErr::ConnectionAcquire(ConnAcquireErr::Timeout))
        }
    }
}

/// 初始化数据库连接池
/// 
/// 使用指定的数据库URL初始化默认连接池，并进行连接验证。
/// 
/// # 参数
/// * `database_url` - 数据库连接URL
/// 
/// # 返回
/// * `Result<(), sea_orm::DbErr>` - 成功返回 `()`，失败返回错误
/// 
/// # 使用示例
/// 
/// init_db_pool("postgres://user:pass@localhost/db").await?;
/// 
pub async fn init_db_pool(database_url: &str) -> Result<(), sea_orm::DbErr> {
    info!("Initializing database pool with URL: {}", database_url);
    let db = Database::connect(database_url).await?;
    let db = Arc::new(db);
    
    let mut pools = GLOBAL_DB_POOL.write().await;
    pools.insert("default".to_string(), db);
    
    info!("Database pool initialized successfully");
    Ok(())
}

/// 获取指定名称的数据库连接
/// 
/// 从连接池中获取指定名称的数据库连接，并提供详细的错误处理和日志记录。
/// 
/// # 参数
/// * `name` - 连接池名称
/// 
/// # 返回
/// * `Result<Arc<DatabaseConnection>, sea_orm::DbErr>` - 成功返回数据库连接的Arc包装，失败返回错误
/// 
/// # 错误
/// * 如果指定名称的连接池不存在，返回 `ConnectionAcquire(ConnAcquireErr::Timeout)`
/// 
/// # 使用示例
/// 
/// let conn = get_named_connection("secondary").await?;
/// 
#[allow(dead_code)]
pub async fn get_named_connection(name: &str) -> Result<Arc<DatabaseConnection>, sea_orm::DbErr> {
    info!("Attempting to get named database connection: {}", name);
    let pools = GLOBAL_DB_POOL.read().await;
    match pools.get(name) {
        Some(db) => {
            info!("Successfully acquired named database connection: {}", name);
            Ok(db.clone())
        },
        None => {
            let error_msg = format!("Database pool '{}' not found", name);
            error!("{}", error_msg);
            Err(sea_orm::DbErr::ConnectionAcquire(ConnAcquireErr::Timeout))
        }
    }
}

/// 获取默认数据库连接
/// 
/// 从默认连接池获取数据库连接的便捷方法。
/// 
/// # 返回
/// * `Result<Arc<DatabaseConnection>, DbErr>` - 成功返回数据库连接的Arc包装，失败返回错误
/// 
/// # 使用示例
/// 
/// let conn = get_connection().await?;
/// 
pub async fn get_connection() -> Result<Arc<DatabaseConnection>, DbErr> {
    get_db_connection().await
}

/// 验证数据库连接
/// 
/// 通过发送ping命令验证数据库连接是否有效。
/// 
/// # 参数
/// * `conn` - 要验证的数据库连接
/// 
/// # 返回
/// * `Result<(), DbErr>` - 成功返回 `()`，失败返回错误
/// 
/// # 使用示例
/// 
/// validate_connection(&conn).await?;
/// 
pub async fn validate_connection(conn: &DatabaseConnection) -> Result<(), DbErr> {
    info!("Validating database connection");
    match conn.ping().await {
        Ok(_) => {
            info!("Database connection validation successful");
            Ok(())
        },
        Err(e) => {
            error!("Database connection validation failed: {}", e);
            Err(e)
        }
    }
}
