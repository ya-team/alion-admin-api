/**
 * AWS S3初始化模块
 * 
 * 本模块负责初始化AWS S3存储服务，包括：
 * - 初始化主S3客户端
 * - 初始化S3连接池
 * - 管理S3客户端连接
 * - 提供S3操作功能
 */
#[allow(dead_code)]
use std::{process, sync::Arc};
use std::error::Error;

use aws_config::BehaviorVersion;
use aws_sdk_s3::{
    config::{Credentials, Region},
    Client as S3Client,
};
use server_config::{OptionalConfigs, S3Config, S3InstancesConfig};
use server_global::global::{get_config, GLOBAL_PRIMARY_S3, GLOBAL_S3_POOL};

use crate::{project_error, project_info};

/**
 * 初始化主S3客户端
 * 
 * 根据配置创建并初始化主S3客户端连接。
 * 如果初始化失败，程序将退出。
 */
pub async fn init_primary_s3() {
    if let Some(config) = get_config::<S3Config>().await {
        match create_s3_client(&config).await {
            Ok(client) => {
                *GLOBAL_PRIMARY_S3.write().await = Some(Arc::new(client));
                project_info!("Primary S3 client initialized");
            },
            Err(e) => {
                project_error!("Failed to initialize primary S3 client: {}", e);
                process::exit(1);
            },
        }
    }
}

/**
 * 初始化所有S3客户端
 * 
 * 从配置中读取所有S3实例配置，
 * 并为每个实例创建S3客户端连接。
 */
pub async fn init_s3_pools() {
    if let Some(s3_instances_config) = get_config::<OptionalConfigs<S3InstancesConfig>>().await {
        if let Some(s3_instances) = &s3_instances_config.configs {
            let _ = init_s3_pool(Some(s3_instances.clone())).await;
        }
    }
}

/**
 * 初始化S3连接池
 * 
 * # 参数
 * - s3_instances_config: S3实例配置列表
 * 
 * # 返回
 * - 成功：返回Ok(())
 * - 失败：返回错误信息
 */
pub async fn init_s3_pool(
    s3_instances_config: Option<Vec<S3InstancesConfig>>,
) -> Result<(), String> {
    if let Some(s3_instances) = s3_instances_config {
        for s3_instance in s3_instances {
            init_s3_connection(&s3_instance.name, &s3_instance.s3).await?;
        }
    }
    Ok(())
}

/**
 * 初始化单个S3连接
 * 
 * # 参数
 * - name: S3实例名称
 * - config: S3配置信息
 * 
 * # 返回
 * - 成功：返回Ok(())
 * - 失败：返回错误信息
 */
async fn init_s3_connection(name: &str, config: &S3Config) -> Result<(), String> {
    match create_s3_client(config).await {
        Ok(client) => {
            let client_arc = Arc::new(client);
            GLOBAL_S3_POOL
                .write()
                .await
                .insert(name.to_string(), client_arc);
            project_info!("S3 client '{}' initialized", name);
            Ok(())
        },
        Err(e) => {
            let error_msg = format!("Failed to initialize S3 client '{}': {}", name, e);
            project_error!("{}", error_msg);
            Err(error_msg)
        },
    }
}

/**
 * 创建S3客户端
 * 
 * # 参数
 * - config: S3配置信息
 * 
 * # 返回
 * - 成功：返回S3Client实例
 * - 失败：返回错误信息
 * 
 * # 处理流程
 * 1. 配置AWS SDK
 * 2. 设置区域
 * 3. 配置端点
 * 4. 设置认证信息
 * 5. 创建客户端
 */
pub async fn create_s3_client(config: &S3Config) -> Result<S3Client, Box<dyn Error>> {
    let mut sdk_config = aws_config::defaults(BehaviorVersion::latest());
    
    // Handle region
    if let Some(region) = &config.region {
        sdk_config = sdk_config.region(Region::new(region.clone()));
    }

    // Handle endpoint
    sdk_config = sdk_config.endpoint_url(&config.endpoint);

    // Handle credentials if provided
    if !config.access_key_id.is_empty() && !config.access_key_secret.is_empty() {
        sdk_config = sdk_config.credentials_provider(
            Credentials::new(
                config.access_key_id.clone(),
                config.access_key_secret.clone(),
                None,
                None,
                "static",
            ),
        );
    }

    let sdk_config = sdk_config.load().await;
    let client = S3Client::new(&sdk_config);

    Ok(client)
}

/**
 * 获取主S3客户端
 * 
 * # 返回
 * - 成功：返回主S3客户端实例
 * - 失败：返回None
 */
#[allow(dead_code)]
pub async fn get_primary_s3_client() -> Option<Arc<S3Client>> {
    GLOBAL_PRIMARY_S3.read().await.clone()
}

/**
 * 获取命名的S3客户端
 * 
 * # 参数
 * - name: S3实例名称
 * 
 * # 返回
 * - 成功：返回对应的S3客户端实例
 * - 失败：返回None
 */
#[allow(dead_code)]
pub async fn get_s3_pool_connection(name: &str) -> Option<Arc<S3Client>> {
    GLOBAL_S3_POOL.read().await.get(name).cloned()
}

/**
 * 添加或更新S3客户端
 * 
 * # 参数
 * - name: S3实例名称
 * - config: S3配置信息
 * 
 * # 返回
 * - 成功：返回Ok(())
 * - 失败：返回错误信息
 */
#[allow(dead_code)]
pub async fn add_or_update_s3_pool(name: &str, config: &S3Config) -> Result<(), String> {
    init_s3_connection(name, config).await
}

/**
 * 移除S3客户端
 * 
 * # 参数
 * - name: S3实例名称
 * 
 * # 返回
 * - 成功：返回Ok(())
 * - 失败：返回错误信息
 */
