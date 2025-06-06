/**
 * 用户管理API
 * 
 * 提供用户管理的CRUD操作接口，包括：
 * - 获取所有用户列表
 * - 分页查询用户列表
 * - 创建新用户
 * - 获取用户详情
 * - 更新用户信息
 * - 删除用户
 * - 权限策略管理（添加/删除）
 */
use std::sync::Arc;

use axum::{
    extract::{Path, Query},
    Extension,
};
use axum_casbin::{casbin::MgmtApi, CasbinAxumLayer};
use server_core::web::{
    auth::User,
    error::AppError,
    page::PaginatedData,
    res::Res,
    validator::ValidatedForm,
};
use server_model::admin::{
    input::{CreateUserInput, UpdateUserInput, UserPageRequest},
    output::UserWithoutPassword,
};
use server_service::admin::{
    SysUserService,
    TUserService,
};

pub struct SysUserApi;

impl SysUserApi {
    /**
     * 获取所有用户列表
     * 
     * # 参数
     * - service: 用户服务实例
     * 
     * # 返回
     * 返回所有用户的列表数据（不包含密码信息）
     */
    pub async fn get_all_users(
        Extension(service): Extension<Arc<SysUserService>>,
    ) -> Result<Res<Vec<UserWithoutPassword>>, AppError> {
        service.find_all().await.map_err(AppError::from).map(Res::new_data)
    }

    /**
     * 分页查询用户列表
     * 
     * # 参数
     * - params: 分页查询参数
     * - service: 用户服务实例
     * - user: 当前认证用户信息
     * 
     * # 返回
     * 返回分页后的用户列表数据（不包含密码信息）
     */
    pub async fn get_paginated_users(
        Query(params): Query<UserPageRequest>,
        Extension(service): Extension<Arc<SysUserService>>,
        user: User,
    ) -> Result<Res<PaginatedData<UserWithoutPassword>>, AppError> {
        print!("user is {:#?}", user);
        service
            .find_paginated_users(params)
            .await
            .map_err(AppError::from)
            .map(Res::new_data)
    }

    /**
     * 删除权限策略
     * 
     * # 参数
     * - cache_enforcer: Casbin执行器
     * 
     * # 返回
     * 返回删除操作的结果
     */
    pub async fn remove_policies(
        Extension(mut cache_enforcer): Extension<CasbinAxumLayer>,
    ) -> Res<bool> {
        let enforcer = cache_enforcer.get_enforcer();
        let mut enforcer_write = enforcer.write().await;
        let rule = vec![
            "1".to_string(),
            "built-in".to_string(),
            "/user/users".to_string(),
            "GET".to_string(),
        ];
        let _ = enforcer_write.remove_policies(vec![rule]).await;
        Res::new_data(true)
    }

    /**
     * 添加权限策略
     * 
     * # 参数
     * - cache_enforcer: Casbin执行器
     * 
     * # 返回
     * 返回添加操作的结果
     */
    pub async fn add_policies(
        Extension(mut cache_enforcer): Extension<CasbinAxumLayer>,
    ) -> Res<bool> {
        let enforcer = cache_enforcer.get_enforcer();
        let mut enforcer_write = enforcer.write().await;
        let rule = vec![
            "1".to_string(),
            "built-in".to_string(),
            "/user/users".to_string(),
            "GET".to_string(),
        ];
        let _ = enforcer_write.add_policy(rule).await;
        Res::new_data(true)
    }

    /**
     * 创建新用户
     * 
     * # 参数
     * - service: 用户服务实例
     * - input: 创建用户的输入参数
     * 
     * # 返回
     * 返回新创建的用户信息（不包含密码）
     */
    pub async fn create_user(
        Extension(service): Extension<Arc<SysUserService>>,
        ValidatedForm(input): ValidatedForm<CreateUserInput>,
    ) -> Result<Res<UserWithoutPassword>, AppError> {
        service.create_user(input).await.map_err(AppError::from).map(Res::new_data)
    }

    /**
     * 获取用户详情
     * 
     * # 参数
     * - id: 用户ID
     * - service: 用户服务实例
     * 
     * # 返回
     * 返回指定用户的详细信息（不包含密码）
     */
    pub async fn get_user(
        Path(id): Path<String>,
        Extension(service): Extension<Arc<SysUserService>>,
    ) -> Result<Res<UserWithoutPassword>, AppError> {
        service.get_user(&id).await.map_err(AppError::from).map(Res::new_data)
    }

    /**
     * 更新用户信息
     * 
     * # 参数
     * - service: 用户服务实例
     * - input: 更新用户的输入参数
     * 
     * # 返回
     * 返回更新后的用户信息（不包含密码）
     */
    pub async fn update_user(
        Extension(service): Extension<Arc<SysUserService>>,
        ValidatedForm(input): ValidatedForm<UpdateUserInput>,
    ) -> Result<Res<UserWithoutPassword>, AppError> {
        service.update_user(input).await.map_err(AppError::from).map(Res::new_data)
    }

    /**
     * 删除用户
     * 
     * # 参数
     * - id: 要删除的用户ID
     * - service: 用户服务实例
     * 
     * # 返回
     * 返回删除操作的结果
     */
    pub async fn delete_user(
        Path(id): Path<String>,
        Extension(service): Extension<Arc<SysUserService>>,
    ) -> Result<Res<()>, AppError> {
        service.delete_user(&id).await.map_err(AppError::from).map(Res::new_data)
    }
}
