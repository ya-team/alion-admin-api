//! 系统域服务模块
//! 
//! 该模块提供了域管理相关的核心功能，包括：
//! - 域CRUD操作
//! - 域分页查询
//! - 域代码和名称唯一性检查
//! 
//! # 主要组件
//! 
//! ## 核心接口
//! * `TDomainService`: 域服务 trait，定义了域管理相关的核心接口
//! * `SysDomainService`: 域服务实现，提供了具体的域管理逻辑
//! 
//! ## 功能特性
//! * 域查询：支持分页查询和关键字搜索
//! * 域创建：支持创建新域，包括代码和名称唯一性检查
//! * 域更新：支持更新域信息，包括代码和名称唯一性检查
//! * 域删除：支持删除域，内置域不可删除
//! 
//! # 使用示例
//! 
//! use server_service::admin::sys_domain_service::*;
//! 
//! // 创建域服务实例
//! let domain_service = SysDomainService;
//! 
//! // 创建新域
//! let domain = domain_service.create_domain(CreateDomainInput {
//!     code: "example".to_string(),
//!     name: "示例域".to_string(),
//!     description: Some("这是一个示例域".to_string()),
//! }).await?;
//! 
//! // 分页查询域
//! let domains = domain_service.find_paginated_domains(DomainPageRequest {
//!     keywords: Some("示例".to_string()),
//!     page_details: PageDetails {
//!         current: 1,
//!         size: 10,
//!     },
//! }).await?;
//! 

use async_trait::async_trait;
use chrono::Local;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, Set,
};
use server_core::web::{error::AppError, page::PaginatedData};
use server_model::admin::{
    entities::{
        prelude::SysDomain,
        sea_orm_active_enums::Status,
        sys_domain::{
            ActiveModel as SysDomainActiveModel, Column as SysDomainColumn, Model as SysDomainModel,
        },
    },
    input::{CreateDomainInput, DomainPageRequest, UpdateDomainInput},
};
use ulid::Ulid;

use crate::{admin::sys_domain_error::DomainError, helper::db_helper};

/// 域服务 trait
/// 
/// 定义了域管理相关的核心接口，包括：
/// - 域查询（分页）
/// - 域创建
/// - 域更新
/// - 域删除
/// 
/// # 使用示例
/// 
/// use server_service::admin::sys_domain_service::*;
/// 
/// let domain_service = SysDomainService;
/// 
/// // 分页查询域
/// let domains = domain_service.find_paginated_domains(DomainPageRequest {
///     keywords: Some("示例".to_string()),
///     page_details: PageDetails {
///         current: 1,
///         size: 10,
///     },
/// }).await?;
/// 
#[async_trait]
pub trait TDomainService {
    /// 分页查询域
    /// 
    /// 根据查询条件分页获取域列表
    /// 
    /// # 参数
    /// * `params` - 分页查询参数，包含关键字和分页信息
    /// 
    /// # 返回
    /// * `Result<PaginatedData<SysDomainModel>, AppError>` - 分页域数据或错误
    async fn find_paginated_domains(
        &self,
        params: DomainPageRequest,
    ) -> Result<PaginatedData<SysDomainModel>, AppError>;

    /// 创建域
    /// 
    /// 创建新域，包括代码和名称唯一性检查
    /// 
    /// # 参数
    /// * `input` - 域创建参数
    /// 
    /// # 返回
    /// * `Result<SysDomainModel, AppError>` - 创建的域信息或错误
    async fn create_domain(&self, input: CreateDomainInput) -> Result<SysDomainModel, AppError>;

    /// 获取域
    /// 
    /// 根据域ID获取域信息
    /// 
    /// # 参数
    /// * `id` - 域ID
    /// 
    /// # 返回
    /// * `Result<SysDomainModel, AppError>` - 域信息或错误
    async fn get_domain(&self, id: &str) -> Result<SysDomainModel, AppError>;

    /// 更新域
    /// 
    /// 更新域信息，包括代码和名称唯一性检查
    /// 
    /// # 参数
    /// * `input` - 域更新参数
    /// 
    /// # 返回
    /// * `Result<SysDomainModel, AppError>` - 更新后的域信息或错误
    async fn update_domain(&self, input: UpdateDomainInput) -> Result<SysDomainModel, AppError>;

    /// 删除域
    /// 
    /// 根据域ID删除域，内置域不可删除
    /// 
    /// # 参数
    /// * `id` - 域ID
    /// 
    /// # 返回
    /// * `Result<(), AppError>` - 删除结果
    async fn delete_domain(&self, id: &str) -> Result<(), AppError>;
}

