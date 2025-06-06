/**
 * 系统菜单服务模块
 *
 * 该模块提供了菜单管理相关的核心功能，包括：
 * - 菜单树构建
 * - 菜单CRUD操作
 * - 菜单路由管理
 * - 角色菜单关联
 *
 * 主要组件
 * --------
 * - TMenuService: 菜单服务 trait，定义了菜单管理相关的核心接口
 * - SysMenuService: 菜单服务实现，提供了具体的菜单管理逻辑
 *
 * 功能特性
 * --------
 * - 菜单树：支持构建菜单树结构
 * - 菜单查询：支持获取菜单列表和常量路由
 * - 菜单创建：支持创建新菜单，包括路由名称唯一性检查
 * - 菜单更新：支持更新菜单信息，包括父菜单和循环引用检查
 * - 菜单删除：支持删除菜单
 * - 角色菜单：支持获取角色关联的菜单ID
 *
 * 使用示例
 * --------
 *
 * use server_service::admin::sys_menu_service::*;
 *
 * // 创建菜单服务实例
 * let menu_service = SysMenuService;
 *
 * // 获取菜单树
 * let menu_tree = menu_service.tree_menu().await?;
 *
 * // 创建新菜单
 * let menu = menu_service.create_menu(CreateMenuInput {
 *     menu_type: MenuType::Menu,
 *     menu_name: "用户管理".to_string(),
 *     icon_type: "icon".to_string(),
 *     icon: "user".to_string(),
 *     route_name: "user".to_string(),
 *     route_path: "/user".to_string(),
 *     component: "user/index".to_string(),
 *     path_param: None,
 *     status: Status::Enabled,
 *     active_menu: None,
 *     hide_in_menu: false,
 *     pid: "0".to_string(),
 *     sequence: 1,
 *     i18n_key: "menu.user".to_string(),
 *     keep_alive: true,
 *     constant: false,
 *     href: None,
 *     multi_tab: false,
 * }).await?;
 */

use async_trait::async_trait;
use chrono::Local;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, Set,
    IntoActiveModel, QuerySelect,
};
use server_model::admin::{
    entities::{
        prelude::{SysMenu, SysRoleMenu},
        sea_orm_active_enums::{MenuType, Status},
        sys_menu::{ActiveModel as SysMenuActiveModel, Column as SysMenuColumn, Model as SysMenuModel},
        sys_role_menu::Column as SysRoleMenuColumn,
    },
    input::{CreateMenuInput, UpdateMenuInput},
    output::{MenuRoute, MenuTree, RouteMeta},
};
use server_utils::TreeBuilder;

use crate::{
    admin::errors::sys_menu_error::MenuError,
    helper::db_helper,
};

/**
 * 菜单服务 trait
 *
 * 定义了菜单管理相关的核心接口，包括：
 * - 菜单树构建
 * - 菜单列表获取
 * - 常量路由获取
 * - 菜单CRUD操作
 * - 角色菜单关联
 *
 * 使用示例
 * --------
 *
 * use server_service::admin::sys_menu_service::*;
 *
 * let menu_service = SysMenuService;
 *
 * // 获取菜单树
 * let menu_tree = menu_service.tree_menu().await?;
 *
 * // 获取菜单列表
 * let menu_list = menu_service.get_menu_list().await?;
 */
#[async_trait]
pub trait TMenuService {
    /**
     * 获取菜单树
     *
     * 获取启用状态的菜单树结构
     *
     * @return Result<Vec<MenuTree>, MenuError> 菜单树或错误
     */
    async fn tree_menu(&self) -> Result<Vec<MenuTree>, MenuError>;

    /**
     * 获取菜单列表
     *
     * 获取所有菜单的树形结构
     *
     * @return Result<Vec<MenuTree>, MenuError> 菜单树或错误
     */
    async fn get_menu_list(&self) -> Result<Vec<MenuTree>, MenuError>;

    /**
     * 获取常量路由
     *
     * 获取所有启用状态的常量路由
     *
     * @return Result<Vec<MenuRoute>, MenuError> 常量路由列表或错误
     */
    async fn get_constant_routes(&self) -> Result<Vec<MenuRoute>, MenuError>;

