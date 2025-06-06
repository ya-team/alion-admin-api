/**
 * 角色相关输入参数定义
 * 
 * 包含角色分页、创建、更新等输入结构体。
 * 用于管理后台的角色管理功能，包括：
 * - 角色列表分页查询
 * - 创建新角色
 * - 更新现有角色
 */

use serde::{Deserialize, Serialize};
use server_core::web::page::PageRequest;
use validator::Validate;

use crate::admin::entities::sea_orm_active_enums::Status;

/**
 * 角色分页请求参数
 * 
 * 用于分页查询角色列表，支持：
 * - 基础分页参数（页码、每页数量等）
 * - 关键字搜索
 * - 状态筛选
 */
#[derive(Debug, Serialize, Deserialize)]
pub struct RolePageRequest {
    #[serde(flatten)]
    pub page_details: PageRequest,
    #[serde(default)]
    pub keywords: Option<String>,
    #[serde(default)]
    pub status: Option<Status>,
}

/**
 * 角色创建/更新输入参数
 * 
 * 用于创建新角色或更新现有角色信息。
 * 包含角色的基本信息，如：
 * - 父级角色ID
 * - 角色编码
 * - 角色名称
 * - 状态
 * - 描述信息
 */
#[derive(Deserialize, Validate)]
pub struct RoleInput {
    /** 父级角色ID */
    pub pid: String,
    /** 角色编码，1-50个字符 */
    #[validate(length(
        min = 1,
        max = 50,
        message = "Code must be between 1 and 50 characters"
    ))]
    pub code: String,
    /** 角色名称，1-50个字符 */
    #[validate(length(
        min = 1,
        max = 50,
        message = "Name must be between 1 and 50 characters"
    ))]
    pub name: String,
    /** 角色状态 */
    pub status: Status,
    /** 角色描述，最多200个字符 */
    #[validate(length(max = 200, message = "Description must not exceed 200 characters"))]
    pub description: Option<String>,
}

/**
 * 角色创建输入类型别名
 * 
 * 用于创建新角色时的输入参数类型。
 */
pub type CreateRoleInput = RoleInput;

/**
 * 角色更新输入参数
 * 
 * 用于更新现有角色信息。
 * 包含：
 * - 角色ID
 * - 角色详细信息（继承自RoleInput）
 */
#[derive(Deserialize, Validate)]
pub struct UpdateRoleInput {
    /** 角色ID */
    pub id: String,
    /** 角色详细信息 */
    #[serde(flatten)]
    pub role: RoleInput,
}
