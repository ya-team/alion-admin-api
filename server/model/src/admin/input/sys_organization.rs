/**
 * 组织相关输入参数定义
 * 
 * 包含组织分页请求结构体。
 */

use serde::{Deserialize, Serialize};
use server_core::web::page::PageRequest;

/**
 * 组织分页请求参数
 * 
 * 用于分页查询组织。
 */
#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizationPageRequest {
    #[serde(flatten)]
    pub page_details: PageRequest,
    pub keywords: Option<String>,
}
