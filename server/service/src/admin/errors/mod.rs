pub mod base_error;
pub mod sys_auth_error;
pub mod sys_user_error;
pub mod sys_role_error;
pub mod sys_menu_error;
pub mod sys_domain_error;
pub mod sys_endpoint_error;
pub mod sys_operation_log_error;
pub mod sys_login_log_error;
pub mod sys_access_key_error;
pub mod sys_authorization_error;

// Re-export base types and macros
pub use base_error::{CommonError, ServiceError};
pub use crate::{impl_from_common_error, impl_from_db_error};

// Re-export all error types
pub use sys_auth_error::AuthError;
pub use sys_user_error::UserError;
pub use sys_role_error::RoleError;
pub use sys_domain_error::DomainError;
pub use sys_access_key_error::AccessKeyError;
pub use sys_authorization_error::AuthorizationError;
