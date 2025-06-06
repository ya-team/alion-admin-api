/**
 * 认证模块
 * 
 * 该模块提供了用户认证和授权相关的功能，包括：
 * - JWT令牌的Claims处理：管理JWT令牌中的用户信息
 * - 用户信息管理：处理用户基本信息和角色
 * - 用户认证中间件：提供请求认证和用户信息提取
 * 
 * # 主要组件
 * 
 * ## Claims
 * JWT令牌的声明部分，包含用户身份和权限信息：
 * - 标准JWT字段（sub, exp, iss, aud等）
 * - 用户信息（用户名、角色、域等）
 * 
 * ## User
 * 用户信息结构，用于在应用内部表示用户：
 * - 用户标识（ID、用户名）
 * - 权限信息（角色列表）
 * - 组织信息（域、组织）
 * 
 * # 使用示例
 * 
 * 
 * // 创建用户Claims
 * let claims = Claims::new(
 *     "user123".to_string(),
 *     "api".to_string(),
 *     "john_doe".to_string(),
 *     vec!["admin".to_string()],
 *     "example.com".to_string(),
 *     Some("org1".to_string())
 * );
 * 
 * // 转换为User实例
 * let user = User::from(claims);
 */

use async_trait::async_trait;
use axum::{
    extract::{FromRequest, Request},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

use crate::web::res::Res;

/**
 * JWT令牌的Claims结构体
 * 
 * 包含JWT令牌中的所有声明信息，包括标准JWT字段和自定义用户信息。
 * 实现了序列化和反序列化，支持JWT令牌的编码和解码。
 * 
 * # 字段说明
 * 
 * ## 标准JWT字段
 * * `sub`: 主题（用户ID）
 * * `exp`: 过期时间
 * * `iss`: 签发者
 * * `aud`: 接收者
 * * `iat`: 签发时间
 * * `nbf`: 生效时间
 * * `jti`: JWT ID
 * 
 * ## 自定义字段
 * * `username`: 用户名
 * * `role`: 用户角色列表
 * * `domain`: 用户所属域
 * * `org`: 用户所属组织
 */
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// 主题（用户ID）
    sub: String,

    /// 过期时间
    exp: Option<usize>,
    /// 签发者
    iss: Option<String>,
    /// 接收者
    aud: String,
    /// 签发时间
    iat: Option<usize>,
    /// 生效时间
    nbf: Option<usize>,
    /// JWT ID
    jti: Option<String>,

    /// 用户名
    username: String,
    /// 用户角色列表
    role: Vec<String>,
    /// 用户所属域
    domain: String,
    /// 用户所属组织
    org: Option<String>,
}

impl Claims {
    /**
     * 创建新的Claims实例
     * 
     * 创建一个包含基本用户信息的Claims实例。
     * 其他JWT字段（exp, iss, iat等）可以通过相应的方法设置。
     * 
     * # 参数
     * * `sub` - 主题（用户ID）
     * * `aud` - 接收者
     * * `username` - 用户名
     * * `role` - 用户角色列表
     * * `domain` - 用户所属域
     * * `org` - 用户所属组织
     * 
     * # 返回
     * * `Self` - 新的Claims实例
     */
    pub fn new(
        sub: String,
        aud: String,
        username: String,
        role: Vec<String>,
        domain: String,
        org: Option<String>,
    ) -> Self {
        Self {
            sub,
            exp: None,
            iss: None,
            aud,
            iat: None,
            nbf: None,
            jti: None,
            username,
            role,
            domain,
            org,
        }
    }

    /**
     * 设置过期时间
     * 
     * 设置JWT令牌的过期时间（Unix时间戳）。
     * 
     * # 参数
     * * `exp` - 过期时间（Unix时间戳）
     */
    pub fn set_exp(&mut self, exp: usize) {
        self.exp = Some(exp);
    }

    /**
     * 设置签发者
     * 
     * 设置JWT令牌的签发者。
     * 
     * # 参数
     * * `iss` - 签发者标识
     */
    pub fn set_iss(&mut self, iss: String) {
        self.iss = Some(iss);
    }

