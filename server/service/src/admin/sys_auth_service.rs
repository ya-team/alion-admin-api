/** 系统认证服务模块
 * 
 * 该模块提供了用户认证相关的核心功能，包括：
 * - 用户登录认证
 * - 用户角色和权限验证
 * - 用户路由获取
 * - 登录事件处理
 * 
 * 主要组件
 * --------
 * 
 * 核心接口
 * --------
 * * `TAuthService`: 认证服务 trait，定义了认证相关的核心接口
 * * `SysAuthService`: 认证服务实现，提供了具体的认证逻辑
 * 
 * 事件处理
 * --------
 * * `AuthEvent`: 认证事件，用于处理登录相关的异步事件
 * * `auth_login_listener`: 登录事件监听器
 * * `jwt_created_listener`: JWT创建事件监听器
 * 
 * 辅助功能
 * --------
 * * `generate_auth_output`: 生成认证输出
 * * `send_auth_event`: 发送认证事件
 * * `handle_auth_event`: 处理认证事件
 * 
 * 使用示例
 * --------
 * /* 创建认证服务实例
 *  * let auth_service = SysAuthService;
 *  * 
 *  * // 执行密码登录
 *  * let output = auth_service.pwd_login(
 *  *     db,
 *  *     LoginInput {
 *  *         username: "admin".to_string(),
 *  *         password: "password".to_string(),
 *  *     },
 *  *     LoginContext {
 *  *         client_ip: "127.0.0.1".to_string(),
 *  *         client_port: Some(8080),
 *  *         address: "localhost".to_string(),
 *  *         user_agent: "Mozilla/5.0".to_string(),
 *  *         request_id: "req-123".to_string(),
 *  *         audience: Audience::Admin,
 *  *         login_type: "password".to_string(),
 *  *         domain: "example.com".to_string(),
 *  *     },
 *  * ).await?;
 *  */
 */

use std::{any::Any, sync::Arc};

use async_trait::async_trait;
#[allow(unused_imports)]
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, JoinType, QueryFilter, QueryOrder, QuerySelect,
    RelationTrait, Set,
};
use server_constant::definition::{Audience};
use server_core::web::{
    auth::Claims,
    jwt::{JwtUtils},
};
use server_global::{project_error, project_info};
use server_model::admin::{
    entities::{
        prelude::{SysRole, SysUser},
        sea_orm_active_enums::Status,
        sys_domain::Column as SysDomainColumn,
        sys_menu::{Column as SysMenuColumn, Entity as SysMenuEntity, Model as SysMenuModel},
        sys_role::{Column as SysRoleColumn, Entity as SysRoleEntity, Relation as SysRoleRelation},
        sys_role_menu::{Column as SysRoleMenuColumn, Entity as SysRoleMenuEntity},
        sys_user::{Column as SysUserColumn, Relation as SysUserRelation},
        sys_user_role::Relation as SysUserRoleRelation,
    },
    input::LoginInput,
    output::{AuthOutput, MenuRoute, RouteMeta, UserRoute, UserWithDomainAndOrgOutput},
};
use server_utils::{SecureUtil, TreeBuilder};
use tokio::sync::mpsc;
use tracing::instrument;
use crate::admin::dto::sys_auth_dto::LoginContext;
use crate::admin::event_handlers::auth_event_handler::{AuthEvent, AuthEventHandler};
use crate::admin::errors::AuthError;

/** 用户查询宏
 * 
 * 用于构建包含域和组织信息的用户查询，包括：
 * - 用户基本信息（ID、用户名、密码等）
 * - 域信息（域代码、域名称）
 * 
 * 参数
 * --------
 * * `$query` - 查询构建器
 * 
 * 返回
 * --------
 * * 构建好的查询，包含用户和域信息
 * 
 * 使用示例
 * --------
 * /* 构建用户查询
 *  * let query = select_user_with_domain_and_org_info!(SysUser::find());
 *  */
 */
