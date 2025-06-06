/**
 * 授权相关输入参数定义
 * 
 * 包含分配权限、分配路由、分配用户等DTO。
 */

use serde::{Deserialize, Serialize};
use validator::Validate;

/**
 * 分配权限DTO
 * 
 * 用于为角色分配权限。
 */
#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AssignPermissionDto {
    #[validate(length(min = 1, message = "domain cannot be empty"))]
    pub domain: String,

    #[validate(length(min = 1, message = "Role ID cannot be empty"))]
    pub role_id: String,

    #[validate(length(min = 1, message = "Permissions array cannot be empty"))]
    pub permissions: Vec<String>,
}

/**
 * 分配路由DTO
 * 
 * 用于为角色分配路由。
 */
#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AssignRouteDto {
    #[validate(length(min = 1, message = "domain cannot be empty"))]
    pub domain: String,

    #[validate(length(min = 1, message = "Role ID cannot be empty"))]
    pub role_id: String,

    #[validate(length(min = 1, message = "Routes array cannot be empty"))]
    pub route_ids: Vec<i32>,
}

/**
 * 分配用户DTO
 * 
 * 用于为角色分配用户。
 */
#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AssignUserDto {
    #[validate(length(min = 1, message = "Role ID cannot be empty"))]
    pub role_id: String,

    #[validate(length(min = 1, message = "Users array cannot be empty"))]
    pub user_ids: Vec<String>,
}
