use std::collections::BTreeMap;

use async_trait::async_trait;
use chrono::Local;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, DeleteResult, EntityTrait, IntoActiveModel,
    PaginatorTrait, QueryFilter, Set,
};
use server_core::{
    web::{error::AppError, page::PaginatedData},
    paginated_data,
};
use server_model::admin::entities::{
    prelude::{SysEndpoint, SysRoleMenu},
    sys_endpoint::{ActiveModel as SysEndpointActiveModel, Column as SysEndpointColumn, Model as SysEndpointModel},
    sys_role_menu::{ActiveModel as SysRoleMenuActiveModel, Column as SysRoleMenuColumn},
};
use server_model::admin::input::EndpointPageRequest;
use server_model::admin::output::EndpointTree;
use tracing::{error, info};

use crate::helper::{db_helper, transaction_helper::execute_in_transaction};
use crate::admin::errors::sys_endpoint_error::EndpointError;

/**
 * 系统端点服务模块
 *
 * 该模块提供了端点（API接口）管理相关的核心功能，包括：
 * - 端点CRUD操作
 * - 端点分页查询
 * - 端点树结构生成
 * - 端点分配到角色
 *
 * 主要组件
 * --------
 * - TEndpointService: 端点服务 trait，定义了端点管理相关的核心接口
 * - SysEndpointService: 端点服务实现，提供了具体的端点管理逻辑
 *
 * 功能特性
 * --------
 * - 端点同步：批量同步接口定义
 * - 端点查询：支持分页查询和关键字搜索
 * - 端点树：按 controller 组织的树结构
 * - 端点分配：支持为角色分配端点权限
 *
 * 使用示例
 * --------
 *
 * use server_service::admin::sys_endpoint_service::*;
 *
 * let endpoint_service = SysEndpointService::new(db.clone());
 *
 * // 分页查询端点
 * let endpoints = endpoint_service.find_paginated_endpoints(EndpointPageRequest {
 *     keywords: Some("user".to_string()),
 *     page_details: PageDetails { current: 1, size: 10 },
 * }).await?;
 *
 * // 获取端点树
 * let tree = endpoint_service.tree_endpoint().await?;
 */

#[async_trait]
pub trait TEndpointService {
    async fn sync_endpoints(&self, endpoints: Vec<SysEndpointModel>) -> Result<(), AppError>;
    async fn find_paginated_endpoints(
        &self,
        params: EndpointPageRequest,
    ) -> Result<PaginatedData<SysEndpointModel>, AppError>;

    async fn tree_endpoint(&self) -> Result<Vec<EndpointTree>, AppError>;
}

#[derive(Clone)]
#[allow(dead_code)]
pub struct SysEndpointService {
    db: DatabaseConnection,
}

