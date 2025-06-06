/**
 * 登录日志服务模块
 *
 * 该模块提供了登录日志管理相关的核心功能，包括：
 * - 登录日志分页查询
 * - 关键字搜索
 *
 * 主要组件
 * --------
 * - TLoginLogService: 登录日志服务 trait，定义了日志管理相关的核心接口
 * - SysLoginLogService: 登录日志服务实现，提供了具体的日志管理逻辑
 *
 * 使用示例
 * --------
 *
 * use server_service::admin::sys_login_log_service::*;
 *
 * let log_service = SysLoginLogService;
 *
 * // 分页查询登录日志
 * let logs = log_service.find_paginated_login_logs(LoginLogPageRequest {
 *     keywords: Some("admin".to_string()),
 *     page_details: PageDetails { current: 1, size: 10 },
 * }).await?;
 */

use async_trait::async_trait;
use sea_orm::{ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};
use server_core::{
    web::{error::AppError, page::PaginatedData},
    paginated_data,
};
use server_model::admin::{
    entities::{
        prelude::SysLoginLog,
        sys_login_log::{Column as SysLoginLogColumn, Model as SysLoginLogModel},
    },
    input::LoginLogPageRequest,
};

use crate::helper::db_helper;

/**
 * 登录日志服务 trait
 *
 * 定义了登录日志管理相关的核心接口，包括：
 * - 日志分页查询
 *
 * 使用示例：
 * let log_service = SysLoginLogService;
 * let logs = log_service.find_paginated_login_logs(...).await?;
 */
#[async_trait]
pub trait TLoginLogService {
    /**
     * 分页查询登录日志
     * @param params 分页查询参数
     * @return Result<PaginatedData<SysLoginLogModel>, AppError>
     */
    async fn find_paginated_login_logs(
        &self,
        params: LoginLogPageRequest,
    ) -> Result<PaginatedData<SysLoginLogModel>, AppError>;
}

/**
 * 登录日志服务实现
 *
 * 实现了 TLoginLogService trait，提供了登录日志的分页查询功能。
 */
pub struct SysLoginLogService;

#[async_trait]
impl TLoginLogService for SysLoginLogService {
    async fn find_paginated_login_logs(
        &self,
        params: LoginLogPageRequest,
    ) -> Result<PaginatedData<SysLoginLogModel>, AppError> {
        let db = db_helper::get_db_connection().await?;
        let mut query = SysLoginLog::find();

        if let Some(ref keywords) = params.keywords {
            let condition = Condition::any()
                .add(SysLoginLogColumn::Domain.contains(keywords))
                .add(SysLoginLogColumn::Username.contains(keywords))
                .add(SysLoginLogColumn::Ip.contains(keywords))
                .add(SysLoginLogColumn::Address.contains(keywords))
                .add(SysLoginLogColumn::UserAgent.contains(keywords));
            query = query.filter(condition);
        }

        query = query.order_by_desc(SysLoginLogColumn::CreatedAt);

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
