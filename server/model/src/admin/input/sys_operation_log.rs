/**
 * 操作日志相关输入参数定义
 * 
 * 包含操作日志分页请求结构体。
 */

use serde::{Deserialize, Serialize};
use server_core::web::page::PageRequest;

/**
 * 操作日志分页请求参数
 * 
 * 用于分页查询操作日志。
 */
#[derive(Debug, Serialize, Deserialize)]
pub struct OperationLogPageRequest {
    #[serde(flatten)]
    pub page_details: PageRequest,
    pub keywords: Option<String>,
}
