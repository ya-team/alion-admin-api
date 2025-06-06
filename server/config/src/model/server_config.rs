/**
 * 服务器配置模块
 * 
 * 定义了HTTP服务器的基本配置参数
 */

use serde::Deserialize;

/**
 * 服务器配置结构体
 * 
 * 包含HTTP服务器的基本配置参数，如主机地址和端口号。
 * 这些参数用于启动HTTP服务器。
 */
#[derive(Deserialize, Debug, Clone)]
pub struct ServerConfig {
    /**
     * 服务器主机地址
     * 
     * 例如：
     * - "127.0.0.1" 用于本地开发
     * - "0.0.0.0" 用于生产环境，允许所有网络接口访问
     */
    pub host: String,

    /**
     * 服务器端口号
     * 
     * 例如：
     * - 8080 用于开发环境
     * - 80 用于HTTP生产环境
     * - 443 用于HTTPS生产环境
     */
    pub port: u32,
}
