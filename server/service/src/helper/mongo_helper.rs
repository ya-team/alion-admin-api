/// MongoDB操作辅助模块
/// 
/// 该模块提供了MongoDB数据库操作的辅助函数，包括：
/// - 主MongoDB实例操作
/// - 命名MongoDB实例操作
/// - 通用接口操作
/// 
/// 支持从主MongoDB实例或命名MongoDB实例获取客户端和集合，
/// 提供了统一的错误处理和类型安全的接口。
/// 
/// # 使用示例
/// 
/// use server_service::helper::mongo_helper::*;
/// 
/// // 获取主MongoDB集合
/// let collection = get_primary_collection::<User>("mydb", "users").await?;
/// 
/// // 获取命名MongoDB集合
/// let collection = get_named_collection::<Log>("logs", "mydb", "logs").await?;
/// 
/// // 使用通用接口
/// let collection = get_collection::<Document>(
///     MongoSource::Named("logs".to_string()),
///     "mydb",
///     "logs"
/// ).await?;
/// 

#[allow(dead_code)]
use mongodb::{Client, Collection};
use server_core::web::error::AppError;
use server_global::global::{GLOBAL_MONGO_POOL, GLOBAL_PRIMARY_MONGO};

/// MongoDB连接来源
/// 
/// 用于指定MongoDB连接的来源，支持主实例和命名实例。
/// 
/// # 变体
/// * `Primary` - 使用主MongoDB实例
/// * `Named(String)` - 使用指定名称的MongoDB实例
#[derive(Debug, Clone)]
pub enum MongoSource {
    /// 主MongoDB实例
    Primary,
    /// 命名的MongoDB实例
    Named(String),
}

// ===== 主数据库操作 =====

/// 获取主MongoDB客户端
/// 
/// 从全局状态中获取主MongoDB客户端实例。
/// 
/// # 返回
/// * `Result<Client, AppError>` - 成功返回MongoDB客户端，失败返回错误
/// 
/// # 错误
/// * 如果主MongoDB未初始化，返回500错误
/// 
/// # 使用示例
/// 
/// let client = get_primary_client().await?;
/// 
pub async fn get_primary_client() -> Result<Client, AppError> {
    let client = GLOBAL_PRIMARY_MONGO
        .read()
        .await
        .clone()
        .ok_or_else(|| AppError {
            code: 500,
            message: "Primary MongoDB not initialized".to_string(),
        })?;
    Ok(client.as_ref().clone())
}

/// 获取主MongoDB指定数据库的集合
/// 
/// 从主MongoDB实例获取指定数据库和集合的引用。
/// 
/// # 类型参数
/// * `T` - 集合文档类型，必须实现Send + Sync trait
/// 
/// # 参数
/// * `db_name` - 数据库名称
/// * `coll_name` - 集合名称
/// 
/// # 返回
/// * `Result<Collection<T>, AppError>` - 成功返回集合引用，失败返回错误
/// 
/// # 使用示例
/// 
/// let collection = get_primary_collection::<User>("mydb", "users").await?;
/// 
pub async fn get_primary_collection<T: Send + Sync>(
    db_name: &str,
    coll_name: &str,
) -> Result<Collection<T>, AppError> {
    let client = get_primary_client().await?;
    Ok(client.database(db_name).collection(coll_name))
}

// ===== 命名数据库操作 =====

/// 获取命名MongoDB客户端
/// 
/// 从全局状态中获取指定名称的MongoDB客户端实例。
/// 
/// # 参数
/// * `name` - MongoDB实例名称
/// 
/// # 返回
/// * `Result<Client, AppError>` - 成功返回MongoDB客户端，失败返回错误
/// 
/// # 错误
/// * 如果指定名称的MongoDB实例不存在，返回500错误
/// 
/// # 使用示例
/// 
/// let client = get_named_client("logs").await?;
/// 
pub async fn get_named_client(name: &str) -> Result<Client, AppError> {
    let pools = GLOBAL_MONGO_POOL.read().await;
    let client = pools.get(name).ok_or_else(|| AppError {
        code: 500,
        message: format!("MongoDB pool '{}' not found", name),
    })?;
    Ok(client.as_ref().clone())
}

/// 获取命名MongoDB指定数据库的集合
/// 
/// 从命名MongoDB实例获取指定数据库和集合的引用。
/// 
/// # 类型参数
/// * `T` - 集合文档类型，必须实现Send + Sync trait
/// 
/// # 参数
/// * `pool_name` - MongoDB实例名称
/// * `db_name` - 数据库名称
/// * `coll_name` - 集合名称
/// 
/// # 返回
/// * `Result<Collection<T>, AppError>` - 成功返回集合引用，失败返回错误
/// 
/// # 使用示例
/// 
/// let collection = get_named_collection::<Log>("logs", "mydb", "logs").await?;
/// 
pub async fn get_named_collection<T: Send + Sync>(
    pool_name: &str,
    db_name: &str,
    coll_name: &str,
) -> Result<Collection<T>, AppError> {
    let client = get_named_client(pool_name).await?;
    Ok(client.database(db_name).collection(coll_name))
}

// ===== 通用接口 =====

/// 获取MongoDB客户端
/// 
/// 根据指定的来源获取MongoDB客户端实例。
/// 
/// # 参数
/// * `source` - MongoDB连接来源
/// 
/// # 返回
/// * `Result<Client, AppError>` - 成功返回MongoDB客户端，失败返回错误
/// 
/// # 使用示例
/// 
/// let client = get_client(MongoSource::Primary).await?;
/// let client = get_client(MongoSource::Named("logs".to_string())).await?;
/// 
pub async fn get_client(source: MongoSource) -> Result<Client, AppError> {
    match source {
        MongoSource::Primary => get_primary_client().await,
        MongoSource::Named(name) => get_named_client(&name).await,
    }
}

/// 获取MongoDB集合
/// 
/// 根据指定的来源获取MongoDB集合引用。
/// 
/// # 类型参数
/// * `T` - 集合文档类型，必须实现Send + Sync trait
/// 
/// # 参数
/// * `source` - MongoDB连接来源
/// * `db_name` - 数据库名称
/// * `coll_name` - 集合名称
/// 
/// # 返回
/// * `Result<Collection<T>, AppError>` - 成功返回集合引用，失败返回错误
/// 
/// # 使用示例
/// 
/// let collection = get_collection::<Document>(
///     MongoSource::Named("logs".to_string()),
///     "mydb",
///     "logs"
/// ).await?;
/// 
pub async fn get_collection<T: Send + Sync>(
    source: MongoSource,
    db_name: &str,
    coll_name: &str,
) -> Result<Collection<T>, AppError> {
    match source {
        MongoSource::Primary => get_primary_collection(db_name, coll_name).await,
        MongoSource::Named(name) => get_named_collection(&name, db_name, coll_name).await,
    }
}
