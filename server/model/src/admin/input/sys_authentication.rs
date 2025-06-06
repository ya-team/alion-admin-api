/**
 * 认证/登录相关输入参数定义
 * 
 * 包含登录请求输入结构体。
 */

use serde::Deserialize;
use validator::Validate;

/**
 * 登录请求输入参数
 * 
 * 用于用户登录接口。
 */
#[derive(Deserialize, Validate)]
pub struct LoginInput {
    #[validate(length(min = 5, message = "Username cannot be empty"))]
    pub username: String,
    #[validate(length(min = 6, message = "Password cannot be empty"))]
    pub password: String,
}