#[allow(dead_code)]
pub async fn remove_s3_pool(name: &str) -> Result<(), String> {
    let mut s3_pool = GLOBAL_S3_POOL.write().await;
    s3_pool
        .remove(name)
        .ok_or_else(|| format!("S3 client '{}' not found", name))?;
    project_info!("S3 client '{}' removed", name);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::initialize_config;
    use aws_sdk_s3::types::{BucketLocationConstraint, CreateBucketConfiguration};
    use log::LevelFilter;
    use simple_logger::SimpleLogger;
    use tokio::sync::Mutex;

    static INITIALIZED: Mutex<Option<Arc<()>>> = Mutex::const_new(None);
    static TEST_BUCKET_NAME: &str = "test-bucket-rust-s3";

    fn setup_logger() {
        let _ = SimpleLogger::new().with_level(LevelFilter::Info).init();
    }

    async fn init() {
        let mut initialized = INITIALIZED.lock().await;
        if initialized.is_none() {
            initialize_config("../resources/application.yaml").await;
            *initialized = Some(Arc::new(()));
        }
    }

    // 测试 S3 基本操作
    async fn test_s3_operations(client: &S3Client) -> Result<(), String> {
        // 确保测试桶存在
        let bucket_exists = client
            .head_bucket()
            .bucket(TEST_BUCKET_NAME)
            .send()
            .await
            .is_ok();

        if !bucket_exists {
            let constraint =
                BucketLocationConstraint::from(client.config().region().unwrap().as_ref());
            let bucket_config = CreateBucketConfiguration::builder()
                .location_constraint(constraint)
                .build();

            client
                .create_bucket()
                .bucket(TEST_BUCKET_NAME)
                .create_bucket_configuration(bucket_config)
                .send()
                .await
                .map_err(|e| format!("Failed to create test bucket: {}", e))?;
        }

        // 上传测试对象
        let test_object_key = "test-object.txt";
        let test_content = "Hello, S3!";

        client
            .put_object()
            .bucket(TEST_BUCKET_NAME)
            .key(test_object_key)
            .body(test_content.as_bytes().to_vec().into())
            .send()
            .await
            .map_err(|e| format!("Failed to upload test object: {}", e))?;

        // 获取测试对象
        let get_response = client
            .get_object()
            .bucket(TEST_BUCKET_NAME)
            .key(test_object_key)
            .send()
            .await
            .map_err(|e| format!("Failed to get test object: {}", e))?;

        let data = get_response
            .body
            .collect()
            .await
            .map_err(|e| format!("Failed to read object data: {}", e))?;

        let content = String::from_utf8(data.into_bytes().to_vec())
            .map_err(|e| format!("Failed to convert data to string: {}", e))?;

        if content != test_content {
            return Err(format!(
                "Object content mismatch. Expected: '{}', Got: '{}'",
                test_content, content
            ));
        }

        // 删除测试对象
        client
            .delete_object()
            .bucket(TEST_BUCKET_NAME)
            .key(test_object_key)
            .send()
            .await
            .map_err(|e| format!("Failed to delete test object: {}", e))?;

        Ok(())
    }

    #[tokio::test]
    async fn test_primary_s3_connection() {
        setup_logger();
        init().await;

        init_primary_s3().await;

        let client = get_primary_s3_client().await;
        assert!(client.is_some(), "Primary S3 client does not exist");

        if let Some(client) = client {
            let result = test_s3_operations(&client).await;
            assert!(
                result.is_ok(),
                "S3 operations test failed: {:?}",
                result.err()
            );
        }
    }

    #[tokio::test]
    async fn test_s3_pool_operations() {
        setup_logger();
        init().await;

        // 使用测试配置创建测试客户端
        let test_config = S3InstancesConfig {
            name: "test_s3".to_string(),
            s3: S3Config {
                endpoint: "http://localhost:4566".to_string(),
                access_key_id: "test_key".to_string(),
                access_key_secret: "test_secret".to_string(),
                region: Some("us-east-1".to_string()),
                bucket: "test-bucket".to_string(),
                use_ssl: false,
                custom_domain: None,
            },
        };

        // 初始化测试S3池
        let result = init_s3_pool(Some(vec![test_config.clone()])).await;
        assert!(
            result.is_ok(),
            "Failed to initialize S3 pool: {:?}",
            result.err()
        );

        // 测试连接池连接
        let pool_connection = get_s3_pool_connection("test_s3").await;
        assert!(pool_connection.is_some(), "Pool connection not found");

        if let Some(client) = pool_connection {
            let result = test_s3_operations(&client).await;
            assert!(
                result.is_ok(),
                "S3 pool operations test failed: {:?}",
                result.err()
            );
        }

        // 测试添加新连接
        let add_result = add_or_update_s3_pool("test_new", &test_config.s3).await;
        assert!(add_result.is_ok(), "Failed to add S3 connection");

        // 测试移除连接
        let remove_result = remove_s3_pool("test_new").await;
        assert!(remove_result.is_ok(), "Failed to remove S3 connection");

        let connection_after_removal = get_s3_pool_connection("test_new").await;
        assert!(
            connection_after_removal.is_none(),
            "S3 connection still exists after removal"
        );
    }

    #[tokio::test]
    async fn test_s3_client_initialization() {
        let config = S3Config {
            endpoint: "http://localhost:4566".to_string(),
            access_key_id: "test_key".to_string(),
            access_key_secret: "test_secret".to_string(),
            region: Some("us-east-1".to_string()),
            bucket: "test-bucket".to_string(),
            use_ssl: false,
            custom_domain: None,
        };

        let result = create_s3_client(&config).await;
        assert!(result.is_ok());
    }
}
