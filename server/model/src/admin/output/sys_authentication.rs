/**
 * 认证相关输出参数定义
 * 
 * 包含认证结果、用户信息和路由信息的输出结构体。
 */

use serde::Serialize;

use super::MenuRoute;

/**
 * 认证输出参数
 * 
 * 用于返回用户登录认证的结果信息。
 */
#[derive(Clone, Debug, Serialize)]
pub struct AuthOutput {
    /** 访问令牌 */
    pub token: String,
    // 为了复用alion-admin-nestjs前端,暂时弃用
    // pub access_token: String,
    /** 刷新令牌 */
    pub refresh_token: String,
}

/**
 * 用户信息输出参数
 * 
 * 用于返回用户的基本信息。
 */
#[derive(Debug, Serialize)]
pub struct UserInfoOutput {
    /** 用户ID */
    #[serde(rename = "userId")]
    pub user_id: String,
    /** 用户名 */
    #[serde(rename = "userName")]
    pub user_name: String,
    /** 用户角色列表 */
    pub roles: Vec<String>,
}

/**
 * 用户路由输出参数
 * 
 * 用于返回用户可访问的路由信息。
 */
#[derive(Debug, Serialize)]
pub struct UserRoute {
    /** 路由列表 */
    pub routes: Vec<MenuRoute>,
    /** 首页路由 */
    pub home: String,
}
