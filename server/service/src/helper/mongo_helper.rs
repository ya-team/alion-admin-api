#![allow(dead_code)]
use mongodb::{Client, Collection};
use server_core::web::error::AppError;
use server_global::global::{GLOBAL_MONGO_POOL, GLOBAL_PRIMARY_MONGO};

/// MongoDB 连接来源
#[derive(Debug, Clone)]
pub enum MongoSource {
    /// 主 MongoDB 实例
    Primary,
    /// 命名的 MongoDB 实例
    Named(String),
}

// ===== 主数据库操作 =====

/// 获取主 MongoDB 客户端
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

/// 获取主 MongoDB 指定数据库的集合
pub async fn get_primary_collection<T: Send + Sync>(
    db_name: &str,
    coll_name: &str,
) -> Result<Collection<T>, AppError> {
    let client = get_primary_client().await?;
    Ok(client.database(db_name).collection(coll_name))
}

// ===== 命名数据库操作 =====

/// 获取命名 MongoDB 客户端
pub async fn get_named_client(name: &str) -> Result<Client, AppError> {
    let pools = GLOBAL_MONGO_POOL.read().await;
    let client = pools.get(name).ok_or_else(|| AppError {
        code: 500,
        message: format!("MongoDB pool '{}' not found", name),
    })?;
    Ok(client.as_ref().clone())
}

/// 获取命名 MongoDB 指定数据库的集合
pub async fn get_named_collection<T: Send + Sync>(
    pool_name: &str,
    db_name: &str,
    coll_name: &str,
) -> Result<Collection<T>, AppError> {
    let client = get_named_client(pool_name).await?;
    Ok(client.database(db_name).collection(coll_name))
}

// ===== 通用接口 =====

/// 获取 MongoDB 客户端
pub async fn get_client(source: MongoSource) -> Result<Client, AppError> {
    match source {
        MongoSource::Primary => get_primary_client().await,
        MongoSource::Named(name) => get_named_client(&name).await,
    }
}

/// 获取 MongoDB 集合
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
