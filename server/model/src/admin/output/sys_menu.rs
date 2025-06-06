/**
 * 菜单相关输出参数定义
 * 
 * 包含菜单路由、元数据和树形结构的输出结构体。
 */

use chrono::NaiveDateTime;
use serde::Serialize;

use crate::admin::entities::sea_orm_active_enums::{MenuType, Status};

/**
 * 菜单路由输出参数
 * 
 * 用于返回菜单的路由信息。
 */
#[derive(Debug, Serialize, Clone)]
pub struct MenuRoute {
    /** 路由名称 */
    pub name: String,
    /** 路由路径 */
    pub path: String,
    /** 组件路径 */
    pub component: String,
    /** 路由元数据 */
    pub meta: RouteMeta,
    /** 子路由列表 */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<MenuRoute>>,
    /** 菜单ID */
    pub id: i32,
    /** 父级菜单ID */
    pub pid: String,
}

/**
 * 路由元数据输出参数
 * 
 * 用于返回路由的元数据信息。
 */
#[derive(Debug, Serialize, Clone)]
pub struct RouteMeta {
    /** 标题 */
    pub title: String,
    /** 国际化键值 */
    #[serde(skip_serializing_if = "Option::is_none", rename = "i18nKey")]
    pub i18n_key: Option<String>,
    /** 是否保持活跃 */
    #[serde(skip_serializing_if = "Option::is_none", rename = "keepAlive")]
    pub keep_alive: Option<bool>,
    /** 是否常量路由 */
    pub constant: bool,
    /** 图标 */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /** 排序 */
    pub order: i32,
    /** 外部链接 */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /** 是否在菜单中隐藏 */
    #[serde(skip_serializing_if = "Option::is_none", rename = "hideInMenu")]
    pub hide_in_menu: Option<bool>,
    /** 激活菜单 */
    #[serde(skip_serializing_if = "Option::is_none", rename = "activeMenu")]
    pub active_menu: Option<String>,
    /** 是否支持多标签 */
    #[serde(skip_serializing_if = "Option::is_none", rename = "multiTab")]
    pub multi_tab: Option<bool>,
}

/**
 * 菜单树形结构输出参数
 * 
 * 用于返回菜单的树形结构信息。
 */
#[derive(Debug, Serialize, Clone)]
pub struct MenuTree {
    /** 菜单ID */
    pub id: i32,
    /** 父级菜单ID */
    pub pid: String,
    /** 菜单类型 */
    #[serde(rename = "menuType")]
    pub menu_type: MenuType,
    /** 菜单名称 */
    #[serde(rename = "menuName")]
    pub menu_name: String,
    /** 图标类型 */
    #[serde(skip_serializing_if = "Option::is_none", rename = "iconType")]
    pub icon_type: Option<i32>,
    /** 图标 */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /** 路由名称 */
    #[serde(rename = "routeName")]
    pub route_name: String,
    /** 路由路径 */
    #[serde(rename = "routePath")]
    pub route_path: String,
    /** 组件路径 */
    pub component: String,
    /** 路径参数 */
    #[serde(skip_serializing_if = "Option::is_none", rename = "pathParam")]
    pub path_param: Option<String>,
    /** 状态 */
    pub status: Status,
    /** 激活菜单 */
    #[serde(skip_serializing_if = "Option::is_none", rename = "activeMenu")]
    pub active_menu: Option<String>,
    /** 是否在菜单中隐藏 */
    #[serde(skip_serializing_if = "Option::is_none", rename = "hideInMenu")]
    pub hide_in_menu: Option<bool>,
    /** 排序 */
    pub sequence: i32,
    /** 国际化键值 */
    #[serde(skip_serializing_if = "Option::is_none", rename = "i18nKey")]
    pub i18n_key: Option<String>,
    /** 是否保持活跃 */
    #[serde(skip_serializing_if = "Option::is_none", rename = "keepAlive")]
    pub keep_alive: Option<bool>,
    /** 是否常量路由 */
    pub constant: bool,
    /** 外部链接 */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /** 是否支持多标签 */
    #[serde(skip_serializing_if = "Option::is_none", rename = "multiTab")]
    pub multi_tab: Option<bool>,
    /** 创建时间 */
    #[serde(rename = "createdAt")]
    pub created_at: NaiveDateTime,
    /** 创建人 */
    #[serde(rename = "createdBy")]
    pub created_by: String,
    /** 更新时间 */
    #[serde(skip_serializing_if = "Option::is_none", rename = "updatedAt")]
    pub updated_at: Option<NaiveDateTime>,
    /** 更新人 */
    #[serde(skip_serializing_if = "Option::is_none", rename = "updatedBy")]
    pub updated_by: Option<String>,
    /** 子菜单列表 */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<MenuTree>>,
}
