//! 系统操作日志服务模块
//! 
//! 该模块提供了操作日志管理相关的核心功能，包括：
//! - 操作日志分页查询
//! - 操作日志事件处理
//! - 操作日志监听器
//! 
//! # 主要组件
//! 
//! ## 核心接口
//! * `TOperationLogService`: 操作日志服务 trait，定义了操作日志管理相关的核心接口
//! * `SysOperationLogService`: 操作日志服务实现，提供了具体的操作日志管理逻辑
//! 
//! ## 功能特性
//! * 日志查询：支持分页查询和关键字搜索
//! * 日志记录：支持记录用户操作日志
//! * 事件处理：支持异步处理操作日志事件
//! 
//! # 使用示例
//! 
//! use server_service::admin::sys_operation_log_service::*;
//! 
//! // 创建操作日志服务实例
//! let log_service = SysOperationLogService;
//! 
//! // 分页查询操作日志
//! let logs = log_service.find_paginated_operation_logs(OperationLogPageRequest {
//!     keywords: Some("admin".to_string()),
//!     page_details: PageDetails {
//!         current: 1,
//!         size: 10,
//!     },
//! }).await?;
//! 
//! // 处理操作日志事件
//! let event = OperationLogContext {
//!     user_id: Some("user1".to_string()),
//!     username: Some("admin".to_string()),
//!     domain: Some("example.com".to_string()),
//!     module_name: "user".to_string(),
//!     description: "创建用户".to_string(),
//!     request_id: "req-123".to_string(),
//!     method: "POST".to_string(),
//!     url: "/api/users".to_string(),
//!     ip: "127.0.0.1".to_string(),
//!     user_agent: "Mozilla/5.0".to_string(),
//!     params: "{}".to_string(),
//!     body: "{}".to_string(),
//!     response: "{}".to_string(),
//!     start_time: chrono::Local::now(),
//!     end_time: chrono::Local::now(),
//!     duration: 100,
//!     created_at: chrono::Local::now(),
//! };
//! 
//! SysOperationLogService::handle_operation_log_event(&event).await?;
//! 

use std::any::Any;

use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter,
    Set,
};
use server_core::web::{error::AppError, page::PaginatedData};
use server_global::{global::OperationLogContext, project_error};
use server_model::admin::{
    entities::{
        prelude::SysOperationLog,
        sys_operation_log::{
            ActiveModel as SysOperationLogActiveModel, Column as SysOperationLogColumn,
            Model as SysOperationLogModel,
        },
    },
    input::OperationLogPageRequest,
};
use tracing::instrument;
use ulid::Ulid;

use crate::helper::db_helper;
use super::errors::sys_operation_log_error::OperationLogError;

/// 操作日志服务 trait
/// 
/// 定义了操作日志管理相关的核心接口，包括：
/// - 操作日志分页查询
/// - 操作日志事件处理
/// 
/// # 使用示例
/// 
/// use server_service::admin::sys_operation_log_service::*;
/// 
/// let log_service = SysOperationLogService;
/// 
/// // 分页查询操作日志
/// let logs = log_service.find_paginated_operation_logs(OperationLogPageRequest {
///     keywords: Some("admin".to_string()),
///     page_details: PageDetails {
///         current: 1,
///         size: 10,
///     },
/// }).await?;
/// 
#[async_trait]
pub trait TOperationLogService {
    /// 分页查询操作日志
    /// 
    /// 根据查询条件分页获取操作日志列表
    /// 
    /// # 参数
    /// * `params` - 分页查询参数，包含关键字和分页信息
    /// 
    /// # 返回
    /// * `Result<PaginatedData<SysOperationLogModel>, AppError>` - 分页操作日志数据或错误
    async fn find_paginated_operation_logs(
        &self,
        params: OperationLogPageRequest,
    ) -> Result<PaginatedData<SysOperationLogModel>, AppError>;