macro_rules! select_user_with_domain_and_org_info {
    ($query:expr) => {{
        $query
            .select_only()
            .column_as(SysUserColumn::Id, "id")
            .column_as(SysUserColumn::Domain, "domain")
            .column_as(SysUserColumn::Username, "username")
            .column_as(SysUserColumn::Password, "password")
            .column_as(SysUserColumn::NickName, "nick_name")
            .column_as(SysUserColumn::Avatar, "avatar")
            .column_as(SysDomainColumn::Code, "domain_code")
            .column_as(SysDomainColumn::Name, "domain_name")
    }};
}

/** 认证服务 trait
 * 
 * 定义了系统认证相关的核心接口，包括：
 * - 密码登录认证
 * - 用户路由获取
 * - 用户基本信息验证
 * - 用户角色获取
 */
#[async_trait]
pub trait TAuthService: Send + Sync {
    /** 执行密码登录认证
     * 
     * 参数
     * --------
     * * `db` - 数据库连接
     * * `input` - 登录输入信息
     * * `context` - 登录上下文信息
     * 
     * 返回
     * --------
     * * `Result<AuthOutput, AuthError>` - 认证输出或错误
     */
    async fn pwd_login(
        &self,
        db: Arc<DatabaseConnection>,
        input: LoginInput,
        context: LoginContext,
    ) -> Result<AuthOutput, AuthError>;

    /** 获取用户路由信息
     * 
     * 参数
     * --------
     * * `db` - 数据库连接
     * * `role_codes` - 角色代码列表
     * * `domain` - 域代码
     * 
     * 返回
     * --------
     * * `Result<UserRoute, AuthError>` - 用户路由信息或错误
     */
    async fn get_user_routes(
        &self,
        db: Arc<DatabaseConnection>,
        role_codes: &[String],
        domain: &str,
    ) -> Result<UserRoute, AuthError>;

    /** 验证用户基本信息
     * 
     * 参数
     * --------
     * * `db` - 数据库连接
     * * `identifier` - 用户标识（用户名）
     * * `password` - 密码
     * * `domain` - 域代码
     * 
     * 返回
     * --------
     * * `Result<UserWithDomainAndOrgOutput, AuthError>` - 用户信息或错误
     */
    async fn verify_user_basic(
        &self,
        db: &Arc<DatabaseConnection>,
        identifier: &str,
        password: &str,
        domain: &str,
    ) -> Result<UserWithDomainAndOrgOutput, AuthError>;

    /** 获取用户角色列表
     * 
     * 参数
     * --------
     * * `user_id` - 用户ID
     * * `db` - 数据库连接
     * 
     * 返回
     * --------
     * * `Result<Vec<String>, AuthError>` - 角色代码列表或错误
     */
    async fn get_user_roles(
        &self,
        user_id: &str,
        db: &Arc<DatabaseConnection>,
    ) -> Result<Vec<String>, AuthError>;
}

/** 系统认证服务实现
 * 
 * 提供了认证服务的具体实现，包括：
 * - 用户登录认证
 * - 用户角色和权限验证
 * - 用户路由获取
 */
#[derive(Clone)]
pub struct SysAuthService;

impl SysAuthService {
    /** 查找第一个有效的路由路径
     * 
     * 递归遍历路由树，返回第一个非空且非根路径的路由路径
     * 
     * 参数
     * --------
     * * `routes` - 路由列表
     * 
     * 返回
     * --------
     * * `Option<String>` - 找到的路由路径，如果没有找到则返回 None
     */
    #[allow(dead_code)]
    fn find_first_valid_route(routes: &[MenuRoute]) -> Option<String> {
        for route in routes {
            if !route.path.is_empty() && route.path != "/" {
                return Some(route.path.clone());
            }
            if let Some(children) = &route.children {
                if let Some(path) = Self::find_first_valid_route(children) {
                    return Some(path);
                }
            }
        }
        None
    }

    /** 检查登录安全性
     * 
     * 执行登录相关的安全检查，包括：
     * - 登录失败次数检查
     * - IP 黑名单检查
     * - 账号锁定检查
     * - 登录时间范围检查
     * 
     * 参数
     * --------
     * * `username` - 用户名
     * * `client_ip` - 客户端IP
     * 
     * 返回
     * --------
     * * `Result<(), AuthError>` - 检查结果
     */
    async fn check_login_security(
        &self,
        _username: &str,
        _client_ip: &str,
    ) -> Result<(), AuthError> {
        // TODO: 实现登录安全检查
        // 1. 检查登录失败次数
        // 2. 检查 IP 黑名单
        // 3. 检查账号是否被锁定
        // 4. 检查是否在允许的时间范围内
        Ok(())
    }

