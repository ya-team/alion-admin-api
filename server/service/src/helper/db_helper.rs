#![allow(dead_code)]

use sea_orm::{DatabaseConnection, DbErr, ConnAcquireErr, Database};
use server_global::global::GLOBAL_DB_POOL;
use std::sync::Arc;
use tracing::{info, error, warn};

/// Get a database connection from the pool with detailed error handling
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

/// Initialize the database connection pool with validation
pub async fn init_db_pool(database_url: &str) -> Result<(), sea_orm::DbErr> {
    info!("Initializing database pool with URL: {}", database_url);
    let db = Database::connect(database_url).await?;
    let db = Arc::new(db);
    
    let mut pools = GLOBAL_DB_POOL.write().await;
    pools.insert("default".to_string(), db);
    
    info!("Database pool initialized successfully");
    Ok(())
}

/// Get a named database connection with validation
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

/// Get a database connection from the default pool with validation
pub async fn get_connection() -> Result<Arc<DatabaseConnection>, DbErr> {
    get_db_connection().await
}

/// Validate database connection
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
