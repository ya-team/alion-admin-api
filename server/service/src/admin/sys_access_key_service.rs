/** 访问密钥服务模块
 * 
 * 该模块提供了访问密钥（API Key）的管理功能，包括：
 * - 访问密钥的创建和删除
 * - 访问密钥的验证和授权
 * - 访问密钥的分页查询
 * 
 * 主要组件
 * --------
 * 
 * 服务接口
 * --------
 * * `TAccessKeyService`: 访问密钥服务接口，定义了核心操作方法
 * * `SysAccessKeyService`: 访问密钥服务实现，提供了具体的业务逻辑
 * 
 * 事件处理
 * --------
 * * `api_key_validate_listener`: API密钥验证事件监听器
 * 
 * 使用示例
 * --------
 * /* 创建访问密钥
 *  * let service = SysAccessKeyService;
 *  * let result = service.create_access_key(CreateAccessKeyInput {
 *  *     domain: "example.com".to_string(),
 *  *     status: "active".to_string(),
 *  *     description: "Test API Key".to_string(),
 *  * }).await?;
 *  */
 */

use std::any::Any;

use async_trait::async_trait;
use chrono::Local;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseTransaction, EntityTrait, PaginatorTrait,
    QueryFilter, Set, TransactionTrait,
};
use server_core::{
    sign::{ApiKeyEvent, ValidatorType},
    web::{error::AppError, page::PaginatedData},
    paginated_data,
};
use server_global::project_info;
use server_model::admin::{
    entities::{
        prelude::SysAccessKey,
        sys_access_key::{
            ActiveModel as SysAccessKeyActiveModel, Column as SysAccessKeyColumn,
            Model as SysAccessKeyModel,
        },
    },
    input::{AccessKeyPageRequest, CreateAccessKeyInput},
};
use tracing::instrument;
use ulid::Ulid;

use crate::helper::db_helper;

use super::sys_access_key_error::AccessKeyError;

/** 访问密钥服务接口
 * 
 * 定义了访问密钥管理的核心接口，包括：
 * - 分页查询访问密钥
 * - 创建访问密钥
 * - 删除访问密钥
 * - 初始化访问密钥
 */
#[async_trait]
pub trait TAccessKeyService {
    /** 分页查询访问密钥
     * 
     * 根据查询条件分页获取访问密钥列表
     * 
     * 参数
     * --------
     * * `params` - 分页查询参数，包含关键字和分页信息
     * 
     * 返回
     * --------
     * * `Result<PaginatedData<SysAccessKeyModel>, AppError>` - 分页访问密钥数据或错误
     */
    async fn find_paginated_access_keys(
        &self,
        params: AccessKeyPageRequest,
    ) -> Result<PaginatedData<SysAccessKeyModel>, AppError>;

    /** 创建访问密钥
     * 
     * 创建新的访问密钥，包括：
     * - 生成访问密钥ID和密钥
     * - 设置密钥状态和描述
     * - 记录创建信息
     * 
     * 参数
     * --------
     * * `input` - 访问密钥创建参数
     * 
     * 返回
     * --------
     * * `Result<SysAccessKeyModel, AppError>` - 创建的访问密钥信息或错误
     */
    async fn create_access_key(
        &self,
        input: CreateAccessKeyInput,
    ) -> Result<SysAccessKeyModel, AppError>;

    /** 删除访问密钥
     * 
     * 根据ID删除访问密钥，包括：
     * - 从数据库中删除记录
     * - 从验证器中移除密钥
     * 
     * 参数
     * --------
     * * `id` - 访问密钥ID
     * 
     * 返回
     * --------
     * * `Result<(), AppError>` - 删除结果
     */
    async fn delete_access_key(&self, id: &str) -> Result<(), AppError>;

    /** 初始化访问密钥
     * 
     * 系统启动时初始化访问密钥，包括：
     * - 加载所有有效的访问密钥
     * - 将密钥添加到验证器
     * 
     * 返回
     * --------
     * * `Result<(), AppError>` - 初始化结果
     */
    async fn initialize_access_key(&self) -> Result<(), AppError>;
}

/** 访问密钥服务实现
 * 
 * 提供了访问密钥管理的具体实现，包括：
 * - 访问密钥的CRUD操作
 * - 访问密钥的验证和授权
 * - 访问密钥的状态管理
 */
#[derive(Clone)]
pub struct SysAccessKeyService;

impl SysAccessKeyService {
    /** 在事务中创建访问密钥
     * 
     * 参数
     * --------
     * * `txn` - 数据库事务
     * * `access_key` - 访问密钥模型
     * 
     * 返回
     * --------
     * * `Result<SysAccessKeyModel, AppError>` - 创建的访问密钥信息或错误
     */
    async fn create_access_key_in_transaction(
        &self,
        txn: &DatabaseTransaction,
        access_key: SysAccessKeyActiveModel,
    ) -> Result<SysAccessKeyModel, AppError> {
        let result = access_key.insert(txn).await.map_err(AppError::from)?;

        // 添加到验证器
        server_core::sign::add_key(ValidatorType::Simple, &result.access_key_id, None).await;
        server_core::sign::add_key(
            ValidatorType::Complex,
            &result.access_key_id,
            Some(&result.access_key_secret),
        )
        .await;

        Ok(result)
    }