    /// 处理操作日志事件
    /// 
    /// 处理操作日志事件，记录用户操作信息
    /// 
    /// # 参数
    /// * `event` - 操作日志上下文
    /// 
    /// # 返回
    /// * `Result<(), AppError>` - 处理结果
    async fn handle_operation_log_event(event: &OperationLogContext) -> Result<(), AppError>;
}

/// 系统操作日志服务
/// 
/// 实现了操作日志管理相关的核心功能，包括：
/// - 操作日志分页查询
/// - 操作日志事件处理
/// 
/// # 使用示例
/// 
/// use server_service::admin::sys_operation_log_service::*;
/// 
/// let log_service = SysOperationLogService;
/// 
/// // 分页查询操作日志
/// let logs = log_service.find_paginated_operation_logs(OperationLogPageRequest {
///     keywords: Some("admin".to_string()),
///     page_details: PageDetails {
///         current: 1,
///         size: 10,
///     },
/// }).await?;
/// 
pub struct SysOperationLogService;

#[async_trait]
impl TOperationLogService for SysOperationLogService {
    async fn find_paginated_operation_logs(
        &self,
        params: OperationLogPageRequest,
    ) -> Result<PaginatedData<SysOperationLogModel>, AppError> {
        let db = db_helper::get_db_connection().await?;
        let mut query = SysOperationLog::find();

        if let Some(ref keywords) = params.keywords {
            let condition = Condition::any()
                .add(SysOperationLogColumn::Username.contains(keywords))
                .add(SysOperationLogColumn::Description.contains(keywords));
            query = query.filter(condition);
        }

        let total = query
            .clone()
            .count(db.as_ref())
            .await
            .map_err(|_| OperationLogError::CreateFailed)?;

        let paginator = query.paginate(db.as_ref(), params.page_details.size);
        let records = paginator
            .fetch_page(params.page_details.current - 1)
            .await
            .map_err(|_| OperationLogError::CreateFailed)?;

        Ok(PaginatedData {
            current: params.page_details.current,
            size: params.page_details.size,
            total,
            records,
        })
    }

    async fn handle_operation_log_event(event: &OperationLogContext) -> Result<(), AppError> {
        let db = db_helper::get_db_connection().await?;

        let result = SysOperationLogActiveModel {
            id: Set(Ulid::new().to_string()),
            user_id: Set(event.user_id.clone().unwrap_or_default()),
            username: Set(event.username.clone().unwrap_or_default()),
            domain: Set(event.domain.clone().unwrap_or_default()),
            module_name: Set(event.module_name.clone()),
            description: Set(event.description.clone()),
            request_id: Set(event.request_id.clone()),
            method: Set(event.method.clone()),
            url: Set(event.url.clone()),
            ip: Set(event.ip.clone()),
            user_agent: Set(event.user_agent.clone()),
            params: Set(event.params.clone()),
            body: Set(event.body.clone()),
            response: Set(event.response.clone()),
            start_time: Set(event.start_time),
            end_time: Set(event.end_time),
            duration: Set(event.duration),
            created_at: Set(event.created_at),
        }
        .insert(db.as_ref())
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(OperationLogError::CreateFailed.into()),
        }
    }
}

/// 系统操作日志监听器
/// 
/// 监听并处理操作日志事件，用于：
/// - 接收操作日志事件
/// - 异步处理操作日志
/// - 错误处理和日志记录
/// 
/// # 参数
/// * `rx` - 事件接收器
#[instrument(skip(rx))]
pub async fn sys_operation_log_listener(
    mut rx: tokio::sync::mpsc::UnboundedReceiver<Box<dyn Any + Send>>,
) {
    while let Some(event) = rx.recv().await {
        if let Some(operation_log_context) = event.downcast_ref::<OperationLogContext>() {
            if let Err(e) = SysOperationLogService::handle_operation_log_event(operation_log_context).await {
                project_error!("Failed to handle operation log event: {:?}", e);
            }
        } else {
            project_error!("Received unknown event type in operation log listener");
        }
    }
}
