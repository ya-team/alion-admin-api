/**
 * 验证器模块
 * 
 * 该模块提供了请求数据的验证功能，用于确保数据的有效性和安全性。
 * 主要功能包括：
 * - 字段验证
 * - 自定义验证规则
 * - 验证错误处理
 * - 验证结果转换
 * 
 * # 主要组件
 * 
 * ## Validator
 * 验证器特征，定义了验证接口：
 * - 验证方法
 * - 错误处理
 * - 结果转换
 * 
 * ## ValidationError
 * 验证错误类型，用于表示验证失败：
 * - 错误消息
 * - 错误字段
 * - 错误代码
 */

use async_trait::async_trait;
use axum::{
    extract::{rejection::JsonRejection, FromRequest, Request},
    http::{header::CONTENT_TYPE, StatusCode},
    response::{IntoResponse, Response},
    Form, Json,
};
use serde::de::DeserializeOwned;
use serde_json::Value as JsonValue;
use thiserror::Error;
use validator::{Validate, ValidationErrors};
use std::future::Future;

use crate::web::res::Res;

/**
 * 验证错误类型枚举
 * 
 * 定义了验证过程中可能出现的各种错误类型：
 * - JsonError：JSON数据格式错误，包含具体的错误信息
 * - FormError：表单数据格式错误
 * - Validation：数据验证错误，包含详细的字段验证错误信息
 * - DataMissing：请求数据缺失错误
 */
#[derive(Debug, Error)]
pub enum ValidationError {
    /// JSON数据格式错误
    #[error("Invalid JSON data: {0}")]
    JsonError(String),

    /// 表单数据格式错误
    #[error("Invalid form data")]
    FormError,

    /// 数据验证错误
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationErrors),

    /// 数据缺失错误
    #[error("Data is missing")]
    DataMissing,
}

/**
 * 已验证的表单数据包装器
 * 
 * 用于存储验证通过的数据，确保数据已经通过了所有验证规则。
 * 通过FromRequest trait实现，支持从HTTP请求中提取和验证数据。
 * 
 * # 类型参数
 * 
 * * `T`: 实现了DeserializeOwned和Validate trait的类型
 */
#[derive(Debug, Clone)]
pub struct ValidatedForm<T>(pub T);

/**
 * 验证宏，用于快速定义必填字段的验证规则
 * 
 * 提供了三种模式：
 * 1. 带长度限制的字符串字段
 * 2. 必填字符串字段
 * 3. 必填自定义类型字段
 * 
 * # 参数
 * 
 * * `$field`: 字段名
 * * `$min`: 最小长度（可选）
 * * `$max`: 最大长度（可选）
 * * `$message`: 错误消息
 * * `$ty`: 字段类型（可选）
 */
#[macro_export]
macro_rules! validate_required {
    ($field:ident, $min:expr, $max:expr, $message:expr) => {
        #[validate(
            required(message = concat!($message, " is required")),
            length(
                min = $min,
                max = $max,
                message = concat!($message, " must be between ", $min, " and ", $max, " characters")
            )
        )]
        pub $field: String,
    };
    ($field:ident, $message:expr) => {
        #[validate(required(message = concat!($message, " is required")))]
        pub $field: String,
    };
    ($field:ident: $ty:ty, $message:expr) => {
        #[validate(required(message = concat!($message, " is required")))]
        pub $field: $ty,
    };
}

/**
 * 验证宏，用于快速定义可选字段的验证规则
 * 
 * 提供了两种模式：
 * 1. 带长度限制的可选字符串字段
 * 2. 可选自定义类型字段
 * 
 * # 参数
 * 
 * * `$field`: 字段名
 * * `$max`: 最大长度（可选）
 * * `$message`: 错误消息（可选）
 * * `$ty`: 字段类型（可选）
 */
#[macro_export]
macro_rules! validate_optional {
    ($field:ident, $max:expr, $message:expr) => {
        #[validate(length(max = $max, message = concat!($message, " must not exceed ", $max, " characters")))]
        pub $field: Option<String>,
    };
    ($field:ident: $ty:ty) => {
        pub $field: Option<$ty>,
    };
}

/**
 * 输入验证 trait，用于定义通用的验证方法
 * 
 * 为实现了Validate trait的类型提供额外的验证功能：
 * - validate_input：验证输入数据并返回Result
 * - validate_with_errors：验证并返回详细的错误信息
 * 
 * # 类型约束
 * 
 * 实现此trait的类型必须：
 * - 实现Validate trait
 * - 实现Send + Sync trait
 */
#[async_trait]
pub trait ValidateInput: Validate + Send + Sync {
    /**
     * 验证输入数据
     * 
     * 使用Validate trait的validate方法验证数据，
     * 并将验证错误转换为ValidationError。
     * 
     * # 返回
     * * `Result<(), ValidationError>` - 验证成功返回Ok(())，失败返回错误
     */
    async fn validate_input(&self) -> Result<(), ValidationError> {
        Validate::validate(self).map_err(ValidationError::from)
    }