    /** 带安全检查的密码登录
     * 
     * 在执行密码登录前先进行安全检查
     * 
     * 参数
     * --------
     * * `db` - 数据库连接
     * * `input` - 登录输入信息
     * * `context` - 登录上下文信息
     * 
     * 返回
     * --------
     * * `Result<AuthOutput, AuthError>` - 认证输出或错误
     */
    #[allow(dead_code)]
    async fn pwd_login_with_security(
        &self,
        db: Arc<DatabaseConnection>,
        input: LoginInput,
        context: LoginContext,
    ) -> Result<AuthOutput, AuthError> {
        self.check_login_security(&input.username, &context.client_ip)
            .await?;

        self.pwd_login(db, input, context).await
    }

    /** 验证用户基本信息
     * 
     * 验证用户的登录凭证，包括：
     * - 用户名和密码验证
     * - 用户状态检查
     * - 域信息验证
     * 
     * 参数
     * --------
     * * `db` - 数据库连接
     * * `identifier` - 用户标识（用户名）
     * * `password` - 密码
     * * `domain` - 域代码
     * 
     * 返回
     * --------
     * * `Result<UserWithDomainAndOrgOutput, AuthError>` - 用户信息或错误
     * 
     * 错误
     * --------
     * * `InvalidCredentials` - 用户名或密码错误
     * * `UserDisabled` - 用户已禁用
     * * `DomainNotFound` - 域不存在
     */
    #[instrument(skip(self, db, password), fields(identifier = %identifier, domain = %domain))]
    async fn verify_user_basic(
        &self,
        db: &Arc<DatabaseConnection>,
        identifier: &str,
        password: &str,
        domain: &str,
    ) -> Result<UserWithDomainAndOrgOutput, AuthError> {
        let user = select_user_with_domain_and_org_info!(SysUser::find())
            .filter(SysUserColumn::Username.eq(identifier))
            .filter(SysDomainColumn::Code.eq(domain))
            .join(JoinType::InnerJoin, SysUserRelation::SysDomain.def())
            .into_model::<UserWithDomainAndOrgOutput>()
            .one(db.as_ref())
            .await
            .map_err(|e| AuthError::DatabaseOperationFailed(e.to_string()))?
            .ok_or_else(|| AuthError::UserNotFound)?;

        //TODO validate user status and domain status

        // 验证密码
        if !SecureUtil::verify_password(password.as_bytes(), &user.password)
            .map_err(|_| AuthError::AuthenticationFailed("Password verification failed".to_string()))?
        {
            return Err(AuthError::InvalidCredentials);
        }

        Ok(user)
    }

    /** 获取用户角色列表
     * 
     * 查询用户关联的所有角色代码
     * 
     * 参数
     * --------
     * * `user_id` - 用户ID
     * * `db` - 数据库连接
     * 
     * 返回
     * --------
     * * `Result<Vec<String>, AuthError>` - 角色代码列表或错误
     */
    #[instrument(skip(self, db), fields(user_id = %user_id))]
    async fn get_user_roles(
        &self,
        user_id: &str,
        db: &Arc<DatabaseConnection>,
    ) -> Result<Vec<String>, AuthError> {
        SysRole::find()
            .join(JoinType::InnerJoin, SysRoleRelation::SysUserRole.def())
            .join(JoinType::InnerJoin, SysUserRoleRelation::SysUser.def())
            .filter(SysUserColumn::Id.eq(user_id))
            .select_only()
            .column(SysRoleColumn::Code)
            .into_tuple()
            .all(db.as_ref())
            .await
            .map_err(|e| AuthError::DatabaseOperationFailed(e.to_string()))
    }