    /**
     * 设置签发时间
     * 
     * 设置JWT令牌的签发时间（Unix时间戳）。
     * 
     * # 参数
     * * `iat` - 签发时间（Unix时间戳）
     */
    pub fn set_iat(&mut self, iat: usize) {
        self.iat = Some(iat);
    }

    /**
     * 设置生效时间
     * 
     * 设置JWT令牌的生效时间（Unix时间戳）。
     * 
     * # 参数
     * * `nbf` - 生效时间（Unix时间戳）
     */
    pub fn set_nbf(&mut self, nbf: usize) {
        self.nbf = Some(nbf);
    }

    /**
     * 设置JWT ID
     * 
     * 设置JWT令牌的唯一标识符。
     * 
     * # 参数
     * * `jti` - JWT ID
     */
    pub fn set_jti(&mut self, jti: String) {
        self.jti = Some(jti);
    }
}

/**
 * 用户信息结构体
 * 
 * 用于在应用内部表示用户信息，包含用户的基本信息和权限信息。
 * 实现了序列化和反序列化，支持用户信息的存储和传输。
 * 
 * # 字段说明
 * 
 * * `user_id`: 用户唯一标识
 * * `username`: 用户名称
 * * `role`: 用户角色列表
 * * `domain`: 用户所属域
 * * `org`: 用户所属组织
 */
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    /// 用户ID
    user_id: String,
    /// 用户名
    username: String,
    /// 用户角色列表
    role: Vec<String>,
    /// 用户所属域
    domain: String,
    /// 用户所属组织
    org: Option<String>,
}

impl User {
    /**
     * 创建新的用户实例
     * 
     * 创建一个包含基本用户信息的User实例。
     * 角色列表初始化为空，组织信息初始化为None。
     * 
     * # 参数
     * * `user_id` - 用户ID
     * * `username` - 用户名
     * * `domain` - 用户所属域
     * 
     * # 返回
     * * `Self` - 新的用户实例
     */
    pub fn new(user_id: String, username: String, domain: String) -> Self {
        Self {
            user_id,
            username,
            role: Vec::new(),
            domain,
            org: None,
        }
    }

    /**
     * 获取用户ID
     * 
     * # 返回
     * * `String` - 用户ID的克隆
     */
    pub fn user_id(&self) -> String {
        self.user_id.clone()
    }

    /**
     * 获取用户名
     * 
     * # 返回
     * * `String` - 用户名的克隆
     */
    pub fn username(&self) -> String {
        self.username.clone()
    }

    /**
     * 获取用户角色列表
     * 
     * # 返回
     * * `Vec<String>` - 用户角色列表的克隆
     */
    pub fn subject(&self) -> Vec<String> {
        self.role.clone()
    }

    /**
     * 获取用户所属域
     * 
     * # 返回
     * * `String` - 用户所属域的克隆
     */
    pub fn domain(&self) -> String {
        self.domain.to_string()
    }
}

impl From<Claims> for User {
    /**
     * 从Claims创建用户实例
     * 
     * 将JWT Claims中的用户信息转换为User实例。
     * 
     * # 参数
     * * `claims` - JWT Claims
     * 
     * # 返回
     * * `Self` - 新的用户实例
     */
    fn from(claims: Claims) -> Self {
        User {
            user_id: claims.sub,
            username: claims.username,
            role: claims.role,
            domain: claims.domain,
            org: claims.org,
        }
    }
}

#[async_trait]
impl<S> FromRequest<S> for User
where
    S: Send + Sync + 'static,
{
    type Rejection = Res<String>;

    /**
     * 从请求中提取用户信息
     * 
     * 从请求的扩展中获取用户信息。
     * 如果用户信息不存在，返回401 Unauthorized错误。
     * 
     * # 参数
     * * `req` - HTTP请求
     * * `_state` - 应用状态
     * 
     * # 返回
     * * `Result<Self, Self::Rejection>` - 成功返回用户信息，失败返回错误响应
     */
    fn from_request(
        req: Request,
        _state: &S,
    ) -> impl std::future::Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            req.extensions()
                .get::<User>()
                .cloned()
                .ok_or_else(|| Res::new_error(StatusCode::UNAUTHORIZED.as_u16(), "Unauthorized"))
        }
    }
}
