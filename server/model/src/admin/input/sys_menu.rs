use serde::{Deserialize, Serialize};
use validator::Validate;
use server_core::web::page::PageRequest;
use crate::admin::entities::sea_orm_active_enums::{Status, MenuType};

// 分页请求
#[derive(Debug, Serialize, Deserialize)]
pub struct MenuPageRequest {
    #[serde(flatten)]
    pub page_details: PageRequest,
    #[serde(default)]
    pub keywords: Option<String>,
    #[serde(default)]
    pub status: Option<Status>,
}

// 菜单输入
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct MenuInput {
    #[validate(length(min = 1, max = 50, message = "Menu name must be between 1 and 50 characters"))]
    pub menu_name: String,
    
    #[validate(length(max = 100, message = "Icon must not exceed 100 characters"))]
    pub icon: Option<String>,
    
    #[validate(length(min = 1, max = 100, message = "Route name must be between 1 and 100 characters"))]
    pub route_name: String,
    
    #[validate(length(min = 1, max = 200, message = "Route path must be between 1 and 200 characters"))]
    pub route_path: String,
    
    #[validate(length(max = 200, message = "Component must not exceed 200 characters"))]
    pub component: String,
    
    #[validate(range(min = 0, max = 999, message = "Icon type must be between 0 and 999"))]
    pub icon_type: Option<i32>,
    
    #[validate(length(max = 200, message = "Path parameter must not exceed 200 characters"))]
    pub path_param: Option<String>,
    
    #[validate(length(max = 200, message = "Active menu must not exceed 200 characters"))]
    pub active_menu: Option<String>,
    
    pub pid: String,
    #[validate(range(min = 0, max = 999, message = "Sequence must be between 0 and 999"))]
    pub sequence: i32,
    pub status: Status,
    pub menu_type: MenuType,
    pub hide_in_menu: Option<bool>,
    #[validate(length(max = 100, message = "I18n key must not exceed 100 characters"))]
    pub i18n_key: Option<String>,
    pub keep_alive: Option<bool>,
    pub constant: bool,
    #[validate(length(max = 200, message = "Href must not exceed 200 characters"))]
    pub href: Option<String>,
    pub multi_tab: Option<bool>,
}

// 创建菜单输入
#[allow(dead_code)]
pub type CreateMenuInput = MenuInput;

// 更新菜单输入
#[derive(Debug, Deserialize, Validate)]
#[allow(dead_code)]
pub struct UpdateMenuInput {
    pub id: i32,
    
    #[serde(flatten)]
    #[validate(nested)]
    pub menu: MenuInput,
}