    /** 获取用户路由信息
     * 
     * 根据用户角色获取可访问的路由信息，包括：
     * - 菜单路由
     * - 路由元数据
     * - 权限信息
     * 
     * 参数
     * --------
     * * `db` - 数据库连接
     * * `role_codes` - 角色代码列表
     * * `domain` - 域代码
     * 
     * 返回
     * --------
     * * `Result<UserRoute, AuthError>` - 用户路由信息或错误
     */
    #[instrument(skip(self, db, role_codes), fields(domain = %domain))]
    #[allow(dead_code)]
    async fn get_user_routes(
        &self,
        db: Arc<DatabaseConnection>,
        role_codes: &[String],
        domain: &str,
    ) -> Result<UserRoute, AuthError> {
        if role_codes.is_empty() {
            return Ok(UserRoute {
                routes: vec![],
                home: "/home".to_string(),
            });
        }

        // 获取角色关联的菜单ID
        let menu_ids = SysRoleMenuEntity::find()
            .select_only()
            .column(SysRoleMenuColumn::MenuId)
            .join_rev(
                JoinType::InnerJoin,
                SysRoleEntity::has_many(SysRoleMenuEntity).into(),
            )
            .filter(SysRoleColumn::Code.is_in(role_codes.to_vec()))
            .filter(SysRoleMenuColumn::Domain.eq(domain))
            .distinct()
            .into_tuple::<i32>()
            .all(db.as_ref())
            .await
            .map_err(|e| AuthError::DatabaseOperationFailed(e.to_string()))?;

        // 查询菜单信息
        let menus = SysMenuEntity::find()
            .filter(SysMenuColumn::Id.is_in(menu_ids))
            .filter(SysMenuColumn::Status.eq(Status::Enabled))
            .order_by_asc(SysMenuColumn::Sequence)
            .into_model::<SysMenuModel>()
            .all(db.as_ref())
            .await
            .map_err(|e| AuthError::DatabaseOperationFailed(e.to_string()))?;

        // 构建菜单路由
        let menu_routes: Vec<MenuRoute> = menus
            .into_iter()
            .map(|menu| MenuRoute {
                name: menu.route_name,
                path: menu.route_path,
                component: menu.component,
                meta: RouteMeta {
                    title: menu.menu_name,
                    i18n_key: menu.i18n_key,
                    keep_alive: menu.keep_alive,
                    constant: menu.constant,
                    icon: menu.icon,
                    order: menu.sequence,
                    href: menu.href,
                    hide_in_menu: menu.hide_in_menu,
                    active_menu: menu.active_menu,
                    multi_tab: menu.multi_tab,
                },
                children: Some(vec![]),
                id: menu.id,
                pid: menu.pid,
            })
            .collect();

        let menu_routes_ref = menu_routes.clone();

        // 构建路由树
        let routes = TreeBuilder::build(
            menu_routes,
            |route| route.name.clone(),
            |route| {
                if route.pid == "0" {
                    None
                } else {
                    menu_routes_ref
                        .iter()
                        .find(|m| m.id.to_string() == route.pid)
                        .map(|m| m.name.clone())
                }
            },
            |route| route.meta.order,
            |route, children| {
                route.children = Some(children);
            },
        );

        let home = "home".to_string();

        Ok(UserRoute { routes, home })
    }
}

#[async_trait]
impl TAuthService for SysAuthService {
    async fn pwd_login(
        &self,
        db: Arc<DatabaseConnection>,
        input: LoginInput,
        context: LoginContext,
    ) -> Result<AuthOutput, AuthError> {
        // 验证用户信息
        let user = self.verify_user_basic(&db, &input.username, &input.password, &context.domain).await?;

        // 获取用户角色
        let role_codes = self.get_user_roles(&user.id, &db).await?;

        // 生成认证输出
        let auth_output = generate_auth_output(
            user.id,
            user.username,
            role_codes,
            user.domain_code,
            None,
            context.audience,
        ).await?;

        Ok(auth_output)
    }

