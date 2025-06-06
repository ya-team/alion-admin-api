use std::sync::Arc;

use async_trait::async_trait;
use axum_casbin::casbin::{CoreApi, MgmtApi, RbacApi};
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set, DbErr,
};
use server_core::web::error::AppError;
use server_model::admin::entities::{
    prelude::{SysDomain, SysEndpoint, SysMenu, SysRole, SysRoleMenu, SysUser, SysUserRole},
    sys_domain::Column as SysDomainColumn,
    sys_endpoint::Column as SysEndpointColumn,
    sys_menu::Column as SysMenuColumn,
    sys_role_menu::{ActiveModel as SysRoleMenuActiveModel, Column as SysRoleMenuColumn},
    sys_user_role::{ActiveModel as SysUserRoleActiveModel, Column as SysUserRoleColumn},
};
use tokio::sync::RwLock;
use tracing::{error, info};
use regex::Regex;

use crate::helper::transaction_helper::execute_in_transaction;
use crate::admin::errors::sys_authorization_error::AuthorizationError;

/// 验证参数是否为空
fn validate_not_empty<T: AsRef<str>>(value: T, field_name: &str) -> Result<(), AppError> {
    if value.as_ref().trim().is_empty() {
        return Err(AppError {
            code: 400,
            message: format!("{} cannot be empty", field_name),
        });
    }
    Ok(())
}

/// 验证ID列表是否为空
fn validate_ids_not_empty<T>(ids: &[T], field_name: &str) -> Result<(), AppError> {
    if ids.is_empty() {
        return Err(AppError {
            code: 400,
            message: format!("{} list cannot be empty", field_name),
        });
    }
    Ok(())
}

/// 验证域代码格式
fn validate_domain_code(code: &str) -> Result<(), AppError> {
    let re = Regex::new(r"^[a-zA-Z][a-zA-Z0-9_-]*$").unwrap();
    if !re.is_match(code) {
        return Err(AppError {
            code: 400,
            message: "Domain code must start with a letter and contain only letters, numbers, underscores, and hyphens".to_string(),
        });
    }
    Ok(())
}

/// 验证角色ID格式
fn validate_role_id(id: &str) -> Result<(), AppError> {
    let re = Regex::new(r"^[a-zA-Z0-9_-]{1,64}$").unwrap();
    if !re.is_match(id) {
        return Err(AppError {
            code: 400,
            message: "Role ID must be 1-64 characters long and contain only letters, numbers, underscores, and hyphens".to_string(),
        });
    }
    Ok(())
}

/// 验证用户ID格式
fn validate_user_id(id: &str) -> Result<(), AppError> {
    let re = Regex::new(r"^[a-zA-Z0-9_-]{1,64}$").unwrap();
    if !re.is_match(id) {
        return Err(AppError {
            code: 400,
            message: "User ID must be 1-64 characters long and contain only letters, numbers, underscores, and hyphens".to_string(),
        });
    }
    Ok(())
}

/// 验证权限ID格式
fn validate_permission_id(id: &str) -> Result<(), AppError> {
    let re = Regex::new(r"^[a-zA-Z0-9_-]{1,64}$").unwrap();
    if !re.is_match(id) {
        return Err(AppError {
            code: 400,
            message: "Permission ID must be 1-64 characters long and contain only letters, numbers, underscores, and hyphens".to_string(),
        });
    }
    Ok(())
}

/// 验证路由ID格式
fn validate_route_id(id: i32) -> Result<(), AppError> {
    if id <= 0 {
        return Err(AppError {
            code: 400,
            message: "Route ID must be a positive integer".to_string(),
        });
    }
    Ok(())
}

/// 授权服务接口定义
#[async_trait]
pub trait TAuthorizationService: Send + Sync {
    /// 为角色分配权限
    /// 
    /// # Arguments
    /// * `domain_code` - 域代码
    /// * `role_id` - 角色ID
    /// * `permission_ids` - 权限ID列表
    /// * `enforcer` - 权限执行器
    /// 
    /// # Validation Rules
    /// * domain_code 不能为空，且必须符合域代码格式
    /// * role_id 不能为空，且必须符合角色ID格式
    /// * permission_ids 不能为空，且每个ID必须符合权限ID格式
    async fn assign_permissions(
        &self,
        domain_code: String,
        role_id: String,
        permission_ids: Vec<String>,
        enforcer: Arc<RwLock<impl CoreApi + MgmtApi + RbacApi + Send + Sync + 'static>>,
    ) -> Result<(), AppError>;

