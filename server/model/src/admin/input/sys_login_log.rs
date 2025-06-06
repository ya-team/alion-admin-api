/**
 * 登录日志相关输入参数定义
 * 
 * 包含登录日志分页请求结构体。
 */

use serde::{Deserialize, Serialize};
use server_core::web::page::PageRequest;

/**
 * 登录日志分页请求参数
 * 
 * 用于分页查询登录日志。
 */
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginLogPageRequest {
    #[serde(flatten)]
    pub page_details: PageRequest,
    pub keywords: Option<String>,
}