    /** 在事务中删除访问密钥
     * 
     * 参数
     * --------
     * * `txn` - 数据库事务
     * * `id` - 访问密钥ID
     * 
     * 返回
     * --------
     * * `Result<(), AppError>` - 删除结果
     */
    async fn delete_access_key_in_transaction(
        &self,
        txn: &DatabaseTransaction,
        id: &str,
    ) -> Result<(), AppError> {
        // 先获取 access key 信息
        let access_key = SysAccessKey::find_by_id(id)
            .one(txn)
            .await
            .map_err(AppError::from)?
            .ok_or_else(|| AppError::from(AccessKeyError::AccessKeyNotFound))?;

        // 从数据库中删除
        SysAccessKey::delete_by_id(id)
            .exec(txn)
            .await
            .map_err(AppError::from)?;

        // 从验证器中移除
        server_core::sign::remove_key(ValidatorType::Simple, &access_key.access_key_id).await;
        server_core::sign::remove_key(ValidatorType::Complex, &access_key.access_key_id).await;

        Ok(())
    }
}

#[async_trait]
impl TAccessKeyService for SysAccessKeyService {
    async fn find_paginated_access_keys(
        &self,
        params: AccessKeyPageRequest,
    ) -> Result<PaginatedData<SysAccessKeyModel>, AppError> {
        let db = db_helper::get_db_connection().await?;
        let mut query = SysAccessKey::find();

        if let Some(ref keywords) = params.keywords {
            let condition = Condition::any().add(SysAccessKeyColumn::Domain.contains(keywords));
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

    async fn create_access_key(
        &self,
        input: CreateAccessKeyInput,
    ) -> Result<SysAccessKeyModel, AppError> {
        let db = db_helper::get_db_connection().await?;
        let txn = db.begin().await.map_err(AppError::from)?;

        let access_key_id = format!("AK{}", Ulid::new().to_string());
        let access_key_secret = format!("SK{}", Ulid::new().to_string());

        let access_key = SysAccessKeyActiveModel {
            id: Set(Ulid::new().to_string()),
            domain: Set(input.domain),
            status: Set(input.status),
            description: Set(input.description),
            access_key_id: Set(access_key_id),
            access_key_secret: Set(access_key_secret),
            created_at: Set(Local::now().naive_local()),
            created_by: Set("TODO".to_string()),
            ..Default::default()
        };

        let result = match self
            .create_access_key_in_transaction(&txn, access_key)
            .await
        {
            Ok(result) => {
                txn.commit().await.map_err(AppError::from)?;
                result
            },
            Err(e) => {
                txn.rollback().await.map_err(AppError::from)?;
                return Err(e);
            },
        };

        Ok(result)
    }

    async fn delete_access_key(&self, id: &str) -> Result<(), AppError> {
        let db = db_helper::get_db_connection().await?;
        let txn = db.begin().await.map_err(AppError::from)?;

        match self.delete_access_key_in_transaction(&txn, id).await {
            Ok(_) => {
                txn.commit().await.map_err(AppError::from)?;
                Ok(())
            },
            Err(e) => {
                txn.rollback().await.map_err(AppError::from)?;
                Err(e)
            },
        }
    }

    async fn initialize_access_key(&self) -> Result<(), AppError> {
        let db = db_helper::get_db_connection().await?;

        let access_keys = SysAccessKey::find()
            .all(db.as_ref())
            .await
            .map_err(AppError::from)?;

        for access_key in access_keys {
            server_core::sign::add_key(ValidatorType::Simple, &access_key.access_key_id, None)
                .await;
            server_core::sign::add_key(
                ValidatorType::Complex,
                &access_key.access_key_id,
                Some(&access_key.access_key_secret),
            )
            .await;
        }

        Ok(())
    }
}

/** API密钥验证事件监听器
 * 
 * 监听并处理API密钥验证事件，用于：
 * - 记录密钥验证日志
 * - 监控密钥使用情况
 * 
 * 参数
 * --------
 * * `rx` - 事件接收器
 */
#[instrument(skip(rx))]
pub async fn api_key_validate_listener(
    mut rx: tokio::sync::mpsc::UnboundedReceiver<Box<dyn Any + Send>>,
) {
    while let Some(event) = rx.recv().await {
        if let Some(api_key_event) = event.downcast_ref::<ApiKeyEvent>() {
            project_info!("API key validated: {:?}", api_key_event);
        }
    }
}
