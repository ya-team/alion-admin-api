use std::sync::Arc;

use axum::{
    extract::{Path, Extension},
    Json,
};
use server_core::web::{error::AppError, res::Res};
use server_model::admin::{
    entities::sys_menu::Model as SysMenuModel,
    input::{CreateMenuInput, UpdateMenuInput},
    output::{MenuRoute, MenuTree},
};
use server_service::admin::{
    SysMenuService,
    TMenuService,
};

pub struct SysMenuApi;

impl SysMenuApi {
    pub async fn tree_menu(
        Extension(service): Extension<Arc<SysMenuService>>,
    ) -> Result<Json<Res<Vec<MenuTree>>>, AppError> {
        let result = service.tree_menu().await?;
        Ok(Json(Res::new_data(result)))
    }

    pub async fn get_menu_list(
        Extension(service): Extension<Arc<SysMenuService>>,
    ) -> Result<Json<Res<Vec<MenuTree>>>, AppError> {
        let result = service.get_menu_list().await?;
        Ok(Json(Res::new_data(result)))
    }

    pub async fn get_constant_routes(
        Extension(service): Extension<Arc<SysMenuService>>,
    ) -> Result<Json<Res<Vec<MenuRoute>>>, AppError> {
        let result = service.get_constant_routes().await?;
        Ok(Json(Res::new_data(result)))
    }

    pub async fn create_menu(
        Extension(service): Extension<Arc<SysMenuService>>,
        Json(input): Json<CreateMenuInput>,
    ) -> Result<Json<Res<SysMenuModel>>, AppError> {
        let result = service.create_menu(input).await?;
        Ok(Json(Res::new_data(result)))
    }

    pub async fn get_menu(
        Extension(service): Extension<Arc<SysMenuService>>,
        Path(id): Path<i32>,
    ) -> Result<Json<Res<SysMenuModel>>, AppError> {
        let result = service.get_menu(id).await?;
        Ok(Json(Res::new_data(result)))
    }

    pub async fn update_menu(
        Extension(service): Extension<Arc<SysMenuService>>,
        Path(id): Path<i32>,
        Json(input): Json<UpdateMenuInput>,
    ) -> Result<Json<Res<SysMenuModel>>, AppError> {
        let result = service.update_menu(id, input).await?;
        Ok(Json(Res::new_data(result)))
    }

    pub async fn delete_menu(
        Extension(service): Extension<Arc<SysMenuService>>,
        Path(id): Path<i32>,
    ) -> Result<Json<Res<()>>, AppError> {
        let result = service.delete_menu(id).await?;
        Ok(Json(Res::new_data(result)))
    }

    pub async fn get_menu_ids_by_role_id(
        Extension(service): Extension<Arc<SysMenuService>>,
        Path((role_id, domain)): Path<(i32, String)>,
    ) -> Result<Json<Res<Vec<i32>>>, AppError> {
        let result = service
            .get_menu_ids_by_role_id(role_id.to_string(), domain)
            .await?;
        Ok(Json(Res::new_data(result)))
    }
}
