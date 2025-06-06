/**
 * 服务器初始化模块
 * 
 * 本模块负责初始化HTTP服务器的基本配置，
 * 包括服务器地址、端口等设置。
 */

use std::error::Error;

use server_config::ServerConfig;
use server_global::global;

use crate::project_info;

/**
 * 获取服务器地址
 * 
 * 从配置中读取服务器的主机地址和端口，
 * 组合成完整的服务器地址。
 * 
 * # 返回
 * - 成功：返回格式化的服务器地址（host:port）
 * - 失败：返回错误信息
 * 
 * # 处理流程
 * 1. 读取服务器配置
 * 2. 组合主机地址和端口
 * 3. 记录配置的服务器地址
 */
pub async fn get_server_address() -> Result<String, Box<dyn Error>> {
    let server_config = global::get_config::<ServerConfig>().await.unwrap();
    let addr = format!("{}:{}", server_config.host, server_config.port);
    project_info!("Server address configured: {}", addr);
    Ok(addr)
}