    /**
     * 创建菜单
     *
     * 创建新菜单，包括路由名称唯一性检查
     *
     * @param input 菜单创建参数
     * @return Result<SysMenuModel, MenuError> 创建的菜单信息或错误
     */
    async fn create_menu(&self, input: CreateMenuInput) -> Result<SysMenuModel, MenuError>;

    /**
     * 获取菜单
     *
     * 根据菜单ID获取菜单信息
     *
     * @param id 菜单ID
     * @return Result<SysMenuModel, MenuError> 菜单信息或错误
     */
    async fn get_menu(&self, id: i32) -> Result<SysMenuModel, MenuError>;

    /**
     * 更新菜单
     *
     * 更新菜单信息，包括父菜单和循环引用检查
     *
     * @param id 菜单ID
     * @param input 菜单更新参数
     * @return Result<SysMenuModel, MenuError> 更新后的菜单信息或错误
     */
    async fn update_menu(&self, id: i32, input: UpdateMenuInput) -> Result<SysMenuModel, MenuError>;

    /**
     * 删除菜单
     *
     * 根据菜单ID删除菜单
     *
     * @param id 菜单ID
     * @return Result<(), MenuError> 删除结果
     */
    async fn delete_menu(&self, id: i32) -> Result<(), MenuError>;

    /**
     * 获取角色菜单ID列表
     *
     * 获取指定角色关联的所有菜单ID
     *
     * @param role_id 角色ID
     * @param domain 域代码
     * @return Result<Vec<i32>, MenuError> 菜单ID列表或错误
     */
    async fn get_menu_ids_by_role_id(&self, role_id: String, domain: String) -> Result<Vec<i32>, MenuError>;
}

/**
 * 系统菜单服务
 *
 * 实现了菜单管理相关的核心功能，包括：
 * - 菜单树构建
 * - 菜单CRUD操作
 * - 菜单路由管理
 * - 角色菜单关联
 *
 * 使用示例
 * --------
 *
 * use server_service::admin::sys_menu_service::*;
 *
 * let menu_service = SysMenuService;
 *
 * // 获取菜单树
 * let menu_tree = menu_service.tree_menu().await?;
 *
 * // 创建菜单
 * let menu = menu_service.create_menu(CreateMenuInput {
 *     menu_type: MenuType::Menu,
 *     menu_name: "用户管理".to_string(),
 *     icon_type: "icon".to_string(),
 *     icon: "user".to_string(),
 *     route_name: "user".to_string(),
 *     route_path: "/user".to_string(),
 *     component: "user/index".to_string(),
 *     path_param: None,
 *     status: Status::Enabled,
 *     active_menu: None,
 *     hide_in_menu: false,
 *     pid: "0".to_string(),
 *     sequence: 1,
 *     i18n_key: "menu.user".to_string(),
 *     keep_alive: true,
 *     constant: false,
 *     href: None,
 *     multi_tab: false,
 * }).await?;
 */
#[derive(Clone)]
pub struct SysMenuService;

impl SysMenuService {
    /**
     * 构建菜单树节点
     *
     * 将菜单模型转换为菜单树节点
     *
     * @param menu 菜单模型
     * @return MenuTree 菜单树节点
     */
    fn build_menu_tree(menu: &SysMenuModel) -> MenuTree {
        MenuTree {
            id: menu.id,
            pid: menu.pid.clone(),
            menu_type: menu.menu_type.clone(),
            menu_name: menu.menu_name.clone(),
            icon_type: menu.icon_type.clone(),
            icon: menu.icon.clone(),
            route_name: menu.route_name.clone(),
            route_path: menu.route_path.clone(),
            component: menu.component.clone(),
            path_param: menu.path_param.clone(),
            status: menu.status.clone(),
            active_menu: menu.active_menu.clone(),
            hide_in_menu: menu.hide_in_menu,
            sequence: menu.sequence,
            i18n_key: menu.i18n_key.clone(),
            keep_alive: menu.keep_alive,
            constant: menu.constant,
            href: menu.href.clone(),
            multi_tab: menu.multi_tab,
            created_at: menu.created_at,
            created_by: menu.created_by.clone(),
            updated_at: menu.updated_at,
            updated_by: menu.updated_by.clone(),
            children: None,
        }
    }

