/**
 * IP地址解析初始化模块
 * 
 * 本模块负责初始化IP地址解析服务，使用xdb数据库
 * 进行IP地址到地理位置的映射。
 */

use std::error::Error;

use xdb::searcher;

use crate::project_info;

/**
 * 初始化IP地址解析数据库
 * 
 * 加载xdb数据库文件，初始化IP地址解析服务。
 * 
 * # 返回
 * - 成功：返回Ok(())
 * - 失败：返回错误信息
 * 
 * # 处理流程
 * 1. 异步加载xdb数据库文件
 * 2. 初始化IP地址解析器
 * 3. 记录初始化结果
 */
pub async fn init_xdb() -> Result<(), Box<dyn Error>> {
    tokio::task::spawn_blocking(|| {
        searcher::searcher_init(Some("server/resources/ip2region.xdb".to_string()));
    })
    .await?;
    project_info!("XDB initialized successfully");
    Ok(())
}