    /// 为角色分配路由
    /// 
    /// # Arguments
    /// * `domain_code` - 域代码
    /// * `role_id` - 角色ID
    /// * `route_ids` - 路由ID列表
    /// 
    /// # Validation Rules
    /// * domain_code 不能为空，且必须符合域代码格式
    /// * role_id 不能为空，且必须符合角色ID格式
    /// * route_ids 不能为空，且每个ID必须为正整数
    async fn assign_routes(
        &self,
        domain_code: String,
        role_id: String,
        route_ids: Vec<i32>,
    ) -> Result<(), AppError>;

    /// 为角色分配用户
    /// 
    /// # Arguments
    /// * `role_id` - 角色ID
    /// * `user_ids` - 用户ID列表
    /// 
    /// # Validation Rules
    /// * role_id 不能为空，且必须符合角色ID格式
    /// * user_ids 不能为空，且每个ID必须符合用户ID格式
    async fn assign_users(
        &self,
        role_id: String,
        user_ids: Vec<String>,
    ) -> Result<(), AppError>;
}

#[derive(Clone)]
pub struct SysAuthorizationService {
    db: Arc<DatabaseConnection>,
}

impl SysAuthorizationService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db: Arc::new(db) }
    }

    /// 获取 domain 和 role，domain 可选
    async fn get_domain_and_role(
        &self,
        domain: Option<&str>,
        role_id: &str,
    ) -> Result<(Option<server_model::admin::entities::sys_domain::Model>, server_model::admin::entities::sys_role::Model), AppError> {
        // Get role first
        let role = SysRole::find_by_id(role_id)
            .one(&*self.db)
            .await
            .map_err(DbErr::from)
            .map_err(AppError::from)?
            .ok_or_else(|| {
                let err = AuthorizationError::role_not_found(role_id.to_string(), "".to_string());
                AppError::from(err)
            })?;

        // If domain is provided, check it
        let domain = if let Some(domain) = domain {
            let domain = SysDomain::find()
                .filter(SysDomainColumn::Code.eq(domain))
                .one(&*self.db)
                .await
                .map_err(DbErr::from)
                .map_err(AppError::from)?
                .ok_or_else(|| {
                    let err = AuthorizationError::domain_not_found(domain.to_string(), "".to_string());
                    AppError::from(err)
                })?;
            Some(domain)
        } else {
            None
        };

        Ok((domain, role))
    }

    /// 同步角色权限
    async fn sync_role_permissions(
        &self,
        role_code: &str,
        domain: &str,
        new_permissions: Vec<server_model::admin::entities::sys_endpoint::Model>,
        enforcer: Arc<RwLock<impl CoreApi + MgmtApi + RbacApi + Send + Sync>>,
    ) -> Result<(), AppError> {
        let mut enforcer_write = enforcer.write().await;
        let existing_permissions =
            enforcer_write.get_filtered_policy(0, vec![role_code.to_string(), domain.to_string()]);

        println!("existing_permissions: {:?}", existing_permissions);

        let new_policies: Vec<Vec<String>> = new_permissions
            .iter()
            .map(|perm| {
                vec![
                    role_code.to_string(),
                    domain.to_string(),
                    perm.path.clone(),
                    perm.method.clone(),
                ]
            })
            .collect();

        println!("new_policies: {:?}", new_policies);

        let existing_policies: Vec<Vec<String>> = existing_permissions
            .iter()
            .map(|perm| {
                vec![
                    perm[0].clone(),
                    perm[1].clone(),
                    perm[2].clone(),
                    perm[3].clone(),
                ]
            })
            .collect();

        println!("existing_policies: {:?}", existing_policies);

        let policies_to_remove: Vec<Vec<String>> = existing_policies
            .iter()
            .filter(|policy| !new_policies.contains(policy))
            .cloned()
            .collect();

        let policies_to_add: Vec<Vec<String>> = new_policies
            .iter()
            .filter(|policy| !existing_policies.contains(policy))
            .cloned()
            .collect();

        if !policies_to_remove.is_empty() {
            let _ = enforcer_write
                .remove_policies(policies_to_remove)
                .await
                .map_err(|e| AppError {
                    code: 500,
                    message: e.to_string(),
                })?;
        }

        if !policies_to_add.is_empty() {
            let _ = enforcer_write
                .add_policies(policies_to_add)
                .await
                .map_err(|e| AppError {
                    code: 500,
                    message: e.to_string(),
                })?;
        }

        Ok(())
    }

    #[allow(dead_code)]
    async fn assign_permissions(&self, _domain: &str, role_id: &str, permission_ids: Vec<String>) -> Result<(), AuthorizationError> {
        // 检查所有权限是否存在
        let permissions = SysEndpoint::find()
            .filter(SysEndpointColumn::Id.is_in(permission_ids.clone()))
            .all(&*self.db)
            .await
            .map_err(|e| AuthorizationError::DatabaseError(Box::new(e)))?;

        if permissions.len() != permission_ids.len() {
            let found_ids: Vec<String> = permissions.iter().map(|p| p.id.to_string()).collect();
            let missing_ids: Vec<String> = permission_ids
                .into_iter()
                .filter(|id| !found_ids.contains(id))
                .collect();
            error!("Some permissions not found: {:?}", missing_ids);
            return Err(AuthorizationError::permissions_not_found(missing_ids, found_ids));
        }

        // Convert to owned String to fix lifetime issue
        let role_id = role_id.to_string();
        let permission_ids = permission_ids.clone();

        // 在事务中执行权限分配
        execute_in_transaction(&self.db, move |txn| {
            let role_id = role_id.clone();
            let permission_ids = permission_ids.clone();
            
            Box::pin(async move {
                // 获取现有权限
                let existing_permissions = SysRoleMenu::find()
                    .filter(SysRoleMenuColumn::RoleId.eq(&role_id))
                    .all(&txn)
                    .await
                    .map_err(|e| AuthorizationError::DatabaseError(Box::new(e)))?;

                // 计算需要添加和删除的权限
                let existing_ids: Vec<String> = existing_permissions
                    .iter()
                    .map(|p| p.menu_id.to_string())
                    .collect();
                let to_add: Vec<String> = permission_ids
                    .iter()
                    .filter(|id| !existing_ids.contains(id))
                    .cloned()
                    .collect();
                let to_delete: Vec<String> = existing_ids
                    .into_iter()
                    .filter(|id| !permission_ids.contains(id))
                    .collect();

                // Store lengths before using iterators
                let add_count = to_add.len();
                let delete_count = to_delete.len();

                // 批量插入新权限
                if !to_add.is_empty() {
                    let to_add_clone = to_add.clone();
                    let role_menus: Vec<SysRoleMenuActiveModel> = to_add_clone
                        .into_iter()
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
                            .map_err(|e| AuthorizationError::DatabaseError(Box::new(e)))?;
                    }
                }

                // 批量删除旧权限
                if !to_delete.is_empty() {
                    let to_delete_clone = to_delete.clone();
                    let delete_ids: Vec<i32> = to_delete_clone
                        .into_iter()
                        .filter_map(|id| id.parse::<i32>().ok())
                        .collect();

                    if !delete_ids.is_empty() {
                        SysRoleMenu::delete_many()
                            .filter(SysRoleMenuColumn::RoleId.eq(&role_id))
                            .filter(SysRoleMenuColumn::MenuId.is_in(delete_ids))
                            .exec(&txn)
                            .await
                            .map_err(|e| AuthorizationError::DatabaseError(Box::new(e)))?;
                    }
                }

                info!(
                    "Successfully assigned permissions to role: role_id={}, added={}, deleted={}",
                    role_id, add_count, delete_count
                );

                Ok(())
            })
        })
        .await
        .map_err(|e| match e {
            AppError { code, message } => AuthorizationError::DatabaseError(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Database error (code {}): {}", code, message),
            ))),
        })
    }
}

