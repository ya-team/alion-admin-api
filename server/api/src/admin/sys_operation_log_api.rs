/**
 * 操作日志管理API
 * 
 * 提供操作日志的查询接口，包括：
 * - 分页查询操作日志列表
 */
use std::sync::Arc;

use axum::extract::{Extension, Query};
use server_core::web::{error::AppError, page::PaginatedData, res::Res};
use server_service::admin::{
    OperationLogPageRequest, SysOperationLogModel, SysOperationLogService, TOperationLogService,
};

pub struct SysOperationLogApi;

impl SysOperationLogApi {
    /**
     * 分页查询操作日志列表
     * 
     * # 参数
     * - params: 分页查询参数
     * - service: 操作日志服务实例
     * 
     * # 返回
     * 返回分页后的操作日志列表数据
     */
    pub async fn get_paginated_operation_logs(
        Query(params): Query<OperationLogPageRequest>,
        Extension(service): Extension<Arc<SysOperationLogService>>,
    ) -> Result<Res<PaginatedData<SysOperationLogModel>>, AppError> {
        service
            .find_paginated_operation_logs(params)
            .await
            .map(Res::new_data)
    }
}
