/**
 * 接口相关输入参数定义
 * 
 * 包含接口分页请求结构体。
 */

use serde::{Deserialize, Serialize};
use server_core::web::page::PageRequest;

/**
 * 接口分页请求参数
 * 
 * 用于分页查询接口。
 */
#[derive(Debug, Serialize, Deserialize)]
pub struct EndpointPageRequest {
    #[serde(flatten)]
    pub page_details: PageRequest,
    pub keywords: Option<String>,
}
