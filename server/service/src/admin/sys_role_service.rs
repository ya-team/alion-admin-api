/**
 * 系统角色服务模块
 *
 * 该模块提供了角色管理相关的核心功能，包括：
 * - 角色分页查询
 * - 角色CRUD操作
 * - 角色菜单关联
 *
 * 主要组件
 * --------
 * - TRoleService: 角色服务 trait，定义了角色管理相关的核心接口
 * - SysRoleService: 角色服务实现，提供了具体的角色管理逻辑
 *
 * 功能特性
 * --------
 * - 角色查询：支持分页查询和关键字搜索
 * - 角色创建：支持创建新角色，包括角色代码唯一性检查
 * - 角色更新：支持更新角色信息
 * - 角色删除：支持删除角色，包括子角色和菜单关联检查
 *
 * 使用示例
 * --------
 *
 * use server_service::admin::sys_role_service::*;
 *
 * // 创建角色服务实例
 * let role_service = SysRoleService;
 *
 * // 分页查询角色
 * let roles = role_service.find_paginated_roles(RolePageRequest {
 *     keywords: Some("admin".to_string()),
 *     page_details: PageDetails {
 *         current: 1,
 *         size: 10,
 *     },
 * }).await?;
 */

use async_trait::async_trait;
use chrono::Local;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, IntoActiveModel, PaginatorTrait, QueryFilter, Set,
};
use server_core::{
    web::page::PaginatedData,
    paginated_data,
};
use server_model::admin::{
    entities::{
        prelude::{SysRole, SysRoleMenu},
        sys_role::{
            ActiveModel as SysRoleActiveModel, Column as SysRoleColumn, Model as SysRoleModel,
        },
        sys_role_menu::Column as SysRoleMenuColumn,
    },
    input::{CreateRoleInput, RolePageRequest, UpdateRoleInput},
};
use ulid::Ulid;

use crate::{
    admin::errors::sys_role_error::RoleError,
    helper::db_helper,
};

/**
 * 角色服务 trait
 *
 * 定义了角色管理相关的核心接口，包括：
 * - 角色分页查询
 * - 角色CRUD操作
 *
 * 使用示例
 * --------
 *
 * use server_service::admin::sys_role_service::*;
 *
 * let role_service = SysRoleService;
 *
 * // 分页查询角色
 * let roles = role_service.find_paginated_roles(RolePageRequest {
 *     keywords: Some("admin".to_string()),
 *     page_details: PageDetails {
 *         current: 1,
 *         size: 10,
 *     },
 * }).await?;
 */
#[async_trait]
pub trait TRoleService {
    /**
     * 分页查询角色
     *
     * 根据查询条件分页获取角色列表
     *
     * @param params 分页查询参数，包含关键字和分页信息
     * @return Result<PaginatedData<SysRoleModel>, RoleError> 分页角色数据或错误
     */
    async fn find_paginated_roles(
        &self,
        params: RolePageRequest,
    ) -> Result<PaginatedData<SysRoleModel>, RoleError>;

    /**
     * 创建角色
     *
     * 创建新角色，包括角色代码唯一性检查
     *
     * @param input 角色创建参数
     * @return Result<SysRoleModel, RoleError> 创建的角色信息或错误
     */
    async fn create_role(&self, input: CreateRoleInput) -> Result<SysRoleModel, RoleError>;

    /**
     * 获取角色
     *
     * 根据角色ID获取角色信息
     *
     * @param id 角色ID
     * @return Result<SysRoleModel, RoleError> 角色信息或错误
     */
    async fn get_role(&self, id: &str) -> Result<SysRoleModel, RoleError>;

    /**
     * 更新角色
     *
     * 更新角色信息，包括角色代码唯一性检查
     *
     * @param input 角色更新参数
     * @return Result<SysRoleModel, RoleError> 更新后的角色信息或错误
     */
    async fn update_role(&self, input: UpdateRoleInput) -> Result<SysRoleModel, RoleError>;

    /**
     * 删除角色
     *
     * 根据角色ID删除角色，包括子角色和菜单关联检查
     *
     * @param id 角色ID
     * @return Result<(), RoleError> 删除结果
     */
    async fn delete_role(&self, id: &str) -> Result<(), RoleError>;
}

/**
 * 系统角色服务
 *
 * 实现了角色管理相关的核心功能，包括：
 * - 角色分页查询
 * - 角色CRUD操作
 * - 角色菜单关联
 *
 * 使用示例
 * --------
 *
 * use server_service::admin::sys_role_service::*;
 *
 * let role_service = SysRoleService;
 *
 * // 分页查询角色
 * let roles = role_service.find_paginated_roles(RolePageRequest {
 *     keywords: Some("admin".to_string()),
 *     page_details: PageDetails {
 *         current: 1,
 *         size: 10,
 *     },
 * }).await?;
 */
#[derive(Clone)]
pub struct SysRoleService;