#[async_trait]
impl TAuthorizationService for SysAuthorizationService {
    async fn assign_permissions(
        &self,
        domain_code: String,
        role_id: String,
        permission_ids: Vec<String>,
        enforcer: Arc<RwLock<impl CoreApi + MgmtApi + RbacApi + Send + Sync + 'static>>,
    ) -> Result<(), AppError> {
        // 参数验证
        validate_not_empty(&domain_code, "domain_code")?;
        validate_domain_code(&domain_code)?;
        validate_not_empty(&role_id, "role_id")?;
        validate_role_id(&role_id)?;
        validate_ids_not_empty(&permission_ids, "permission_ids")?;
        for id in &permission_ids {
            validate_permission_id(id)?;
        }

        // Check domain and role first
        let (domain_opt, role) = self.get_domain_and_role(Some(&domain_code), &role_id).await?;
        let domain_code = domain_opt.as_ref().unwrap().code.clone();
        let role_code = role.code.clone();

        // Get permissions
        let permissions = SysEndpoint::find()
            .filter(SysEndpointColumn::Id.is_in(permission_ids.clone()))
            .all(&*self.db)
            .await
            .map_err(DbErr::from)
            .map_err(AppError::from)?;

        if permissions.is_empty() {
            let found_ids: Vec<String> = permissions.iter().map(|p| p.id.to_string()).collect();
            let err = AuthorizationError::permissions_not_found(vec![], found_ids);
            return Err(AppError::from(err));
        }

        // Sync permissions with enforcer
        self.sync_role_permissions(&role_code, &domain_code, permissions, enforcer)
            .await?;

        Ok(())
    }

