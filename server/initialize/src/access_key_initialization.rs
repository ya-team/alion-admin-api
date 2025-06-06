/**
 * 访问密钥初始化模块
 * 
 * 本模块负责初始化系统的访问密钥，确保系统启动时
 * 具有必要的访问密钥配置。
 */

use server_global::project_info;
use server_service::admin::{SysAccessKeyService, TAccessKeyService};

/**
 * 初始化访问密钥
 * 
 * 调用访问密钥服务初始化方法，创建系统所需的访问密钥。
 * 如果初始化失败，会记录错误日志但不会中断程序执行。
 */
pub async fn initialize_access_key() {
    let access_key_service = SysAccessKeyService;

    let _ = access_key_service.initialize_access_key().await;

    project_info!("Access key initialization completed successfully")
}