impl SysRoleService {
    /**
     * 检查角色是否存在
     *
     * 检查角色代码是否已存在，支持排除当前角色
     *
     * @param id 角色ID（可选）
     * @param code 角色代码
     * @return Result<(), RoleError> 检查结果
     *
     * 错误
     * -----
     * - DuplicateRoleCode: 角色代码已存在
     */
    async fn check_role_exists(&self, id: Option<&str>, code: &str) -> Result<(), RoleError> {
        let db = db_helper::get_db_connection().await?;
        let mut query = SysRole::find().filter(SysRoleColumn::Code.eq(code));

        if let Some(id) = id {
            query = query.filter(SysRoleColumn::Id.ne(id));
        }

        let existing_role = query.one(db.as_ref()).await?;

        if existing_role.is_some() {
            return Err(RoleError::DuplicateRoleCode);
        }

        Ok(())
    }
}

#[async_trait]
impl TRoleService for SysRoleService {
    /**
     * 分页查询角色
     *
     * 根据查询条件分页获取角色列表
     *
     * @param params 分页查询参数，包含关键字和分页信息
     * @return Result<PaginatedData<SysRoleModel>, RoleError> 分页角色数据或错误
     */
    async fn find_paginated_roles(
        &self,
        params: RolePageRequest,
    ) -> Result<PaginatedData<SysRoleModel>, RoleError> {
        let db = db_helper::get_db_connection().await?;
        let mut query = SysRole::find();

        if let Some(ref keywords) = params.keywords {
            let condition = Condition::any().add(SysRoleColumn::Code.contains(keywords));
            query = query.filter(condition);
        }

        let total = query
            .clone()
            .count(db.as_ref())
            .await?;

        let paginator = query.paginate(db.as_ref(), params.page_details.size);
        let records = paginator
            .fetch_page(params.page_details.current - 1)
            .await?;

        Ok(paginated_data!(
            total,
            params.page_details.current,
            params.page_details.size,
            records
        ))
    }

    /**
     * 创建角色
     *
     * 创建新角色，包括角色代码唯一性检查
     *
     * @param input 角色创建参数
     * @return Result<SysRoleModel, RoleError> 创建的角色信息或错误
     */
    async fn create_role(&self, input: CreateRoleInput) -> Result<SysRoleModel, RoleError> {
        self.check_role_exists(None, &input.code).await?;

        let db = db_helper::get_db_connection().await?;
        let role = SysRoleActiveModel {
            id: Set(Ulid::new().to_string()),
            code: Set(input.code),
            name: Set(input.name),
            description: Set(input.description),
            pid: Set(input.pid),
            status: Set(input.status),
            created_at: Set(Local::now().naive_local()),
            created_by: Set("system".to_string()),
            ..Default::default()
        };

        let role_model = role.insert(db.as_ref()).await?;
        Ok(role_model)
    }

    /**
     * 获取角色
     *
     * 根据角色ID获取角色信息
     *
     * @param id 角色ID
     * @return Result<SysRoleModel, RoleError> 角色信息或错误
     */
    async fn get_role(&self, id: &str) -> Result<SysRoleModel, RoleError> {
        let db = db_helper::get_db_connection().await?;
        SysRole::find_by_id(id)
            .one(db.as_ref())
            .await?
            .ok_or(RoleError::RoleNotFound)
    }

    /**
     * 更新角色
     *
     * 更新角色信息，包括角色代码唯一性检查
     *
     * @param input 角色更新参数
     * @return Result<SysRoleModel, RoleError> 更新后的角色信息或错误
     */
    async fn update_role(&self, input: UpdateRoleInput) -> Result<SysRoleModel, RoleError> {
        let mut role = self.get_role(&input.id).await?.into_active_model();

        if input.role.code != *role.code.as_ref() {
            self.check_role_exists(Some(&input.id), &input.role.code).await?;
        }

        role.code = Set(input.role.code);
        role.name = Set(input.role.name);
        role.description = Set(input.role.description);
        role.pid = Set(input.role.pid);
        role.status = Set(input.role.status);

        let db = db_helper::get_db_connection().await?;
        let updated_role = role.update(db.as_ref()).await?;
        Ok(updated_role)
    }

    /**
     * 删除角色
     *
     * 根据角色ID删除角色，包括子角色和菜单关联检查
     *
     * @param id 角色ID
     * @return Result<(), RoleError> 删除结果
     *
     * 错误
     * -----
     * - HasChildren: 存在子角色
     * - InUse: 角色正在使用中
     * - RoleNotFound: 角色不存在
     */
    async fn delete_role(&self, id: &str) -> Result<(), RoleError> {
        let db = db_helper::get_db_connection().await?;

        let _role = self.get_role(id).await?;

        let has_children = SysRole::find()
            .filter(SysRoleColumn::Pid.eq(id))
            .one(db.as_ref())
            .await?
            .is_some();

        if has_children {
            return Err(RoleError::HasChildren);
        }

        let in_use = SysRoleMenu::find()
            .filter(SysRoleMenuColumn::RoleId.eq(id))
            .one(db.as_ref())
            .await?
            .is_some();

        if in_use {
            return Err(RoleError::InUse);
        }

        let result = SysRole::delete_by_id(id)
            .exec(db.as_ref())
            .await?;

        if result.rows_affected == 0 {
            return Err(RoleError::RoleNotFound);
        }

        Ok(())
    }
}