    async fn assign_routes(
        &self,
        domain_code: String,
        role_id: String,
        route_ids: Vec<i32>,
    ) -> Result<(), AppError> {
        // 参数验证
        validate_not_empty(&domain_code, "domain_code")?;
        validate_domain_code(&domain_code)?;
        validate_not_empty(&role_id, "role_id")?;
        validate_role_id(&role_id)?;
        validate_ids_not_empty(&route_ids, "route_ids")?;
        for &id in &route_ids {
            validate_route_id(id)?;
        }

        let (domain_opt, role) = self.get_domain_and_role(Some(&domain_code), &role_id).await?;
        let domain_code = domain_opt.unwrap().code;
        let role_id = role.id;
        
        // 先批量检查所有 route_ids 是否存在
        let routes = SysMenu::find()
            .filter(SysMenuColumn::Id.is_in(route_ids.clone()))
            .all(&*self.db)
            .await
            .map_err(DbErr::from)
            .map_err(AppError::from)?;

        let found_ids: Vec<i32> = routes.iter().map(|r| r.id).collect();
        let missing_ids: Vec<i32> = route_ids.iter().filter(|id| !found_ids.contains(id)).cloned().collect();
        if !missing_ids.is_empty() {
            let err = AuthorizationError::routes_not_found(missing_ids, found_ids);
            return Err(AppError::from(err));
        }

        execute_in_transaction(&self.db, move |mut txn| {
            let domain_code = domain_code.clone();
            let role_id = role_id.clone();
            let route_ids = route_ids.clone();
            Box::pin(async move {
                let existing_routes = SysRoleMenu::find()
                    .filter(
                        SysRoleMenuColumn::RoleId
                            .eq(&role_id)
                            .and(SysRoleMenuColumn::Domain.eq(&domain_code)),
                    )
                    .all(&mut txn)
                    .await?;

                let existing_route_ids: Vec<i32> = existing_routes.iter().map(|r| r.menu_id).collect();
                let new_route_ids: Vec<i32> = route_ids
                    .iter()
                    .filter(|id| !existing_route_ids.contains(id))
                    .cloned()
                    .collect();
                let route_ids_to_delete: Vec<i32> = existing_route_ids
                    .iter()
                    .filter(|id| !route_ids.contains(id))
                    .cloned()
                    .collect();

                // 批量插入
                if !new_route_ids.is_empty() {
                    let role_menus: Vec<SysRoleMenuActiveModel> = new_route_ids
                        .iter()
                        .map(|route_id| SysRoleMenuActiveModel {
                            role_id: sea_orm::Set(role_id.clone()),
                            menu_id: sea_orm::Set(*route_id),
                            domain: sea_orm::Set(domain_code.clone()),
                            ..Default::default()
                        })
                        .collect();
                    SysRoleMenu::insert_many(role_menus)
                        .exec(&mut txn)
                        .await?;
                }

                // 批量删除
                if !route_ids_to_delete.is_empty() {
                    SysRoleMenu::delete_many()
                        .filter(
                            SysRoleMenuColumn::RoleId
                                .eq(&role_id)
                                .and(SysRoleMenuColumn::Domain.eq(&domain_code))
                                .and(SysRoleMenuColumn::MenuId.is_in(route_ids_to_delete)),
                        )
                        .exec(&mut txn)
                        .await?;
                }

                txn.commit().await?;
                Ok(())
            })
        })
        .await
    }

