/**
 * 配置初始化模块
 * 
 * 本模块负责从配置文件加载系统配置信息，
 * 包括数据库、Redis、JWT等配置项。
 */

use crate::{project_error, project_info};

/**
 * 初始化系统配置
 * 
 * 从指定的配置文件路径加载系统配置。
 * 
 * # 参数
 * - file_path: 配置文件路径
 * 
 * # 处理流程
 * 1. 读取配置文件
 * 2. 解析配置内容
 * 3. 初始化全局配置
 * 4. 记录初始化结果
 */
pub async fn initialize_config(file_path: &str) {
    match server_config::init_from_file(file_path).await {
        Ok(_) => {
            project_info!("Configuration initialized successfully from: {}", file_path)
        },
        Err(e) => {
            project_error!("Failed to initialize config from {}: {:?}", file_path, e);
        },
    }
}
