/**
 * 用户相关输入参数定义
 * 
 * 包含用户分页、创建、更新等输入结构体。
 */

use serde::{Deserialize, Serialize};
use server_core::web::page::PageRequest;
use validator::Validate;

use crate::admin::entities::sea_orm_active_enums::Status;

/**
 * 用户分页请求参数
 * 
 * 用于分页查询用户。
 */
#[derive(Debug, Serialize, Deserialize)]
pub struct UserPageRequest {
    #[serde(flatten)]
    pub page_details: PageRequest,
    pub keywords: Option<String>,
}

/**
 * 用户创建/更新输入参数
 * 
 * 用于创建和更新用户。
 */
#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UserInput {
    pub domain: String,
    #[validate(length(
        min = 1,
        max = 50,
        message = "Username must be between 1 and 50 characters"
    ))]
    pub username: String,
    #[validate(length(
        min = 6,
        max = 100,
        message = "Password must be between 6 and 100 characters"
    ))]
    pub password: String,
    #[validate(length(
        min = 1,
        max = 50,
        message = "Nick name must be between 1 and 50 characters"
    ))]
    pub nick_name: String,
    pub avatar: Option<String>,
    #[validate(email(message = "Invalid email format"))]
    pub email: Option<String>,
    #[validate(length(max = 20, message = "Phone number must not exceed 20 characters"))]
    pub phone_number: Option<String>,
    pub status: Status,
}

/**
 * 用户创建输入类型别名
 */
pub type CreateUserInput = UserInput;

/**
 * 用户更新输入参数
 * 
 * 用于更新用户。
 */
#[derive(Deserialize, Validate)]
pub struct UpdateUserInput {
    pub id: String,
    #[serde(flatten)]
    pub user: UserInput,
}
