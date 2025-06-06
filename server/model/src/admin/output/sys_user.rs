/**
 * 用户相关输出参数定义
 * 
 * 包含用户信息、带域和组织信息的用户信息等输出结构体。
 */

use chrono::NaiveDateTime;
use sea_orm::FromQueryResult;
use serde::Serialize;

use crate::admin::entities::{sea_orm_active_enums::Status, sys_user::Model as SysUserModel};

/**
 * 用户信息输出参数
 * 
 * 用于返回用户的基本信息，不包含密码等敏感信息。
 */
#[derive(Debug, FromQueryResult, Clone)]
pub struct UserWithDomainAndOrgOutput {
    /** 用户ID */
    pub id: String,
    pub domain: String,
    pub username: String,
    pub password: String,
    pub nick_name: String,
    pub avatar: Option<String>,
    pub domain_code: String,
    pub domain_name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserWithoutPassword {
    pub id: String,
    pub domain: String,
    pub username: String,
    pub nick_name: String,
    pub avatar: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub status: Status,
    pub created_at: NaiveDateTime,
    pub created_by: String,
    pub updated_at: Option<NaiveDateTime>,
    pub updated_by: Option<String>,
}

impl From<SysUserModel> for UserWithoutPassword {
    fn from(model: SysUserModel) -> Self {
        Self {
            id: model.id,
            domain: model.domain,
            username: model.username,
            nick_name: model.nick_name,
            avatar: model.avatar,
            email: model.email,
            phone_number: model.phone_number,
            status: model.status,
            created_at: model.created_at,
            created_by: model.created_by,
            updated_at: model.updated_at,
            updated_by: model.updated_by,
        }
    }
}
