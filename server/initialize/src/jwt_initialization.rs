/**
 * JWT初始化模块
 * 
 * 本模块负责初始化JWT（JSON Web Token）相关配置，
 * 包括密钥、验证规则等。
 */

use std::sync::Arc;
use std::error::Error;

use server_config::JwtConfig;
use server_global::{global, Validation};
use tokio::sync::Mutex;

/**
 * 初始化JWT配置
 * 
 * 从配置中加载JWT相关设置，初始化JWT密钥和验证规则。
 * 
 * # 返回
 * - 成功：返回Ok(())
 * - 失败：返回错误信息
 * 
 * # 处理流程
 * 1. 加载JWT配置
 * 2. 创建JWT密钥
 * 3. 设置验证规则（包括签发者、受众等）
 * 4. 初始化全局JWT配置
 */
pub async fn init_jwt() -> Result<(), Box<dyn Error>> {
    let jwt_config = global::get_config::<JwtConfig>().await
        .ok_or_else(|| Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "JWT config not found")))?;
    
    let keys = global::Keys::new(jwt_config.secret.as_bytes());
    let mut validation = Validation::default();
    validation.leeway = 60;
    validation.set_issuer(&[jwt_config.issuer.clone()]);
    validation.set_audience(&[jwt_config.audience.clone()]);

    global::KEYS.set(Arc::new(Mutex::new(keys)))
        .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to set JWT keys: {}", e))))?;
    
    global::VALIDATION.set(Arc::new(Mutex::new(validation)))
        .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to set JWT validation: {}", e))))?;

    Ok(())
}
