/**
 * 沙箱管理API
 * 
 * 提供沙箱环境的测试接口，包括：
 * - 简单API密钥测试
 * - 复杂API密钥测试
 */
use server_core::web::{error::AppError, res::Res};

pub struct SysSandboxApi;

impl SysSandboxApi {
    /**
     * 简单API密钥测试
     * 
     * # 返回
     * 返回固定的测试字符串 "SimpleApiKey"
     */
    pub async fn test_simple_api_key() -> Result<Res<String>, AppError> {
        Ok(Res::new_data("SimpleApiKey".to_string()))
    }

    /**
     * 复杂API密钥测试
     * 
     * # 返回
     * 返回固定的测试字符串 "ComplexApiKey"
     */
    pub async fn test_complex_api_key() -> Result<Res<String>, AppError> {
        Ok(Res::new_data("ComplexApiKey".to_string()))
    }
}
