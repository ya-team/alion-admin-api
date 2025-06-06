/**
 * 组织管理API
 * 
 * 提供组织管理的查询接口，包括：
 * - 分页查询组织列表
 */
use std::sync::Arc;

use axum::extract::{Extension, Query};
use server_core::web::{error::AppError, page::PaginatedData, res::Res};
use server_service::admin::{
    OrganizationPageRequest, SysOrganizationModel, SysOrganizationService, TOrganizationService,
};

pub struct SysOrganizationApi;

impl SysOrganizationApi {
    /**
     * 分页查询组织列表
     * 
     * # 参数
     * - params: 分页查询参数
     * - service: 组织服务实例
     * 
     * # 返回
     * 返回分页后的组织列表数据
     */
    pub async fn get_paginated_organizations(
        Query(params): Query<OrganizationPageRequest>,
        Extension(service): Extension<Arc<SysOrganizationService>>,
    ) -> Result<Res<PaginatedData<SysOrganizationModel>>, AppError> {
        service
            .find_paginated_organizations(params)
            .await
            .map(Res::new_data)
    }
}