/// 系统域服务
/// 
/// 实现了域管理相关的核心功能，包括：
/// - 域CRUD操作
/// - 域分页查询
/// - 域代码和名称唯一性检查
/// 
/// # 使用示例
/// 
/// use server_service::admin::sys_domain_service::*;
/// 
/// let domain_service = SysDomainService;
/// 
/// // 创建域
/// let domain = domain_service.create_domain(CreateDomainInput {
///     code: "example".to_string(),
///     name: "示例域".to_string(),
///     description: Some("这是一个示例域".to_string()),
/// }).await?;
/// 
#[derive(Clone)]
pub struct SysDomainService;

impl SysDomainService {
    /// 检查域代码和名称唯一性
    /// 
    /// 检查域代码和名称是否已存在，支持排除当前域
    /// 
    /// # 参数
    /// * `id` - 当前域ID（可选）
    /// * `code` - 域代码
    /// * `name` - 域名称
    /// 
    /// # 返回
    /// * `Result<(), AppError>` - 检查结果
    /// 
    /// # 错误
    /// * `DuplicateCode` - 域代码已存在
    /// * `DuplicateName` - 域名称已存在
    async fn check_domain_exists(
        &self,
        id: Option<&str>,
        code: &str,
        name: &str,
    ) -> Result<(), AppError> {
        let id_str = id.unwrap_or("-1");
        let db = db_helper::get_db_connection().await?;

        let code_exists = SysDomain::find()
            .filter(SysDomainColumn::Code.eq(code))
            .filter(SysDomainColumn::Id.ne(id_str))
            .one(db.as_ref())
            .await
            .map_err(AppError::from)?
            .is_some();

        if code_exists {
            return Err(DomainError::DuplicateCode.into());
        }

        let name_exists = SysDomain::find()
            .filter(SysDomainColumn::Name.eq(name))
            .filter(SysDomainColumn::Id.ne(id_str))
            .one(db.as_ref())
            .await
            .map_err(AppError::from)?
            .is_some();

        if name_exists {
            return Err(DomainError::DuplicateName.into());
        }

        Ok(())
    }
}

#[async_trait]
impl TDomainService for SysDomainService {
    async fn find_paginated_domains(
        &self,
        params: DomainPageRequest,
    ) -> Result<PaginatedData<SysDomainModel>, AppError> {
        let db = db_helper::get_db_connection().await?;
        let mut query = SysDomain::find();

        if let Some(ref keywords) = params.keywords {
            let condition = Condition::any().add(SysDomainColumn::Name.contains(keywords));
            query = query.filter(condition);
        }

        let total = query
            .clone()
            .count(db.as_ref())
            .await
            .map_err(AppError::from)?;

        let paginator = query.paginate(db.as_ref(), params.page_details.size);
        let records = paginator
            .fetch_page(params.page_details.current - 1)
            .await
            .map_err(AppError::from)?;

        Ok(PaginatedData {
            current: params.page_details.current,
            size: params.page_details.size,
            total,
            records,
        })
    }

    async fn create_domain(&self, input: CreateDomainInput) -> Result<SysDomainModel, AppError> {
        self.check_domain_exists(None, &input.code, &input.name)
            .await?;

        let db = db_helper::get_db_connection().await?;

        let domain = SysDomainActiveModel {
            id: Set(Ulid::new().to_string()),
            code: Set(input.code),
            name: Set(input.name),
            description: Set(input.description),
            status: Set(Status::Enabled),
            created_at: Set(Local::now().naive_local()),
            created_by: Set("TODO".to_string()),
            ..Default::default()
        };

        let result = domain.insert(db.as_ref()).await.map_err(AppError::from)?;
        Ok(result)
    }

    async fn get_domain(&self, id: &str) -> Result<SysDomainModel, AppError> {
        let db = db_helper::get_db_connection().await?;
        SysDomain::find_by_id(id)
            .one(db.as_ref())
            .await
            .map_err(AppError::from)?
            .ok_or_else(|| DomainError::DomainNotFound.into())
    }

    async fn update_domain(&self, input: UpdateDomainInput) -> Result<SysDomainModel, AppError> {
        let db = db_helper::get_db_connection().await?;
        let existing_domain = self.get_domain(&input.id).await?;

        if existing_domain.code == "built-in" {
            return Err(DomainError::BuiltInDomain.into());
        }

        self.check_domain_exists(Some(&input.id), &input.domain.code, &input.domain.name)
            .await?;

        let mut domain: SysDomainActiveModel = existing_domain.into();
        domain.code = Set(input.domain.code);
        domain.name = Set(input.domain.name);
        domain.description = Set(input.domain.description);

        let updated_domain = domain.update(db.as_ref()).await.map_err(AppError::from)?;
        Ok(updated_domain)
    }

    async fn delete_domain(&self, id: &str) -> Result<(), AppError> {
        let domain = self.get_domain(id).await?;

        if domain.code == "built-in" {
            return Err(DomainError::BuiltInDomain.into());
        }

        let db = db_helper::get_db_connection().await?;
        SysDomain::delete_by_id(id)
            .exec(db.as_ref())
            .await
            .map_err(AppError::from)?;
        Ok(())
    }
}
