/**
 * 角色管理API
 * 
 * 提供角色管理的CRUD操作接口，包括：
 * - 分页查询角色列表
 * - 创建新角色
 * - 获取角色详情
 * - 更新角色信息
 * - 删除角色
 */
use std::sync::Arc;

use axum::{
    extract::{Path, Query},
    Extension,
    Json,
};
use server_core::web::{
    error::AppError,
    page::PaginatedData,
    res::Res,
};
use server_model::admin::{
    entities::sys_role::Model as SysRoleModel,
    input::{CreateRoleInput, RolePageRequest, UpdateRoleInput},
};
use server_service::admin::{
    SysRoleService,
    TRoleService,
};

pub struct SysRoleApi;

impl SysRoleApi {
    /**
     * 分页查询角色列表
     * 
     * # 参数
     * - service: 角色服务实例
     * - params: 分页查询参数
     * 
     * # 返回
     * 返回分页后的角色列表数据
     */
    pub async fn find_paginated_roles(
        Extension(service): Extension<Arc<SysRoleService>>,
        Query(params): Query<RolePageRequest>,
    ) -> Result<Json<Res<PaginatedData<SysRoleModel>>>, AppError> {
        let result = service.find_paginated_roles(params).await?;
        Ok(Json(Res::new_data(result)))
    }

    /**
     * 创建新角色
     * 
     * # 参数
     * - service: 角色服务实例
     * - input: 创建角色的输入参数
     * 
     * # 返回
     * 返回新创建的角色信息
     */
    pub async fn create_role(
        Extension(service): Extension<Arc<SysRoleService>>,
        Json(input): Json<CreateRoleInput>,
    ) -> Result<Json<Res<SysRoleModel>>, AppError> {
        let result = service.create_role(input).await?;
        Ok(Json(Res::new_data(result)))
    }

    /**
     * 获取角色详情
     * 
     * # 参数
     * - service: 角色服务实例
     * - id: 角色ID
     * 
     * # 返回
     * 返回指定角色的详细信息
     */
    pub async fn get_role(
        Extension(service): Extension<Arc<SysRoleService>>,
        Path(id): Path<i32>,
    ) -> Result<Json<Res<SysRoleModel>>, AppError> {
        let result = service.get_role(&id.to_string()).await?;
        Ok(Json(Res::new_data(result)))
    }

    /**
     * 更新角色信息
     * 
     * # 参数
     * - service: 角色服务实例
     * - input: 更新角色的输入参数
     * 
     * # 返回
     * 返回更新后的角色信息
     */
    pub async fn update_role(
        Extension(service): Extension<Arc<SysRoleService>>,
        Json(input): Json<UpdateRoleInput>,
    ) -> Result<Json<Res<SysRoleModel>>, AppError> {
        let result = service.update_role(input).await?;
        Ok(Json(Res::new_data(result)))
    }

    /**
     * 删除角色
     * 
     * # 参数
     * - service: 角色服务实例
     * - id: 要删除的角色ID
     * 
     * # 返回
     * 返回删除操作的结果
     */
    pub async fn delete_role(
        Extension(service): Extension<Arc<SysRoleService>>,
        Path(id): Path<i32>,
    ) -> Result<Json<Res<()>>, AppError> {
        let result = service.delete_role(&id.to_string()).await?;
        Ok(Json(Res::new_data(result)))
    }
}
