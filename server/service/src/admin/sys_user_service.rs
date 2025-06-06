//! 系统用户服务模块
//! 
//! 该模块提供了用户管理相关的核心功能，包括：
//! - 用户CRUD操作
//! - 用户分页查询
//! - 用户名唯一性检查
//! 
//! # 主要组件
//! 
//! ## 核心接口
//! * `TUserService`: 用户服务 trait，定义了用户管理相关的核心接口
//! * `SysUserService`: 用户服务实现，提供了具体的用户管理逻辑
//! 
//! ## 功能特性
//! * 用户创建：支持创建新用户，包括密码加密
//! * 用户查询：支持单个查询和分页查询
//! * 用户更新：支持更新用户信息，包括用户名唯一性检查
//! * 用户删除：支持删除用户
//! 
//! # 使用示例
//! 
//! use server_service::admin::sys_user_service::*;
//! 
//! // 创建用户服务实例
//! let user_service = SysUserService;
//! 
//! // 创建新用户
//! let user = user_service.create_user(CreateUserInput {
//!     domain: "example.com".to_string(),
//!     username: "admin".to_string(),
//!     password: "password123".to_string(),
//!     nick_name: "Admin User".to_string(),
//!     avatar: None,
//!     email: Some("admin@example.com".to_string()),
//!     phone_number: Some("1234567890".to_string()),
//!     status: true,
//! }).await?;
//! 
//! // 分页查询用户
//! let users = user_service.find_paginated_users(UserPageRequest {
//!     keywords: Some("admin".to_string()),
//!     page_details: PageDetails {
//!         current: 1,
//!         size: 10,
//!     },
//! }).await?;
//! 

use async_trait::async_trait;
use chrono::Local;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, IntoActiveModel, PaginatorTrait,
    QueryFilter, Set,
};
use server_core::web::page::PaginatedData;
use server_model::admin::{
    entities::{
        prelude::SysUser,
        sys_user::{
            ActiveModel as SysUserActiveModel, Column as SysUserColumn, Model as SysUserModel,
        },
    },
    input::{CreateUserInput, UpdateUserInput, UserPageRequest},
    output::UserWithoutPassword,
};
use server_utils::SecureUtil;
use ulid::Ulid;

use super::sys_user_error::UserError;
use crate::helper::db_helper;

/// 用户服务 trait
/// 
/// 定义了用户管理相关的核心接口，包括：
/// - 用户查询（全部/分页）
/// - 用户创建
/// - 用户更新
/// - 用户删除
/// 
/// # 使用示例
/// 
/// use server_service::admin::sys_user_service::*;
/// 
/// let user_service = SysUserService;
/// 
/// // 查询所有用户
/// let users = user_service.find_all().await?;
/// 
/// // 分页查询用户
/// let users = user_service.find_paginated_users(UserPageRequest {
///     keywords: Some("admin".to_string()),
///     page_details: PageDetails {
///         current: 1,
///         size: 10,
///     },
/// }).await?;
/// 
#[async_trait]
pub trait TUserService {
    /// 查询所有用户
    /// 
    /// 返回系统中所有用户的信息（不包含密码）
    /// 
    /// # 返回
    /// * `Result<Vec<UserWithoutPassword>, UserError>` - 用户列表或错误
    async fn find_all(&self) -> Result<Vec<UserWithoutPassword>, UserError>;

    /// 分页查询用户
    /// 
    /// 根据查询条件分页获取用户列表
    /// 
    /// # 参数
    /// * `params` - 分页查询参数，包含关键字和分页信息
    /// 
    /// # 返回
    /// * `Result<PaginatedData<UserWithoutPassword>, UserError>` - 分页用户数据或错误
    async fn find_paginated_users(
        &self,
        params: UserPageRequest,
    ) -> Result<PaginatedData<UserWithoutPassword>, UserError>;

    /// 创建用户
    /// 
    /// 创建新用户，包括密码加密
    /// 
    /// # 参数
    /// * `input` - 用户创建参数
    /// 
    /// # 返回
    /// * `Result<UserWithoutPassword>, UserError>` - 创建的用户信息或错误
    async fn create_user(&self, input: CreateUserInput) -> Result<UserWithoutPassword, UserError>;

    /// 获取用户
    /// 
    /// 根据用户ID获取用户信息
    /// 
    /// # 参数
    /// * `id` - 用户ID
    /// 
    /// # 返回
    /// * `Result<UserWithoutPassword>, UserError>` - 用户信息或错误
    async fn get_user(&self, id: &str) -> Result<UserWithoutPassword, UserError>;

    /// 更新用户
    /// 
    /// 更新用户信息，包括用户名唯一性检查
    /// 
    /// # 参数
    /// * `input` - 用户更新参数
    /// 
    /// # 返回
    /// * `Result<UserWithoutPassword>, UserError>` - 更新后的用户信息或错误
    async fn update_user(&self, input: UpdateUserInput) -> Result<UserWithoutPassword, UserError>;

    /// 删除用户
    /// 
    /// 根据用户ID删除用户
    /// 
    /// # 参数
    /// * `id` - 用户ID
    /// 
    /// # 返回
    /// * `Result<(), UserError>` - 删除结果
    async fn delete_user(&self, id: &str) -> Result<(), UserError>;
}

