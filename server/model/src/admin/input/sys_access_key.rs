/**
 * 访问密钥相关输入参数定义
 * 
 * 包含访问密钥分页请求与创建输入结构体。
 */

use serde::{Deserialize, Serialize};
use server_core::web::page::PageRequest;
use validator::Validate;

use crate::admin::entities::sea_orm_active_enums::Status;

/**
 * 访问密钥分页请求参数
 * 
 * 用于分页查询访问密钥。
 */
#[derive(Debug, Serialize, Deserialize)]
pub struct AccessKeyPageRequest {
    #[serde(flatten)]
    pub page_details: PageRequest,
    pub keywords: Option<String>,
}

/**
 * 访问密钥创建输入参数
 * 
 * 用于创建访问密钥。
 */
#[derive(Deserialize, Validate)]
pub struct AccessKeyInput {
    pub domain: String,
    pub status: Status,
    #[validate(length(max = 200, message = "Description must not exceed 200 characters"))]
    pub description: Option<String>,
}

/**
 * 访问密钥创建输入类型别名
 */
pub type CreateAccessKeyInput = AccessKeyInput;
