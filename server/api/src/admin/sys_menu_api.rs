/**
 * 菜单管理API
 * 
 * 提供菜单管理的CRUD操作接口，包括：
 * - 获取菜单树形结构
 * - 获取菜单列表
 * - 获取常量路由
 * - 创建菜单
 * - 获取菜单详情
 * - 更新菜单
 * - 删除菜单
 * - 获取角色关联的菜单ID列表
 */
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
    /**
     * 获取菜单树形结构
     * 
     * # 参数
     * - service: 菜单服务实例
     * 
     * # 返回
     * 返回菜单的树形结构数据
     */
    pub async fn tree_menu(
        Extension(service): Extension<Arc<SysMenuService>>,
    ) -> Result<Json<Res<Vec<MenuTree>>>, AppError> {
        let result = service.tree_menu().await?;
        Ok(Json(Res::new_data(result)))
    }

    /**
     * 获取菜单列表
     * 
     * # 参数
     * - service: 菜单服务实例
     * 
     * # 返回
     * 返回菜单列表数据
     */
    pub async fn get_menu_list(
        Extension(service): Extension<Arc<SysMenuService>>,
    ) -> Result<Json<Res<Vec<MenuTree>>>, AppError> {
        let result = service.get_menu_list().await?;
        Ok(Json(Res::new_data(result)))
    }

    /**
     * 获取常量路由
     * 
     * # 参数
     * - service: 菜单服务实例
     * 
     * # 返回
     * 返回常量路由列表
     */
    pub async fn get_constant_routes(
        Extension(service): Extension<Arc<SysMenuService>>,
    ) -> Result<Json<Res<Vec<MenuRoute>>>, AppError> {
        let result = service.get_constant_routes().await?;
        Ok(Json(Res::new_data(result)))
    }

    /**
     * 创建菜单
     * 
     * # 参数
     * - service: 菜单服务实例
     * - input: 创建菜单的输入参数
     * 
     * # 返回
     * 返回新创建的菜单信息
     */
    pub async fn create_menu(
        Extension(service): Extension<Arc<SysMenuService>>,
        Json(input): Json<CreateMenuInput>,
    ) -> Result<Json<Res<SysMenuModel>>, AppError> {
        let result = service.create_menu(input).await?;
        Ok(Json(Res::new_data(result)))
    }

    /**
     * 获取菜单详情
     * 
     * # 参数
     * - service: 菜单服务实例
     * - id: 菜单ID
     * 
     * # 返回
     * 返回指定菜单的详细信息
     */
    pub async fn get_menu(
        Extension(service): Extension<Arc<SysMenuService>>,
        Path(id): Path<i32>,
    ) -> Result<Json<Res<SysMenuModel>>, AppError> {
        let result = service.get_menu(id).await?;
        Ok(Json(Res::new_data(result)))
    }

    /**
     * 更新菜单
     * 
     * # 参数
     * - service: 菜单服务实例
     * - id: 菜单ID
     * - input: 更新菜单的输入参数
     * 
     * # 返回
     * 返回更新后的菜单信息
     */
    pub async fn update_menu(
        Extension(service): Extension<Arc<SysMenuService>>,
        Path(id): Path<i32>,
        Json(input): Json<UpdateMenuInput>,
    ) -> Result<Json<Res<SysMenuModel>>, AppError> {
        let result = service.update_menu(id, input).await?;
        Ok(Json(Res::new_data(result)))
    }

    /**
     * 删除菜单
     * 
     * # 参数
     * - service: 菜单服务实例
     * - id: 要删除的菜单ID
     * 
     * # 返回
     * 返回删除操作的结果
     */
    pub async fn delete_menu(
        Extension(service): Extension<Arc<SysMenuService>>,
        Path(id): Path<i32>,
    ) -> Result<Json<Res<()>>, AppError> {
        let result = service.delete_menu(id).await?;
        Ok(Json(Res::new_data(result)))
    }

    /**
     * 获取角色关联的菜单ID列表
     * 
     * # 参数
     * - service: 菜单服务实例
     * - role_id: 角色ID
     * - domain: 域
     * 
     * # 返回
     * 返回角色关联的菜单ID列表
     */
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
