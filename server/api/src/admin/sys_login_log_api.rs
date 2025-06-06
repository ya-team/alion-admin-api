/**
 * 登录日志管理API
 * 
 * 提供登录日志的查询接口，包括：
 * - 分页查询登录日志列表
 */
use std::sync::Arc;

use axum::extract::{Extension, Query};
use server_core::web::{error::AppError, page::PaginatedData, res::Res};
use server_service::admin::{
    LoginLogPageRequest, SysLoginLogModel, SysLoginLogService, TLoginLogService,
};

pub struct SysLoginLogApi;

impl SysLoginLogApi {
    /**
     * 分页查询登录日志列表
     * 
     * # 参数
     * - params: 分页查询参数
     * - service: 登录日志服务实例
     * 
     * # 返回
     * 返回分页后的登录日志列表数据
     */
    pub async fn get_paginated_login_logs(
        Query(params): Query<LoginLogPageRequest>,
        Extension(service): Extension<Arc<SysLoginLogService>>,
    ) -> Result<Res<PaginatedData<SysLoginLogModel>>, AppError> {
        service
            .find_paginated_login_logs(params)
            .await
            .map(Res::new_data)
    }
}