    /**
     * 构建树形结构
     *
     * 将菜单树节点列表构建为树形结构
     *
     * @param menu_trees 菜单树节点列表
     * @return Vec<MenuTree> 树形结构的菜单树
     */
    fn build_tree_structure(menu_trees: Vec<MenuTree>) -> Vec<MenuTree> {
        TreeBuilder::build(
            menu_trees,
            |node| node.id,
            |node| {
                if node.pid == "0" {
                    None
                } else {
                    Some(node.pid.parse::<i32>().unwrap_or(-1))
                }
            },
            |node| node.sequence,
            |node, children| node.children = Some(children),
        )
    }

    /**
     * 检查路由名称唯一性
     *
     * 检查路由名称是否已存在，支持排除当前菜单
     *
     * @param route_name 路由名称
     * @param exclude_id 排除的菜单ID（可选）
     * @return Result<(), MenuError> 检查结果
     *
     * 错误
     * -----
     * - DuplicateRouteName: 路由名称已存在
     */
    async fn check_route_name_unique(&self, route_name: &str, exclude_id: Option<i32>) -> Result<(), MenuError> {
        let db = db_helper::get_db_connection().await?;
        let mut query = SysMenu::find().filter(SysMenuColumn::RouteName.eq(route_name));
        
        if let Some(id) = exclude_id {
            query = query.filter(SysMenuColumn::Id.ne(id));
        }
        
        let exists = query.one(db.as_ref()).await.map_err(MenuError::from)?;
        if exists.is_some() {
            return Err(MenuError::DuplicateRouteName);
        }
        Ok(())
    }

    /**
     * 检查父菜单
     *
     * 检查父菜单是否存在且为目录类型
     *
     * @param pid 父菜单ID
     * @return Result<(), MenuError> 检查结果
     *
     * 错误
     * -----
     * - ParentMenuNotFound: 父菜单不存在
     * - ParentNotDirectory: 父菜单不是目录类型
     */
    async fn check_parent_menu(&self, pid: &str) -> Result<(), MenuError> {
        if pid == "0" {
            return Ok(());
        }
        
        let db = db_helper::get_db_connection().await?;
        let parent = SysMenu::find()
            .filter(SysMenuColumn::Id.eq(pid.parse::<i32>().unwrap_or(0)))
            .one(db.as_ref())
            .await
            .map_err(MenuError::from)?;
        
        match parent {
            Some(menu) if menu.menu_type == MenuType::Directory => Ok(()),
            Some(_) => Err(MenuError::ParentNotDirectory),
            None => Err(MenuError::ParentMenuNotFound),
        }
    }

    /**
     * 检查循环引用
     *
     * 检查菜单的父子关系是否存在循环引用
     *
     * @param id 当前菜单ID
     * @param pid 父菜单ID
     * @return Result<(), MenuError> 检查结果
     *
     * 错误
     * -----
     * - CircularReference: 存在循环引用
     */
    async fn check_circular_reference(&self, id: i32, pid: &str) -> Result<(), MenuError> {
        if pid == "0" {
            return Ok(());
        }
        
        let db = db_helper::get_db_connection().await?;
        let mut current_id = pid.parse::<i32>().unwrap_or(0);
        let mut visited = std::collections::HashSet::new();
        visited.insert(id);
        
        while current_id != 0 {
            if visited.contains(&current_id) {
                return Err(MenuError::CircularReference);
            }
            visited.insert(current_id);
            
            let parent = SysMenu::find()
                .filter(SysMenuColumn::Id.eq(current_id))
                .one(db.as_ref())
                .await
                .map_err(MenuError::from)?;
            
            match parent {
                Some(menu) => current_id = menu.pid.parse::<i32>().unwrap_or(0),
                None => break,
            }
        }
        Ok(())
    }
}

#[async_trait]
impl TMenuService for SysMenuService {
    /**
     * 获取菜单树
     *
     * 获取启用状态的菜单树结构
     *
     * @return Result<Vec<MenuTree>, MenuError> 菜单树或错误
     */
    async fn tree_menu(&self) -> Result<Vec<MenuTree>, MenuError> {
        let db = db_helper::get_db_connection().await?;
        let menus = SysMenu::find()
            .filter(SysMenuColumn::Status.eq(Status::Enabled))
            .order_by(SysMenuColumn::Sequence, sea_orm::Order::Asc)
            .all(db.as_ref())
            .await
            .map_err(MenuError::from)?;
        
        let menu_trees: Vec<MenuTree> = menus.iter().map(Self::build_menu_tree).collect();
        Ok(Self::build_tree_structure(menu_trees))
    }