    async fn get_user_routes(
        &self,
        db: Arc<DatabaseConnection>,
        role_codes: &[String],
        domain: &str,
    ) -> Result<UserRoute, AuthError> {
        if role_codes.is_empty() {
            return Ok(UserRoute {
                routes: vec![],
                home: "/home".to_string(),
            });
        }

        // 获取角色关联的菜单ID
        let menu_ids = SysRoleMenuEntity::find()
            .select_only()
            .column(SysRoleMenuColumn::MenuId)
            .join_rev(
                JoinType::InnerJoin,
                SysRoleEntity::has_many(SysRoleMenuEntity).into(),
            )
            .filter(SysRoleColumn::Code.is_in(role_codes.to_vec()))
            .filter(SysRoleMenuColumn::Domain.eq(domain))
            .distinct()
            .into_tuple::<i32>()
            .all(db.as_ref())
            .await
            .map_err(|e| AuthError::DatabaseOperationFailed(e.to_string()))?;

        // 查询菜单信息
        let menus = SysMenuEntity::find()
            .filter(SysMenuColumn::Id.is_in(menu_ids))
            .filter(SysMenuColumn::Status.eq(Status::Enabled))
            .order_by_asc(SysMenuColumn::Sequence)
            .into_model::<SysMenuModel>()
            .all(db.as_ref())
            .await
            .map_err(|e| AuthError::DatabaseOperationFailed(e.to_string()))?;

        // 构建菜单路由
        let menu_routes: Vec<MenuRoute> = menus
            .into_iter()
            .map(|menu| MenuRoute {
                name: menu.route_name,
                path: menu.route_path,
                component: menu.component,
                meta: RouteMeta {
                    title: menu.menu_name,
                    i18n_key: menu.i18n_key,
                    keep_alive: menu.keep_alive,
                    constant: menu.constant,
                    icon: menu.icon,
                    order: menu.sequence,
                    href: menu.href,
                    hide_in_menu: menu.hide_in_menu,
                    active_menu: menu.active_menu,
                    multi_tab: menu.multi_tab,
                },
                children: Some(vec![]),
                id: menu.id,
                pid: menu.pid,
            })
            .collect();

        let menu_routes_ref = menu_routes.clone();

        // 构建路由树
        let routes = TreeBuilder::build(
            menu_routes,
            |route| route.name.clone(),
            |route| {
                if route.pid == "0" {
                    None
                } else {
                    menu_routes_ref
                        .iter()
                        .find(|m| m.id.to_string() == route.pid)
                        .map(|m| m.name.clone())
                }
            },
            |route| route.meta.order,
            |route, children| {
                route.children = Some(children);
            },
        );

        Ok(UserRoute { 
            routes, 
            home: "home".to_string() 
        })
    }

    async fn verify_user_basic(
        &self,
        db: &Arc<DatabaseConnection>,
        identifier: &str,
        password: &str,
        domain: &str,
    ) -> Result<UserWithDomainAndOrgOutput, AuthError> {
        let user = select_user_with_domain_and_org_info!(SysUser::find())
            .filter(SysUserColumn::Username.eq(identifier))
            .filter(SysDomainColumn::Code.eq(domain))
            .join(JoinType::InnerJoin, SysUserRelation::SysDomain.def())
            .into_model::<UserWithDomainAndOrgOutput>()
            .one(db.as_ref())
            .await
            .map_err(|e| AuthError::DatabaseOperationFailed(e.to_string()))?
            .ok_or_else(|| AuthError::UserNotFound)?;

        // 验证密码
        if !SecureUtil::verify_password(password.as_bytes(), &user.password)
            .map_err(|_| AuthError::AuthenticationFailed("Password verification failed".to_string()))?
        {
            return Err(AuthError::InvalidCredentials);
        }

        Ok(user)
    }

    async fn get_user_roles(
        &self,
        user_id: &str,
        db: &Arc<DatabaseConnection>,
    ) -> Result<Vec<String>, AuthError> {
        SysRole::find()
            .join(JoinType::InnerJoin, SysRoleRelation::SysUserRole.def())
            .join(JoinType::InnerJoin, SysUserRoleRelation::SysUser.def())
            .filter(SysUserColumn::Id.eq(user_id))
            .select_only()
            .column(SysRoleColumn::Code)
            .into_tuple()
            .all(db.as_ref())
            .await
            .map_err(|e| AuthError::DatabaseOperationFailed(e.to_string()))
    }
}

/** 发送认证事件
 * 
 * 将认证事件发送到事件通道，用于异步处理
 * 
 * 参数
 * --------
 * * `sender` - 事件发送器
 * * `auth_event` - 认证事件
 * 
 * 返回
 * --------
 * * `Result<(), AuthError>` - 发送结果
 */
