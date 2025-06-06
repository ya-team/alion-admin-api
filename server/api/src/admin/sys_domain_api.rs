/**
 * 域管理API
 * 
 * 提供域管理的CRUD操作接口，包括：
 * - 分页查询域列表
 * - 创建新的域
 * - 获取指定域信息
 * - 更新域信息
 * - 删除指定的域
 */
use std::sync::Arc;

use axum::{
    extract::{Path, Query},
    Extension,
};
use server_core::web::{error::AppError, page::PaginatedData, res::Res, validator::ValidatedForm};
use server_service::admin::{
    CreateDomainInput, DomainPageRequest, SysDomainModel, SysDomainService, TDomainService,
    UpdateDomainInput,
};

pub struct SysDomainApi;

impl SysDomainApi {
    /**
     * 分页查询域列表
     * 
     * # 参数
     * - params: 分页查询参数
     * - service: 域服务实例
     * 
     * # 返回
     * 返回分页后的域列表数据
     */
    pub async fn get_paginated_domains(
        Query(params): Query<DomainPageRequest>,
        Extension(service): Extension<Arc<SysDomainService>>,
    ) -> Result<Res<PaginatedData<SysDomainModel>>, AppError> {
        service
            .find_paginated_domains(params)
            .await
            .map(Res::new_data)
    }

    /**
     * 创建新的域
     * 
     * # 参数
     * - service: 域服务实例
     * - input: 创建域的输入参数
     * 
     * # 返回
     * 返回新创建的域信息
     */
    pub async fn create_domain(
        Extension(service): Extension<Arc<SysDomainService>>,
        ValidatedForm(input): ValidatedForm<CreateDomainInput>,
    ) -> Result<Res<SysDomainModel>, AppError> {
        service.create_domain(input).await.map(Res::new_data)
    }

    /**
     * 获取指定域信息
     * 
     * # 参数
     * - id: 域ID
     * - service: 域服务实例
     * 
     * # 返回
     * 返回指定域的详细信息
     */
    pub async fn get_domain(
        Path(id): Path<String>,
        Extension(service): Extension<Arc<SysDomainService>>,
    ) -> Result<Res<SysDomainModel>, AppError> {
        service.get_domain(&id).await.map(Res::new_data)
    }

    /**
     * 更新域信息
     * 
     * # 参数
     * - service: 域服务实例
     * - input: 更新域的输入参数
     * 
     * # 返回
     * 返回更新后的域信息
     */
    pub async fn update_domain(
        Extension(service): Extension<Arc<SysDomainService>>,
        ValidatedForm(input): ValidatedForm<UpdateDomainInput>,
    ) -> Result<Res<SysDomainModel>, AppError> {
        service.update_domain(input).await.map(Res::new_data)
    }

    /**
     * 删除指定的域
     * 
     * # 参数
     * - id: 要删除的域ID
     * - service: 域服务实例
     * 
     * # 返回
     * 返回删除操作的结果
     */
    pub async fn delete_domain(
        Path(id): Path<String>,
        Extension(service): Extension<Arc<SysDomainService>>,
    ) -> Result<Res<()>, AppError> {
        service.delete_domain(&id).await.map(Res::new_data)
    }
}