/// 系统用户服务
/// 
/// 实现了用户管理相关的核心功能，包括：
/// - 用户CRUD操作
/// - 用户分页查询
/// - 用户名唯一性检查
/// 
/// # 使用示例
/// 
/// use server_service::admin::sys_user_service::*;
/// 
/// let user_service = SysUserService;
/// 
/// // 创建用户
/// let user = user_service.create_user(CreateUserInput {
///     domain: "example.com".to_string(),
///     username: "admin".to_string(),
///     password: "password123".to_string(),
///     nick_name: "Admin User".to_string(),
///     avatar: None,
///     email: Some("admin@example.com".to_string()),
///     phone_number: Some("1234567890".to_string()),
///     status: true,
/// }).await?;
/// 
#[derive(Clone)]
pub struct SysUserService;

impl SysUserService {
    /// 检查用户名唯一性
    /// 
    /// 检查用户名是否已存在
    /// 
    /// # 参数
    /// * `username` - 用户名
    /// 
    /// # 返回
    /// * `Result<(), UserError>` - 检查结果
    /// 
    /// # 错误
    /// * `UsernameAlreadyExists` - 用户名已存在
    async fn check_username_unique(&self, username: &str) -> Result<(), UserError> {
        let db = db_helper::get_db_connection().await?;
        let existing_user = SysUser::find()
            .filter(SysUserColumn::Username.eq(username))
            .one(db.as_ref())
            .await?;

        if existing_user.is_some() {
            return Err(UserError::UsernameAlreadyExists);
        }
        Ok(())
    }

    /// 根据ID获取用户
    /// 
    /// 根据用户ID获取用户模型
    /// 
    /// # 参数
    /// * `id` - 用户ID
    /// 
    /// # 返回
    /// * `Result<SysUserModel, UserError>` - 用户模型或错误
    /// 
    /// # 错误
    /// * `UserNotFound` - 用户不存在
    async fn get_user_by_id(&self, id: String) -> Result<SysUserModel, UserError> {
        let db = db_helper::get_db_connection().await?;
        SysUser::find_by_id(id)
            .one(db.as_ref())
            .await?
            .ok_or(UserError::UserNotFound)
    }
}

#[async_trait]
impl TUserService for SysUserService {
    async fn find_all(&self) -> Result<Vec<UserWithoutPassword>, UserError> {
        let db = db_helper::get_db_connection().await?;
        SysUser::find()
            .all(db.as_ref())
            .await
            .map(|users| users.into_iter().map(UserWithoutPassword::from).collect())
            .map_err(UserError::from)
    }

    async fn find_paginated_users(
        &self,
        params: UserPageRequest,
    ) -> Result<PaginatedData<UserWithoutPassword>, UserError> {
        let db = db_helper::get_db_connection().await?;
        let mut query = SysUser::find();

        if let Some(ref keywords) = params.keywords {
            let condition = Condition::any().add(SysUserColumn::Username.contains(keywords));
            query = query.filter(condition);
        }

        let total = query
            .clone()
            .count(db.as_ref())
            .await?;

        let paginator = query.paginate(db.as_ref(), params.page_details.size);
        let records = paginator
            .fetch_page(params.page_details.current - 1)
            .await?
            .into_iter()
            .map(UserWithoutPassword::from)
            .collect();

        Ok(PaginatedData {
            current: params.page_details.current,
            size: params.page_details.size,
            total,
            records,
        })
    }

    async fn create_user(&self, input: CreateUserInput) -> Result<UserWithoutPassword, UserError> {
        self.check_username_unique(&input.username).await?;

        let db = db_helper::get_db_connection().await?;
        let user = SysUserActiveModel {
            id: Set(Ulid::new().to_string()),
            domain: Set(input.domain),
            username: Set(input.username),
            password: Set(SecureUtil::hash_password(input.password.as_bytes()).unwrap()),
            built_in: Set(false),
            nick_name: Set(input.nick_name),
            avatar: Set(input.avatar),
            email: Set(input.email),
            phone_number: Set(input.phone_number),
            status: Set(input.status),
            created_at: Set(Local::now().naive_local()),
            created_by: Set("TODO".to_string()),
            ..Default::default()
        };

        let user_model = user.insert(db.as_ref()).await?;
        Ok(UserWithoutPassword::from(user_model))
    }

    async fn get_user(&self, id: &str) -> Result<UserWithoutPassword, UserError> {
        let db = db_helper::get_db_connection().await?;
        SysUser::find_by_id(id)
            .one(db.as_ref())
            .await?
            .map(UserWithoutPassword::from)
            .ok_or(UserError::UserNotFound)
    }

    async fn update_user(&self, input: UpdateUserInput) -> Result<UserWithoutPassword, UserError> {
        let mut user = self.get_user_by_id(input.id).await?.into_active_model();

        if input.user.username != *user.username.as_ref() {
            self.check_username_unique(&input.user.username).await?;
        }

        user.domain = Set(input.user.domain);
        user.username = Set(input.user.username);
        user.password = Set(input.user.password); // TODO: Note: In a real application, you should hash the password
        user.nick_name = Set(input.user.nick_name);
        user.avatar = Set(input.user.avatar);
        user.email = Set(input.user.email);
        user.phone_number = Set(input.user.phone_number);
        user.status = Set(input.user.status);

        let db = db_helper::get_db_connection().await?;
        let updated_user = user.update(db.as_ref()).await?;
        Ok(UserWithoutPassword::from(updated_user))
    }

    async fn delete_user(&self, id: &str) -> Result<(), UserError> {
        let db = db_helper::get_db_connection().await?;

        let result = SysUser::delete_by_id(id)
            .exec(db.as_ref())
            .await?;

        if result.rows_affected == 0 {
            return Err(UserError::UserNotFound);
        }

        Ok(())
    }
}