    async fn assign_users(
        &self,
        role_id: String,
        user_ids: Vec<String>,
    ) -> Result<(), AppError> {
        // 参数验证
        validate_not_empty(&role_id, "role_id")?;
        validate_role_id(&role_id)?;
        validate_ids_not_empty(&user_ids, "user_ids")?;
        for id in &user_ids {
            validate_user_id(id)?;
        }

        let (_, role) = self.get_domain_and_role(None, &role_id).await?;
        let role_id = role.id;

        // 先批量检查所有 user_ids 是否存在
        let users = SysUser::find()
            .filter(server_model::admin::entities::sys_user::Column::Id.is_in(user_ids.clone()))
            .all(&*self.db)
            .await
            .map_err(DbErr::from)
            .map_err(AppError::from)?;

        let found_ids: Vec<String> = users.iter().map(|u| u.id.clone()).collect();
        let missing_ids: Vec<String> = user_ids.iter().filter(|id| !found_ids.contains(id)).cloned().collect();
        if !missing_ids.is_empty() {
            let err = AuthorizationError::users_not_found(missing_ids, found_ids);
            return Err(AppError::from(err));
        }

        execute_in_transaction(&self.db, move |mut txn| {
            let role_id = role_id.clone();
            let user_ids = user_ids.clone();
            Box::pin(async move {
                let existing_user_roles = SysUserRole::find()
                    .filter(SysUserRoleColumn::RoleId.eq(&role_id))
                    .all(&mut txn)
                    .await?;

                let existing_user_ids: Vec<String> = existing_user_roles
                    .iter()
                    .map(|r| r.user_id.clone())
                    .collect();
                let new_user_ids: Vec<String> = user_ids
                    .iter()
                    .filter(|id| !existing_user_ids.contains(id))
                    .cloned()
                    .collect();
                let user_ids_to_delete: Vec<String> = existing_user_ids
                    .iter()
                    .filter(|id| !user_ids.contains(id))
                    .cloned()
                    .collect();

                // 批量插入
                if !new_user_ids.is_empty() {
                    let user_roles: Vec<SysUserRoleActiveModel> = new_user_ids
                        .iter()
                        .map(|user_id| SysUserRoleActiveModel {
                            role_id: sea_orm::Set(role_id.clone()),
                            user_id: sea_orm::Set(user_id.clone()),
                            ..Default::default()
                        })
                        .collect();
                    SysUserRole::insert_many(user_roles)
                        .exec(&mut txn)
                        .await?;
                }

                // 批量删除
                if !user_ids_to_delete.is_empty() {
                    SysUserRole::delete_many()
                        .filter(
                            SysUserRoleColumn::RoleId
                                .eq(&role_id)
                                .and(SysUserRoleColumn::UserId.is_in(user_ids_to_delete)),
                        )
                        .exec(&mut txn)
                        .await?;
                }

                txn.commit().await?;
                Ok(())
            })
        })
        .await
    }
}
