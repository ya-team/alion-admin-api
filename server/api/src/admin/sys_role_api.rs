use std::sync::Arc;

use axum::{
    extract::{Path, Query},
    Extension,
    Json,
};
use server_core::web::{
    error::AppError,
    page::PaginatedData,
    res::Res,
};
use server_model::admin::{
    entities::sys_role::Model as SysRoleModel,
    input::{CreateRoleInput, RolePageRequest, UpdateRoleInput},
};
use server_service::admin::{
    SysRoleService,
    TRoleService,
};

pub struct SysRoleApi;

impl SysRoleApi {
    pub async fn find_paginated_roles(
        Extension(service): Extension<Arc<SysRoleService>>,
        Query(params): Query<RolePageRequest>,
    ) -> Result<Json<Res<PaginatedData<SysRoleModel>>>, AppError> {
        let result = service.find_paginated_roles(params).await?;
        Ok(Json(Res::new_data(result)))
    }

    pub async fn create_role(
        Extension(service): Extension<Arc<SysRoleService>>,
        Json(input): Json<CreateRoleInput>,
    ) -> Result<Json<Res<SysRoleModel>>, AppError> {
        let result = service.create_role(input).await?;
        Ok(Json(Res::new_data(result)))
    }

    pub async fn get_role(
        Extension(service): Extension<Arc<SysRoleService>>,
        Path(id): Path<i32>,
    ) -> Result<Json<Res<SysRoleModel>>, AppError> {
        let result = service.get_role(&id.to_string()).await?;
        Ok(Json(Res::new_data(result)))
    }

    pub async fn update_role(
        Extension(service): Extension<Arc<SysRoleService>>,
        Json(input): Json<UpdateRoleInput>,
    ) -> Result<Json<Res<SysRoleModel>>, AppError> {
        let result = service.update_role(input).await?;
        Ok(Json(Res::new_data(result)))
    }

    pub async fn delete_role(
        Extension(service): Extension<Arc<SysRoleService>>,
        Path(id): Path<i32>,
    ) -> Result<Json<Res<()>>, AppError> {
        let result = service.delete_role(&id.to_string()).await?;
        Ok(Json(Res::new_data(result)))
    }
}