impl SysEndpointService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    #[allow(dead_code)]
    async fn batch_update_endpoints(
        &self,
        db: &DatabaseConnection,
        endpoints: Vec<SysEndpointModel>,
    ) -> Result<(), AppError> {
        let now = Local::now().naive_local();
        let active_models: Vec<SysEndpointActiveModel> = endpoints
            .into_iter()
            .map(|endpoint| {
                let mut active_model: SysEndpointActiveModel = endpoint.into_active_model();
                active_model.updated_at = Set(Some(now));
                active_model
            })
            .collect();

        SysEndpoint::insert_many(active_models)
            .on_conflict(
                sea_orm::sea_query::OnConflict::column(SysEndpointColumn::Id)
                    .update_columns([
                        SysEndpointColumn::Path,
                        SysEndpointColumn::Method,
                        SysEndpointColumn::Action,
                        SysEndpointColumn::Resource,
                        SysEndpointColumn::Controller,
                        SysEndpointColumn::Summary,
                        SysEndpointColumn::UpdatedAt,
                    ])
                    .to_owned(),
            )
            .exec(db)
            .await
            .map_err(AppError::from)?;

        Ok(())
    }

    #[allow(dead_code)]
    async fn batch_remove_endpoints(
        &self,
        db: &DatabaseConnection,
        endpoints_to_remove: Vec<String>,
    ) -> Result<DeleteResult, AppError> {
        SysEndpoint::delete_many()
            .filter(SysEndpointColumn::Id.is_in(endpoints_to_remove))
            .exec(db)
            .await
            .map_err(AppError::from)
    }

    fn create_endpoint_tree(&self, endpoints: &[SysEndpointModel]) -> Vec<EndpointTree> {
        let mut controller_map: BTreeMap<String, EndpointTree> = BTreeMap::new();

        for endpoint in endpoints {
            let controller = endpoint.controller.clone();

            let controller_node =
                controller_map
                    .entry(controller.clone())
                    .or_insert(EndpointTree {
                        id: format!("controller-{}", controller),
                        path: String::new(),
                        method: String::new(),
                        action: String::new(),
                        resource: String::new(),
                        controller: controller.clone(),
                        summary: None,
                        children: Some(Vec::new()),
                    });

            if let Some(children) = &mut controller_node.children {
                children.push(EndpointTree {
                    id: endpoint.id.to_string(),
                    path: endpoint.path.clone(),
                    method: endpoint.method.clone(),
                    action: endpoint.action.clone(),
                    resource: endpoint.resource.clone(),
                    controller: endpoint.controller.clone(),
                    summary: endpoint.summary.clone(),
                    children: Some(Vec::new()),
                });
            }
        }

        controller_map.into_values().collect()
    }

    #[allow(dead_code)]
    async fn assign_endpoints(&self, role_id: &str, endpoint_ids: Vec<String>) -> Result<(), EndpointError> {
        let role_id = role_id.to_string();
        let endpoint_ids = std::sync::Arc::new(endpoint_ids);
        // 检查所有端点是否存在
        let endpoints = SysEndpoint::find()
            .filter(SysEndpointColumn::Id.is_in(endpoint_ids.as_ref().clone()))
            .all(&self.db)
            .await
            .map_err(|e| EndpointError::DatabaseError(Box::new(e)))?;

        if endpoints.len() != endpoint_ids.len() {
            let found_ids: Vec<String> = endpoints.iter().map(|e| e.id.to_string()).collect();
            let missing_ids: Vec<String> = endpoint_ids
                .iter()
                .filter(|id| !found_ids.contains(id))
                .cloned()
                .collect();
            error!("Some endpoints not found: {:?}", missing_ids);
            return Err(EndpointError::EndpointsNotFound(missing_ids.into_iter().filter_map(|id| id.parse::<i32>().ok()).collect()));
        }

        // 在事务中执行端点分配
        execute_in_transaction(&self.db, move |txn| {
            let role_id = role_id.clone();
            let endpoint_ids = endpoint_ids.clone();
            Box::pin(async move {
                // 获取现有端点
                let existing_endpoints = SysRoleMenu::find()
                    .filter(SysRoleMenuColumn::RoleId.eq(&role_id))
                    .all(&txn)
                    .await
                    .map_err(|e| EndpointError::DatabaseError(Box::new(e)))?;

                // 计算需要添加和删除的端点
                let existing_ids: Vec<String> = existing_endpoints
                    .iter()
                    .map(|e| e.menu_id.to_string())
                    .collect();
                let to_add: Vec<String> = endpoint_ids
                    .iter()
                    .filter(|id| !existing_ids.contains(id))
                    .cloned()
                    .collect();
                let to_delete: Vec<String> = existing_ids
                    .into_iter()
                    .filter(|id| !endpoint_ids.contains(id))
                    .collect();

                // 批量插入新端点
                if !to_add.is_empty() {
                    let role_menus: Vec<SysRoleMenuActiveModel> = to_add.clone().into_iter()
                        .filter_map(|id| id.parse::<i32>().ok().map(|menu_id| SysRoleMenuActiveModel {
                            role_id: Set(role_id.clone()),
                            menu_id: Set(menu_id),
                            ..Default::default()
                        }))
                        .collect();

                    if !role_menus.is_empty() {
                        SysRoleMenu::insert_many(role_menus)
                            .exec(&txn)
                            .await
                            .map_err(|e| EndpointError::DatabaseError(Box::new(e)))?;
                    }
                }

                // 批量删除旧端点
                if !to_delete.is_empty() {
                    let delete_ids: Vec<i32> = to_delete.clone().into_iter()
                        .filter_map(|id| id.parse::<i32>().ok())
                        .collect();

                    if !delete_ids.is_empty() {
                        SysRoleMenu::delete_many()
                            .filter(SysRoleMenuColumn::RoleId.eq(&role_id))
                            .filter(SysRoleMenuColumn::MenuId.is_in(delete_ids))
                            .exec(&txn)
                            .await
                            .map_err(|e| EndpointError::DatabaseError(Box::new(e)))?;
                    }
                }

                info!(
                    "Successfully assigned endpoints to role: role_id={}, added={}, deleted={}",
                    role_id,
                    to_add.len(),
                    to_delete.len()
                );

                Ok(())
            })
        })
        .await
        .map_err(|e| match e {
            AppError { code, message } => EndpointError::DatabaseError(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Database error (code {}): {}", code, message),
            ))),
        })
    }
}

