/**
 * 系统组织服务模块
 *
 * 该模块提供了组织管理相关的核心功能，包括：
 * - 组织分页查询
 * - 组织信息管理
 *
 * 主要组件
 * --------
 * - TOrganizationService: 组织服务 trait，定义了组织管理相关的核心接口
 * - SysOrganizationService: 组织服务实现，提供了具体的组织管理逻辑
 *
 * 功能特性
 * --------
 * - 组织查询：支持分页查询和关键字搜索
 * - 组织管理：支持组织信息的增删改查
 *
 * 使用示例
 * --------
 *
 * use server_service::admin::sys_organization_service::*;
 *
 * // 创建组织服务实例
 * let org_service = SysOrganizationService;
 *
 * // 分页查询组织
 * let orgs = org_service.find_paginated_organizations(OrganizationPageRequest {
 *     keywords: Some("admin".to_string()),
 *     page_details: PageDetails {
 *         current: 1,
 *         size: 10,
 *     },
 * }).await?;
 */

use async_trait::async_trait;
use sea_orm::{ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter};
use server_core::{
    web::{error::AppError, page::PaginatedData},
    paginated_data,
};
use server_model::admin::{
    entities::{
        prelude::SysOrganization,
        sys_organization::{Column as SysOrganizationColumn, Model as SysOrganizationModel},
    },
    input::OrganizationPageRequest,
};

use crate::helper::db_helper;

/**
 * 组织服务 trait
 *
 * 定义了组织管理相关的核心接口，包括：
 * - 组织分页查询
 *
 * 使用示例
 * --------
 *
 * use server_service::admin::sys_organization_service::*;
 *
 * let org_service = SysOrganizationService;
 *
 * // 分页查询组织
 * let orgs = org_service.find_paginated_organizations(OrganizationPageRequest {
 *     keywords: Some("admin".to_string()),
 *     page_details: PageDetails {
 *         current: 1,
 *         size: 10,
 *     },
 * }).await?;
 */
#[async_trait]
pub trait TOrganizationService {
    /**
     * 分页查询组织
     *
     * 根据查询条件分页获取组织列表
     *
     * @param params 分页查询参数，包含关键字和分页信息
     * @return Result<PaginatedData<SysOrganizationModel>, AppError> 分页组织数据或错误
     */
    async fn find_paginated_organizations(
        &self,
        params: OrganizationPageRequest,
    ) -> Result<PaginatedData<SysOrganizationModel>, AppError>;
}

/**
 * 系统组织服务
 *
 * 实现了组织管理相关的核心功能，包括：
 * - 组织分页查询
 *
 * 使用示例
 * --------
 *
 * use server_service::admin::sys_organization_service::*;
 *
 * let org_service = SysOrganizationService;
 *
 * // 分页查询组织
 * let orgs = org_service.find_paginated_organizations(OrganizationPageRequest {
 *     keywords: Some("admin".to_string()),
 *     page_details: PageDetails {
 *         current: 1,
 *         size: 10,
 *     },
 * }).await?;
 */
pub struct SysOrganizationService;

#[async_trait]
impl TOrganizationService for SysOrganizationService {
    /**
     * 分页查询组织
     *
     * 根据查询条件分页获取组织列表
     *
     * @param params 分页查询参数，包含关键字和分页信息
     * @return Result<PaginatedData<SysOrganizationModel>, AppError> 分页组织数据或错误
     */
    async fn find_paginated_organizations(
        &self,
        params: OrganizationPageRequest,
    ) -> Result<PaginatedData<SysOrganizationModel>, AppError> {
        let db = db_helper::get_db_connection().await?;
        let mut query = SysOrganization::find();

        if let Some(ref keywords) = params.keywords {
            let condition = Condition::any()
                .add(SysOrganizationColumn::Code.contains(keywords))
                .add(SysOrganizationColumn::Name.contains(keywords))
                .add(SysOrganizationColumn::Description.contains(keywords));
            query = query.filter(condition);
        }

        let total = query
            .clone()
            .count(db.as_ref())
            .await
            .map_err(AppError::from)?;

        let paginator = query.paginate(db.as_ref(), params.page_details.size);
        let records = paginator
            .fetch_page(params.page_details.current - 1)
            .await
            .map_err(AppError::from)?;

        Ok(paginated_data!(
            total,
            params.page_details.current,
            params.page_details.size,
            records
        ))
    }
}