    /**
     * 获取菜单列表
     *
     * 获取所有菜单的树形结构
     *
     * @return Result<Vec<MenuTree>, MenuError> 菜单树或错误
     */
    async fn get_menu_list(&self) -> Result<Vec<MenuTree>, MenuError> {
        let db = db_helper::get_db_connection().await?;
        let menus = SysMenu::find()
            .order_by(SysMenuColumn::Sequence, sea_orm::Order::Asc)
            .all(db.as_ref())
            .await
            .map_err(MenuError::from)?;
        
        let menu_trees: Vec<MenuTree> = menus.iter().map(Self::build_menu_tree).collect();
        Ok(Self::build_tree_structure(menu_trees))
    }

    /**
     * 获取常量路由
     *
     * 获取所有启用状态的常量路由
     *
     * @return Result<Vec<MenuRoute>, MenuError> 常量路由列表或错误
     */
    async fn get_constant_routes(&self) -> Result<Vec<MenuRoute>, MenuError> {
        let db = db_helper::get_db_connection().await?;
        let menus = SysMenu::find()
            .filter(SysMenuColumn::Constant.eq(true))
            .filter(SysMenuColumn::Status.eq(Status::Enabled))
            .order_by(SysMenuColumn::Sequence, sea_orm::Order::Asc)
            .all(db.as_ref())
            .await
            .map_err(MenuError::from)?;
        
        let routes = menus
            .into_iter()
            .map(|menu| MenuRoute {
                id: menu.id,
                pid: menu.pid,
                name: menu.route_name,
                path: menu.route_path,
                component: menu.component,
                meta: RouteMeta {
                    title: menu.menu_name,
                    i18n_key: menu.i18n_key,
                    keep_alive: menu.keep_alive,
                    constant: menu.constant,
                    icon: menu.icon,
                    order: menu.sequence,
                    href: menu.href,
                    hide_in_menu: menu.hide_in_menu,
                    active_menu: menu.active_menu,
                    multi_tab: menu.multi_tab,
                },
                children: vec![].into(),
            })
            .collect();
        
        Ok(routes)
    }

    /**
     * 创建菜单
     *
     * 创建新菜单，包括路由名称唯一性检查
     *
     * @param input 菜单创建参数
     * @return Result<SysMenuModel, MenuError> 创建的菜单信息或错误
     */
    async fn create_menu(&self, input: CreateMenuInput) -> Result<SysMenuModel, MenuError> {
        // 检查路由名称唯一性
        self.check_route_name_unique(&input.route_name, None).await?;
        
        // 检查父菜单
        self.check_parent_menu(&input.pid).await?;
        
        // 检查循环引用
        self.check_circular_reference(0, &input.pid).await?;
        
        let db = db_helper::get_db_connection().await?;
        let now = Local::now().naive_local();
        
        let menu = SysMenuActiveModel {
            menu_type: Set(input.menu_type),
            menu_name: Set(input.menu_name),
            icon_type: Set(input.icon_type),
            icon: Set(input.icon),
            route_name: Set(input.route_name),
            route_path: Set(input.route_path),
            component: Set(input.component),
            path_param: Set(input.path_param),
            status: Set(input.status),
            active_menu: Set(input.active_menu),
            hide_in_menu: Set(input.hide_in_menu),
            pid: Set(input.pid),
            sequence: Set(input.sequence),
            i18n_key: Set(input.i18n_key),
            keep_alive: Set(input.keep_alive),
            constant: Set(input.constant),
            href: Set(input.href),
            multi_tab: Set(input.multi_tab),
            created_at: Set(now),
            created_by: Set("system".to_string()),
            ..Default::default()
        };
        
        menu.insert(db.as_ref())
            .await
            .map_err(MenuError::from)
    }

    /**
     * 获取菜单
     *
     * 根据菜单ID获取菜单信息
     *
     * @param id 菜单ID
     * @return Result<SysMenuModel, MenuError> 菜单信息或错误
     */
    async fn get_menu(&self, id: i32) -> Result<SysMenuModel, MenuError> {
        let db = db_helper::get_db_connection().await?;
        SysMenu::find_by_id(id)
            .one(db.as_ref())
            .await
            .map_err(MenuError::from)?
            .ok_or(MenuError::MenuNotFound)
    }

