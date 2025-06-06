use async_trait::async_trait;
use chrono::Local;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, QueryFilter, QueryOrder, Set,
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

#[async_trait]
pub trait TMenuService {
    async fn tree_menu(&self) -> Result<Vec<MenuTree>, MenuError>;
    async fn get_menu_list(&self) -> Result<Vec<MenuTree>, MenuError>;
    async fn get_constant_routes(&self) -> Result<Vec<MenuRoute>, MenuError>;
    async fn create_menu(&self, input: CreateMenuInput) -> Result<SysMenuModel, MenuError>;
    async fn get_menu(&self, id: i32) -> Result<SysMenuModel, MenuError>;
    async fn update_menu(&self, id: i32, input: UpdateMenuInput) -> Result<SysMenuModel, MenuError>;
    async fn delete_menu(&self, id: i32) -> Result<(), MenuError>;
    async fn get_menu_ids_by_role_id(&self, role_id: String, domain: String) -> Result<Vec<i32>, MenuError>;
}

#[derive(Clone)]
pub struct SysMenuService;

impl SysMenuService {
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

    async fn get_menu(&self, id: i32) -> Result<SysMenuModel, MenuError> {
        let db = db_helper::get_db_connection().await?;
        SysMenu::find_by_id(id)
            .one(db.as_ref())
            .await
            .map_err(MenuError::from)?
            .ok_or(MenuError::MenuNotFound)
    }

    async fn update_menu(&self, id: i32, input: UpdateMenuInput) -> Result<SysMenuModel, MenuError> {
        let menu = self.get_menu(id).await?;
        
        // 检查路由名称唯一性
        if menu.route_name != input.menu.route_name {
            self.check_route_name_unique(&input.menu.route_name, Some(id)).await?;
        }
        
        // 检查父菜单
        if menu.pid != input.menu.pid {
            self.check_parent_menu(&input.menu.pid).await?;
            self.check_circular_reference(id, &input.menu.pid).await?;
        }
        
        let db = db_helper::get_db_connection().await?;
        let mut menu: SysMenuActiveModel = menu.into();
        
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

    async fn delete_menu(&self, id: i32) -> Result<(), MenuError> {
        let menu = self.get_menu(id).await?;
        
        // 检查是否有子菜单
        let db = db_helper::get_db_connection().await?;
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
        
        let menu: SysMenuActiveModel = menu.into();
        menu.delete(db.as_ref())
            .await
            .map_err(MenuError::from)?;
        
        Ok(())
    }

    async fn get_menu_ids_by_role_id(&self, role_id: String, domain: String) -> Result<Vec<i32>, MenuError> {
        let db = db_helper::get_db_connection().await?;
        
        let role_menus = SysRoleMenu::find()
            .filter(
                Condition::all()
                    .add(SysRoleMenuColumn::RoleId.eq(role_id))
                    .add(SysRoleMenuColumn::Domain.eq(domain)),
            )
            .all(db.as_ref())
            .await
            .map_err(MenuError::from)?;
        
        let menu_ids: Vec<i32> = role_menus.iter().map(|rm| rm.menu_id).collect();
        
        if menu_ids.is_empty() {
            return Ok(vec![]);
        }
        
        let menus = SysMenu::find()
            .filter(
                Condition::all()
                    .add(SysMenuColumn::Id.is_in(menu_ids))
                    .add(SysMenuColumn::Status.eq(Status::Enabled))
                    .add(SysMenuColumn::Constant.eq(false)),
            )
            .all(db.as_ref())
            .await
            .map_err(MenuError::from)?;
        
        Ok(menus.iter().map(|menu| menu.id).collect())
    }
}