#[async_trait]
impl TEndpointService for SysEndpointService {
    async fn sync_endpoints(&self, endpoints: Vec<SysEndpointModel>) -> Result<(), AppError> {
        let db = db_helper::get_db_connection().await?;

        execute_in_transaction(&db, move |mut txn| {
            Box::pin(async move {
                // Get existing endpoints
                let existing_endpoints = SysEndpoint::find()
                    .all(&mut txn)
                    .await
                    .map_err(AppError::from)?;

                // Create maps for easier lookup
                let existing_map: BTreeMap<String, SysEndpointModel> = existing_endpoints
                    .into_iter()
                    .map(|e| (e.path.clone(), e))
                    .collect();

                let new_map: BTreeMap<String, SysEndpointModel> = endpoints
                    .into_iter()
                    .map(|e| (e.path.clone(), e))
                    .collect();

                // Find endpoints to add, update, and delete
                let to_add: Vec<SysEndpointModel> = new_map
                    .values()
                    .filter(|e| !existing_map.contains_key(&e.path))
                    .cloned()
                    .collect();

                let to_update: Vec<SysEndpointModel> = new_map
                    .values()
                    .filter(|e| {
                        existing_map
                            .get(&e.path)
                            .map(|existing| existing.method != e.method)
                            .unwrap_or(false)
                    })
                    .cloned()
                    .collect();

                let to_delete: Vec<String> = existing_map
                    .keys()
                    .filter(|path| !new_map.contains_key(*path))
                    .cloned()
                    .collect();

                // Perform operations
                if !to_add.is_empty() {
                    let active_models: Vec<SysEndpointActiveModel> = to_add
                        .into_iter()
                        .map(|e| e.into_active_model())
                        .collect();
                    SysEndpoint::insert_many(active_models)
                        .exec(&mut txn)
                        .await
                        .map_err(AppError::from)?;
                }

                for endpoint in to_update {
                    let active_model = endpoint.into_active_model();
                    active_model.update(&mut txn).await.map_err(AppError::from)?;
                }

                if !to_delete.is_empty() {
                    SysEndpoint::delete_many()
                        .filter(SysEndpointColumn::Path.is_in(to_delete))
                        .exec(&mut txn)
                        .await
                        .map_err(AppError::from)?;
                }

                txn.commit().await.map_err(AppError::from)?;
                Ok(())
            })
        })
        .await
    }

    async fn find_paginated_endpoints(
        &self,
        params: EndpointPageRequest,
    ) -> Result<PaginatedData<SysEndpointModel>, AppError> {
        let db = db_helper::get_db_connection().await?;
        let mut query = SysEndpoint::find();

        if let Some(ref keywords) = params.keywords {
            let condition = Condition::any()
                .add(SysEndpointColumn::Path.contains(keywords))
                .add(SysEndpointColumn::Method.contains(keywords))
                .add(SysEndpointColumn::Controller.contains(keywords));
            query = query.filter(condition);
        }

        let total = query.clone().count(db.as_ref()).await.map_err(AppError::from)?;

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

    async fn tree_endpoint(&self) -> Result<Vec<EndpointTree>, AppError> {
        let db = db_helper::get_db_connection().await?;
        let endpoints = SysEndpoint::find().all(db.as_ref()).await.map_err(AppError::from)?;

        Ok(self.create_endpoint_tree(&endpoints))
    }
}