#[instrument(skip(sender, auth_event))]
#[allow(dead_code)]
async fn send_auth_event(
    sender: mpsc::UnboundedSender<Box<dyn std::any::Any + Send>>,
    auth_event: AuthEvent,
) -> Result<(), AuthError> {
    sender
        .send(Box::new(auth_event))
        .map_err(AuthError::SendError)?;
    Ok(())
}

/** 生成认证输出
 * 
 * 根据用户信息和角色生成认证输出，包括：
 * - 访问令牌
 * - 刷新令牌
 * - 用户信息
 * - 角色信息
 * 
 * 参数
 * --------
 * * `user_id` - 用户ID
 * * `username` - 用户名
 * * `role_codes` - 角色代码列表
 * * `domain_code` - 域代码
 * * `organization_name` - 组织名称（可选）
 * * `audience` - 认证受众
 * 
 * 返回
 * --------
 * * `Result<AuthOutput, AuthError>` - 认证输出或错误
 */
#[instrument(skip(role_codes))]
pub async fn generate_auth_output(
    user_id: String,
    username: String,
    role_codes: Vec<String>,
    domain_code: String,
    organization_name: Option<String>,
    audience: Audience,
) -> Result<AuthOutput, AuthError> {
    let claims = Claims::new(
        user_id,
        audience.as_str().to_string(),
        username,
        role_codes,
        domain_code,
        organization_name,
    );

    let token = JwtUtils::generate_token(&claims)
        .await
        .map_err(|e| AuthError::JwtGenerationFailed(e.to_string()))?;

    let refresh_token = JwtUtils::generate_token(&claims)
        .await
        .map_err(|e| AuthError::JwtGenerationFailed(e.to_string()))?;

    Ok(AuthOutput {
        token,
        refresh_token,
    })
}

/** 登录事件监听器
 * 
 * 监听并处理登录相关事件，包括：
 * - 登录日志记录
 * - 访问令牌管理
 * 
 * 参数
 * --------
 * * `rx` - 事件接收器
 */
pub async fn auth_login_listener(
    mut rx: tokio::sync::mpsc::UnboundedReceiver<Box<dyn Any + Send>>,
) {
    while let Some(event) = rx.recv().await {
        if let Some(auth_event) = event.downcast_ref::<AuthEvent>() {
            if let Err(e) = handle_auth_event(auth_event).await {
                project_error!("Failed to handle AuthEvent: {:?}", e);
            }
        }
    }
}

/** 处理认证事件
 * 
 * 处理具体的认证事件，包括：
 * - 登录日志记录
 * - 访问令牌管理
 * 
 * 参数
 * --------
 * * `auth_event` - 认证事件
 * 
 * 返回
 * --------
 * * `Result<(), AuthError>` - 处理结果
 */
#[instrument(skip(auth_event))]
async fn handle_auth_event(auth_event: &AuthEvent) -> Result<(), AuthError> {
    AuthEventHandler::handle_login(AuthEvent {
        user_id: auth_event.user_id.clone(),
        username: auth_event.username.clone(),
        domain: auth_event.domain.clone(),
        access_token: auth_event.access_token.clone(),
        refresh_token: auth_event.refresh_token.clone(),
        client_ip: auth_event.client_ip.clone(),
        address: auth_event.address.clone(),
        client_port: auth_event.client_port,
        user_agent: auth_event.user_agent.clone(),
        request_id: auth_event.request_id.clone(),
        login_type: auth_event.login_type.clone(),
    })
    .await
    .map_err(|e| AuthError::LoginHandlerError(format!("{:?}", e)))
}

/** JWT创建事件监听器
 * 
 * 监听并处理JWT创建事件，用于：
 * - 令牌创建记录
 * - 令牌状态管理
 * 
 * 参数
 * --------
 * * `rx` - 事件接收器
 */
pub async fn jwt_created_listener(mut rx: tokio::sync::mpsc::UnboundedReceiver<String>) {
    while let Some(jwt) = rx.recv().await {
        project_info!("JWT created: {}", jwt);
        // TODO: Consider storing the token into the database
    }
}