    /**
     * 验证并返回错误信息
     * 
     * 验证数据并返回详细的错误信息列表。
     * 错误信息包含每个字段的具体验证错误。
     * 
     * # 返回
     * * `Result<(), Vec<String>>` - 验证成功返回Ok(())，失败返回错误信息列表
     */
    async fn validate_with_errors(&self) -> Result<(), Vec<String>> {
        match Validate::validate(self) {
            Ok(_) => Ok(()),
            Err(errors) => {
                let error_messages: Vec<String> = errors
                    .field_errors()
                    .into_iter()
                    .flat_map(|(_, errors)| {
                        errors.iter().map(|error| {
                            error
                                .message
                                .as_ref()
                                .map(|cow| cow.to_string())
                                .unwrap_or_else(|| "Unknown error".to_string())
                        })
                    })
                    .collect();
                Err(error_messages)
            }
        }
    }
}

// 为所有实现了 Validate 的类型自动实现 ValidateInput
#[async_trait]
impl<T: Validate + Send + Sync> ValidateInput for T {}

/**
 * 实现从请求中提取并验证表单数据的功能
 * 
 * 支持从HTTP请求中提取JSON或表单数据，并进行验证。
 * 根据Content-Type头自动选择数据提取方式。
 * 
 * # 类型参数
 * 
 * * `S`: 应用状态类型
 * * `T`: 实现了DeserializeOwned和Validate trait的类型
 */
impl<S, T> FromRequest<S> for ValidatedForm<T>
where
    T: DeserializeOwned + Validate + Send + Sync + 'static,
    S: Send + Sync + 'static,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
    Form<T>: FromRequest<S>,
{
    type Rejection = ValidationError;

    /**
     * 从请求中提取并验证数据
     * 
     * 根据Content-Type头选择数据提取方式：
     * - application/json：提取JSON数据
     * - application/x-www-form-urlencoded：提取表单数据
     * 
     * # 参数
     * * `req` - HTTP请求
     * * `state` - 应用状态
     * 
     * # 返回
     * * `Result<Self, Self::Rejection>` - 成功返回验证后的数据，失败返回错误
     */
    fn from_request(
        req: Request,
        state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            let content_type = req
                .headers()
                .get(CONTENT_TYPE)
                .and_then(|value| value.to_str().ok());

            let data = match content_type.as_deref() {
                Some(ct) if ct.contains(mime::APPLICATION_JSON.as_ref()) => {
                    let Json(data) = Json::<T>::from_request(req, state)
                        .await
                        .map_err(|e| ValidationError::JsonError(e.to_string()))?;
                    data
                },
                Some(ct) if ct.contains(mime::APPLICATION_WWW_FORM_URLENCODED.as_ref()) => {
                    let Form(data) = Form::<T>::from_request(req, state)
                        .await
                        .map_err(|_| ValidationError::FormError)?;
                    data
                },
                _ => return Err(ValidationError::DataMissing),
            };

            Validate::validate(&data).map_err(ValidationError::from)?;
            Ok(ValidatedForm(data))
        }
    }
}

/**
 * 实现验证错误的响应转换
 * 
 * 将ValidationError转换为标准化的HTTP响应。
 * 根据错误类型生成不同的错误消息和状态码。
 */
impl IntoResponse for ValidationError {
    /**
     * 将验证错误转换为HTTP响应
     * 
     * 根据错误类型生成不同的响应：
     * - JsonError：返回400状态码和JSON错误信息
     * - FormError：返回400状态码和表单错误信息
     * - Validation：返回400状态码和详细的字段验证错误
     * - DataMissing：返回400状态码和数据缺失错误信息
     * 
     * # 返回
     * * `Response` - HTTP响应
     */
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ValidationError::JsonError(msg) => (StatusCode::BAD_REQUEST, msg),
            ValidationError::FormError => {
                (StatusCode::BAD_REQUEST, "Invalid form data".to_string())
            },
            ValidationError::Validation(errors) => {
                let error_messages: serde_json::Map<String, JsonValue> = errors
                    .field_errors()
                    .into_iter()
                    .map(|(field, errors)| {
                        let messages: Vec<String> = errors
                            .iter()
                            .map(|error| {
                                error
                                    .message
                                    .as_ref()
                                    .map(|cow| cow.to_string())
                                    .unwrap_or_else(|| "Unknown error".to_string())
                            })
                            .collect();
                        (
                            field.to_string(),
                            JsonValue::Array(messages.into_iter().map(JsonValue::String).collect()),
                        )
                    })
                    .collect();
                (
                    StatusCode::BAD_REQUEST,
                    serde_json::to_string(
                        &serde_json::json!({ "validation_errors": error_messages }),
                    )
                    .unwrap(),
                )
            },
            ValidationError::DataMissing => {
                (StatusCode::BAD_REQUEST, "Data is missing".to_string())
            },
        };

        Res::<String>::new_error(status.as_u16(), &error_message).into_response()
    }
}
