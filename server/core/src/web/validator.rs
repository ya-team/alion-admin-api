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

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Invalid JSON data: {0}")]
    JsonError(String),

    #[error("Invalid form data")]
    FormError,

    #[error("Validation error: {0}")]
    Validation(#[from] ValidationErrors),

    #[error("Data is missing")]
    DataMissing,
}

#[derive(Debug, Clone)]
pub struct ValidatedForm<T>(pub T);

/// 验证宏，用于快速定义必填字段的验证规则
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

/// 验证宏，用于快速定义可选字段的验证规则
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

/// 验证 trait，用于定义通用的验证方法
#[async_trait]
pub trait ValidateInput: Validate + Send + Sync {
    /// 验证输入数据
    async fn validate_input(&self) -> Result<(), ValidationError> {
        Validate::validate(self).map_err(ValidationError::from)
    }

    /// 验证并返回错误信息
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

impl<S, T> FromRequest<S> for ValidatedForm<T>
where
    T: DeserializeOwned + Validate + Send + Sync + 'static,
    S: Send + Sync + 'static,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
    Form<T>: FromRequest<S>,
{
    type Rejection = ValidationError;

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

impl IntoResponse for ValidationError {
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
