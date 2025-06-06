/**
 * 访问密钥管理API
 * 
 * 提供访问密钥的CRUD操作接口，包括：
 * - 分页查询访问密钥列表
 * - 创建新的访问密钥
 * - 删除指定的访问密钥
 */
use std::sync::Arc;

use axum::{
    extract::{Path, Query},
    Extension,
};
use server_core::web::{error::AppError, page::PaginatedData, res::Res, validator::ValidatedForm};
use server_service::admin::{
    AccessKeyPageRequest, CreateAccessKeyInput, SysAccessKeyModel, SysAccessKeyService,
    TAccessKeyService,
};

pub struct SysAccessKeyApi;

impl SysAccessKeyApi {
    /**
     * 分页查询访问密钥列表
     * 
     * # 参数
     * - params: 分页查询参数
     * - service: 访问密钥服务实例
     * 
     * # 返回
     * 返回分页后的访问密钥列表数据
     */
    pub async fn get_paginated_access_keys(
        Query(params): Query<AccessKeyPageRequest>,
        Extension(service): Extension<Arc<SysAccessKeyService>>,
    ) -> Result<Res<PaginatedData<SysAccessKeyModel>>, AppError> {
        service
            .find_paginated_access_keys(params)
            .await
            .map(Res::new_data)
    }

    /**
     * 创建新的访问密钥
     * 
     * # 参数
     * - service: 访问密钥服务实例
     * - input: 创建访问密钥的输入参数
     * 
     * # 返回
     * 返回新创建的访问密钥信息
     */
    pub async fn create_access_key(
        Extension(service): Extension<Arc<SysAccessKeyService>>,
        ValidatedForm(input): ValidatedForm<CreateAccessKeyInput>,
    ) -> Result<Res<SysAccessKeyModel>, AppError> {
        service.create_access_key(input).await.map(Res::new_data)
    }

    /**
     * 删除指定的访问密钥
     * 
     * # 参数
     * - id: 要删除的访问密钥ID
     * - service: 访问密钥服务实例
     * 
     * # 返回
     * 返回删除操作的结果
     */
    pub async fn delete_access_key(
        Path(id): Path<String>,
        Extension(service): Extension<Arc<SysAccessKeyService>>,
    ) -> Result<Res<()>, AppError> {
        service.delete_access_key(&id).await.map(Res::new_data)
    }
}
