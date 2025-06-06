use server_core::web::error::{ApiError, AppError};
use thiserror::Error;
use sea_orm::DbErr;

#[derive(Debug, Error)]
pub enum MenuError {
    #[error("Menu not found")]
    MenuNotFound,

    #[error("Duplicate route name")]
    DuplicateRouteName,

    #[error("Parent menu not found")]
    ParentMenuNotFound,

    #[error("Parent menu must be a directory")]
    ParentNotDirectory,

    #[error("Menu has children, cannot delete")]
    HasChildren,

    #[error("Menu is in use by roles, cannot delete")]
    InUse,

    #[error("Cannot move menu to its own submenu")]
    CircularReference,

    #[error("Database operation failed: {0}")]
    DatabaseOperationFailed(String),
}

impl ApiError for MenuError {
    fn code(&self) -> u16 {
        match self {
            MenuError::MenuNotFound => 5001,
            MenuError::DuplicateRouteName => 5002,
            MenuError::ParentMenuNotFound => 5003,
            MenuError::ParentNotDirectory => 5004,
            MenuError::HasChildren => 5005,
            MenuError::InUse => 5006,
            MenuError::CircularReference => 5007,
            MenuError::DatabaseOperationFailed(_) => 5008,
        }
    }

    fn message(&self) -> String {
        self.to_string()
    }
}

impl From<MenuError> for AppError {
    fn from(err: MenuError) -> Self {
        AppError {
            code: err.code(),
            message: err.message(),
        }
    }
}

impl From<DbErr> for MenuError {
    fn from(err: DbErr) -> Self {
        MenuError::DatabaseOperationFailed(err.to_string())
    }
}

impl MenuError {
    pub fn database_error(err: DbErr) -> Self {
        MenuError::DatabaseOperationFailed(err.to_string())
    }

    pub fn authentication_error(msg: String) -> Self {
        MenuError::DatabaseOperationFailed(msg)
    }

    pub fn authorization_error(msg: String) -> Self {
        MenuError::DatabaseOperationFailed(msg)
    }

    pub fn not_found_error(msg: String) -> Self {
        MenuError::DatabaseOperationFailed(msg)
    }

    pub fn validation_error(msg: String) -> Self {
        MenuError::DatabaseOperationFailed(msg)
    }

    pub fn internal_error(msg: String) -> Self {
        MenuError::DatabaseOperationFailed(msg)
    }
}
