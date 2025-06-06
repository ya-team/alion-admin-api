use async_trait::async_trait;
use chrono::Local;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, IntoActiveModel, PaginatorTrait, QueryFilter, Set,
};
use server_core::web::page::PaginatedData;
use server_model::admin::{
    entities::{
        prelude::{SysRole, SysRoleMenu},
        sys_role::{
            ActiveModel as SysRoleActiveModel, Column as SysRoleColumn, Model as SysRoleModel,
        },
        sys_role_menu::Column as SysRoleMenuColumn,
    },
    input::{CreateRoleInput, RolePageRequest, UpdateRoleInput},
};
use ulid::Ulid;

use crate::{
    admin::errors::sys_role_error::RoleError,
    helper::db_helper,
};

#[async_trait]
pub trait TRoleService {
    async fn find_paginated_roles(
        &self,
        params: RolePageRequest,
    ) -> Result<PaginatedData<SysRoleModel>, RoleError>;

    async fn create_role(&self, input: CreateRoleInput) -> Result<SysRoleModel, RoleError>;
    async fn get_role(&self, id: &str) -> Result<SysRoleModel, RoleError>;
    async fn update_role(&self, input: UpdateRoleInput) -> Result<SysRoleModel, RoleError>;
    async fn delete_role(&self, id: &str) -> Result<(), RoleError>;
}

#[derive(Clone)]
pub struct SysRoleService;

impl SysRoleService {
    async fn check_role_exists(&self, id: Option<&str>, code: &str) -> Result<(), RoleError> {
        let db = db_helper::get_db_connection().await?;
        let mut query = SysRole::find().filter(SysRoleColumn::Code.eq(code));

        if let Some(id) = id {
            query = query.filter(SysRoleColumn::Id.ne(id));
        }

        let existing_role = query.one(db.as_ref()).await?;

        if existing_role.is_some() {
            return Err(RoleError::DuplicateRoleCode);
        }

        Ok(())
    }
}

#[async_trait]
impl TRoleService for SysRoleService {
    async fn find_paginated_roles(
        &self,
        params: RolePageRequest,
    ) -> Result<PaginatedData<SysRoleModel>, RoleError> {
        let db = db_helper::get_db_connection().await?;
        let mut query = SysRole::find();

        if let Some(ref keywords) = params.keywords {
            let condition = Condition::any().add(SysRoleColumn::Code.contains(keywords));
            query = query.filter(condition);
        }

        let total = query
            .clone()
            .count(db.as_ref())
            .await?;

        let paginator = query.paginate(db.as_ref(), params.page_details.size);
        let records = paginator
            .fetch_page(params.page_details.current - 1)
            .await?;

        Ok(PaginatedData {
            current: params.page_details.current,
            size: params.page_details.size,
            total,
            records,
        })
    }

    async fn create_role(&self, input: CreateRoleInput) -> Result<SysRoleModel, RoleError> {
        self.check_role_exists(None, &input.code).await?;

        let db = db_helper::get_db_connection().await?;
        let role = SysRoleActiveModel {
            id: Set(Ulid::new().to_string()),
            code: Set(input.code),
            name: Set(input.name),
            description: Set(input.description),
            pid: Set(input.pid),
            status: Set(input.status),
            created_at: Set(Local::now().naive_local()),
            created_by: Set("system".to_string()),
            ..Default::default()
        };

        let role_model = role.insert(db.as_ref()).await?;
        Ok(role_model)
    }

    async fn get_role(&self, id: &str) -> Result<SysRoleModel, RoleError> {
        let db = db_helper::get_db_connection().await?;
        SysRole::find_by_id(id)
            .one(db.as_ref())
            .await?
            .ok_or(RoleError::RoleNotFound)
    }

    async fn update_role(&self, input: UpdateRoleInput) -> Result<SysRoleModel, RoleError> {
        let mut role = self.get_role(&input.id).await?.into_active_model();

        if input.role.code != *role.code.as_ref() {
            self.check_role_exists(Some(&input.id), &input.role.code).await?;
        }

        role.code = Set(input.role.code);
        role.name = Set(input.role.name);
        role.description = Set(input.role.description);
        role.pid = Set(input.role.pid);
        role.status = Set(input.role.status);

        let db = db_helper::get_db_connection().await?;
        let updated_role = role.update(db.as_ref()).await?;
        Ok(updated_role)
    }

    async fn delete_role(&self, id: &str) -> Result<(), RoleError> {
        let db = db_helper::get_db_connection().await?;

        let _role = self.get_role(id).await?;

        let has_children = SysRole::find()
            .filter(SysRoleColumn::Pid.eq(id))
            .one(db.as_ref())
            .await?
            .is_some();

        if has_children {
            return Err(RoleError::HasChildren);
        }

        let in_use = SysRoleMenu::find()
            .filter(SysRoleMenuColumn::RoleId.eq(id))
            .one(db.as_ref())
            .await?
            .is_some();

        if in_use {
            return Err(RoleError::InUse);
        }

        let result = SysRole::delete_by_id(id)
            .exec(db.as_ref())
            .await?;

        if result.rows_affected == 0 {
            return Err(RoleError::RoleNotFound);
        }

        Ok(())
    }
}