    /**
     * 更新菜单
     *
     * 更新菜单信息，包括父菜单和循环引用检查
     *
     * @param id 菜单ID
     * @param input 菜单更新参数
     * @return Result<SysMenuModel, MenuError> 更新后的菜单信息或错误
     */
    async fn update_menu(&self, id: i32, input: UpdateMenuInput) -> Result<SysMenuModel, MenuError> {
        let menu = self.get_menu(id).await?;
        
        // 检查路由名称唯一性
        if input.menu.route_name != menu.route_name {
            self.check_route_name_unique(&input.menu.route_name, Some(id)).await?;
        }
        
        // 检查父菜单
        if input.menu.pid != menu.pid {
            self.check_parent_menu(&input.menu.pid).await?;
            self.check_circular_reference(id, &input.menu.pid).await?;
        }
        
        let db = db_helper::get_db_connection().await?;
        let mut menu = menu.into_active_model();
        
        menu.menu_type = Set(input.menu.menu_type);
        menu.menu_name = Set(input.menu.menu_name);
        menu.icon_type = Set(input.menu.icon_type);
        menu.icon = Set(input.menu.icon);
        menu.route_name = Set(input.menu.route_name);
        menu.route_path = Set(input.menu.route_path);
        menu.component = Set(input.menu.component);
        menu.path_param = Set(input.menu.path_param);
        menu.status = Set(input.menu.status);
        menu.active_menu = Set(input.menu.active_menu);
        menu.hide_in_menu = Set(input.menu.hide_in_menu);
        menu.pid = Set(input.menu.pid);
        menu.sequence = Set(input.menu.sequence);
        menu.i18n_key = Set(input.menu.i18n_key);
        menu.keep_alive = Set(input.menu.keep_alive);
        menu.constant = Set(input.menu.constant);
        menu.href = Set(input.menu.href);
        menu.multi_tab = Set(input.menu.multi_tab);
        menu.updated_at = Set(Some(Local::now().naive_local()));
        menu.updated_by = Set(Some("system".to_string()));
        
        menu.update(db.as_ref())
            .await
            .map_err(MenuError::from)
    }

    /**
     * 删除菜单
     *
     * 根据菜单ID删除菜单
     *
     * @param id 菜单ID
     * @return Result<(), MenuError> 删除结果
     */
    async fn delete_menu(&self, id: i32) -> Result<(), MenuError> {
        let db = db_helper::get_db_connection().await?;
        
        // 检查是否存在子菜单
        let has_children = SysMenu::find()
            .filter(SysMenuColumn::Pid.eq(id.to_string()))
            .one(db.as_ref())
            .await
            .map_err(MenuError::from)?
            .is_some();
        
        if has_children {
            return Err(MenuError::HasChildren);
        }
        
        // 检查是否被角色使用
        let in_use = SysRoleMenu::find()
            .filter(SysRoleMenuColumn::MenuId.eq(id))
            .one(db.as_ref())
            .await
            .map_err(MenuError::from)?
            .is_some();
        
        if in_use {
            return Err(MenuError::InUse);
        }
        
        SysMenu::delete_by_id(id)
            .exec(db.as_ref())
            .await
            .map_err(MenuError::from)?;
        
        Ok(())
    }

    /**
     * 获取角色菜单ID列表
     *
     * 获取指定角色关联的所有菜单ID
     *
     * @param role_id 角色ID
     * @param domain 域代码
     * @return Result<Vec<i32>, MenuError> 菜单ID列表或错误
     */
    async fn get_menu_ids_by_role_id(&self, role_id: String, domain: String) -> Result<Vec<i32>, MenuError> {
        let db = db_helper::get_db_connection().await?;
        SysRoleMenu::find()
            .filter(SysRoleMenuColumn::RoleId.eq(role_id))
            .filter(SysRoleMenuColumn::Domain.eq(domain))
            .select_only()
            .column(SysRoleMenuColumn::MenuId)
            .into_tuple()
            .all(db.as_ref())
            .await
            .map_err(MenuError::from)
    }
}
