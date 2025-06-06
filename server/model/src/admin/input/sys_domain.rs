/**
 * 域名相关输入参数定义
 * 
 * 包含域名分页、创建、更新等输入结构体。
 */

use serde::{Deserialize, Serialize};
use server_core::web::page::PageRequest;
use validator::Validate;

/**
 * 域名分页请求参数
 * 
 * 用于分页查询域名。
 */
#[derive(Debug, Serialize, Deserialize)]
pub struct DomainPageRequest {
    #[serde(flatten)]
    pub page_details: PageRequest,
    pub keywords: Option<String>,
}

/**
 * 域名创建/更新输入参数
 * 
 * 用于创建和更新域名。
 */
#[derive(Deserialize, Validate)]
pub struct DomainInput {
    #[validate(length(
        min = 1,
        max = 50,
        message = "Code must be between 1 and 50 characters"
    ))]
    pub code: String,
    #[validate(length(
        min = 1,
        max = 100,
        message = "Name must be between 1 and 100 characters"
    ))]
    pub name: String,
    #[validate(length(max = 500, message = "Description must not exceed 500 characters"))]
    pub description: Option<String>,
}

/**
 * 域名创建输入类型别名
 */
pub type CreateDomainInput = DomainInput;

/**
 * 域名更新输入参数
 * 
 * 用于更新域名。
 */
#[derive(Deserialize, Validate)]
pub struct UpdateDomainInput {
    pub id: String,
    #[serde(flatten)]
    pub domain: DomainInput,
}
