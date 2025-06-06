/// API密钥中间件模块
/// 
/// 该模块提供了API密钥验证的中间件功能，包括：
/// - 简单API密钥验证
/// - 复杂签名验证
/// - 路由保护
/// - 请求参数解析
/// - 事件通知

use axum::{
    body::Body,
    extract::Request,
    http::{HeaderMap, StatusCode, Uri},
    middleware::Next,
    response::IntoResponse,
};
use once_cell::sync::Lazy;
use server_constant::definition::consts::SystemEvent;
use server_global::global;
use std::{collections::HashSet, sync::RwLock};

use crate::web::res::Res;

use super::{ApiKeyEvent, ComplexApiKeyValidator, SimpleApiKeyValidator};

/// 受保护路径的全局集合
///
/// 该集合存储需要API密钥验证的路径
static PROTECTED_PATHS: Lazy<RwLock<HashSet<String>>> = Lazy::new(|| RwLock::new(HashSet::new()));

/// API密钥的来源位置
///
/// 该枚举定义了API密钥可能的位置
#[derive(Clone, Copy, PartialEq)]
pub enum ApiKeySource {
    /// 从请求头中获取
    Header,
    /// 从查询参数中获取
    Query,
}

/// 简单API密钥验证配置
///
/// 该结构体包含简单API密钥验证的配置选项
#[derive(Clone)]
pub struct SimpleApiKeyConfig {
    /// API密钥的来源位置
    pub source: ApiKeySource,
    /// API密钥参数名称
    pub key_name: String,
}

impl Default for SimpleApiKeyConfig {
    /// 返回默认配置（从请求头中获取x-api-key）
    fn default() -> Self {
        Self {
            source: ApiKeySource::Header,
            key_name: "x-api-key".to_string(),
        }
    }
}

/// 复杂API密钥验证配置
///
/// 该结构体包含复杂API密钥验证的配置选项，支持签名验证
#[derive(Clone)]
pub struct ComplexApiKeyConfig {
    /// 访问密钥ID参数名称
    pub key_name: String,
    /// 时间戳参数名称
    pub timestamp_name: String,
    /// Nonce参数名称
    pub nonce_name: String,
    /// 签名参数名称
    pub signature_name: String,
}

impl Default for ComplexApiKeyConfig {
    /// 返回默认配置
    fn default() -> Self {
        Self {
            key_name: "AccessKeyId".to_string(),
            timestamp_name: "timestamp".to_string(),
            nonce_name: "nonce".to_string(),
            signature_name: "signature".to_string(),
        }
    }
}

/// API密钥验证策略
///
/// 该枚举定义了可能的API密钥验证策略
#[derive(Clone)]
pub enum ApiKeyValidation {
    /// 简单API密钥验证
    Simple(SimpleApiKeyValidator, SimpleApiKeyConfig),
    /// 复杂API密钥验证（带签名）
    Complex(ComplexApiKeyValidator, ComplexApiKeyConfig),
}

/// 添加需要API密钥验证的受保护路由
///
/// # 参数
/// * `path` - 要保护的路径
#[allow(dead_code)]
pub fn protect_route(path: &str) {
    if let Ok(mut paths) = PROTECTED_PATHS.write() {
        paths.insert(path.to_string());
    }
}

/// 检查URI路径是否需要API密钥验证
///
/// # 参数
/// * `uri` - 要检查的URI
///
/// # 返回
/// * `true` - 如果路径需要验证
/// * `false` - 如果路径不需要验证
#[inline]
fn is_protected_path(uri: &Uri) -> bool {
    if let Ok(paths) = PROTECTED_PATHS.read() {
        let path = uri.path();
        paths.contains(path.strip_suffix('/').unwrap_or(path))
    } else {
        false
    }
}

/// API密钥验证中间件
///
/// 该中间件检查请求的API密钥是否有效
///
/// # 参数
/// * `validator` - API密钥验证策略
/// * `req` - 请求对象
/// * `next` - 下一个中间件
///
/// # 返回
/// * 如果验证通过，返回下一个中间件的响应
/// * 如果验证失败，返回401 Unauthorized响应
/// * 如果请求格式错误，返回400 Bad Request响应
#[inline]
pub async fn api_key_middleware(
    validator: ApiKeyValidation,
    req: Request<Body>,
    next: Next,
) -> impl IntoResponse {
    if !is_protected_path(req.uri()) {
        return next.run(req).await.into_response();
    }

    match validate_request(&validator, &req) {
        Ok(true) => next.run(req).await.into_response(),
        Ok(false) => Res::<()>::new_error(
            StatusCode::UNAUTHORIZED.as_u16(),
            "Invalid API key or signature",
        )
        .into_response(),
        Err(e) => Res::<()>::new_error(StatusCode::BAD_REQUEST.as_u16(), e).into_response(),
    }
}

/// 从请求头中获取值
///
/// # 参数
/// * `headers` - 请求头
/// * `name` - 头名称
///
/// # 返回
/// * `Some(&str)` - 如果找到头值
/// * `None` - 如果未找到头值
#[inline]
fn get_header_value<'a>(headers: &'a HeaderMap, name: &str) -> Option<&'a str> {
    headers.get(name).and_then(|v| v.to_str().ok())
}

/// 从查询参数中获取值
///
/// # 参数
/// * `params` - 查询参数列表
/// * `name` - 参数名称
///
/// # 返回
/// * `Some(&str)` - 如果找到参数值
/// * `None` - 如果未找到参数值
#[inline]
fn get_query_value<'a>(params: &'a [(String, String)], name: &str) -> Option<&'a str> {
    params
        .iter()
        .find(|(k, _)| k == name)
        .map(|(_, v)| v.as_str())
}

/// 验证请求中的API密钥
///
/// # 参数
/// * `validator` - API密钥验证策略
/// * `req` - 请求对象
///
/// # 返回
/// * `Ok(true)` - 如果验证通过
/// * `Ok(false)` - 如果验证失败
/// * `Err(&str)` - 如果请求格式错误
#[inline]
fn validate_request(
    validator: &ApiKeyValidation,
    req: &Request<Body>,
) -> Result<bool, &'static str> {
    let headers = req.headers();
    let query = req.uri().query().unwrap_or("");
    let params = if !query.is_empty() {
        parse_query(query)
    } else {
        Vec::new()
    };

    match validator {
        ApiKeyValidation::Simple(validator, config) => {
            let api_key = match config.source {
                ApiKeySource::Header => get_header_value(headers, &config.key_name),
                ApiKeySource::Query => get_query_value(&params, &config.key_name),
            }
            .ok_or("Missing API key")?;

            global::send_dyn_event(
                SystemEvent::AuthApiKeyValidatedEvent.as_ref(),
                Box::new(ApiKeyEvent {
                    api_key: api_key.to_owned(),
                }),
            );
            Ok(validator.validate_key(api_key))
        },
        ApiKeyValidation::Complex(validator, config) => {
            let api_key =
                get_query_value(&params, &config.key_name).ok_or("Missing AccessKeyId")?;

            let timestamp = get_query_value(&params, &config.timestamp_name)
                .ok_or("Missing timestamp")?
                .parse::<i64>()
                .map_err(|_| "Invalid timestamp")?;

            let nonce = get_query_value(&params, &config.nonce_name).ok_or("Missing nonce")?;

            let signature =
                get_query_value(&params, &config.signature_name).ok_or("Missing signature")?;

            let params_for_signing: Vec<(String, String)> = params
                .iter()
                .filter(|(k, _)| k != &config.signature_name)
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect();

            global::send_dyn_event(
                SystemEvent::AuthApiKeyValidatedEvent.as_ref(),
                Box::new(ApiKeyEvent {
                    api_key: api_key.to_owned(),
                }),
            );
            Ok(validator.validate_signature(
                api_key,
                &params_for_signing,
                signature,
                timestamp,
                nonce,
            ))
        },
    }
}

/// 解析查询字符串为键值对
///
/// # 参数
/// * `query` - 查询字符串
///
/// # 返回
/// * `Vec<(String, String)>` - 解析后的键值对列表
#[inline]
fn parse_query(query: &str) -> Vec<(String, String)> {
    let capacity = query.matches('&').count() + 1;
    let mut params = Vec::with_capacity(capacity);

    for pair in query.split('&') {
        if let Some((k, v)) = pair.split_once('=') {
            if !k.is_empty() && !v.is_empty() {
                params.push((k.to_string(), v.to_string()));
            }
        }
    }

    params
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    /// 测试API密钥签名验证
    #[test]
    fn test_api_key_sign() {
        let validator = ComplexApiKeyValidator::new(None);
        validator.add_key_secret("test-access-key".to_string(), "test-secret-key".to_string());

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;
        let nonce = format!("nonce_{}", timestamp);

        let mut params = vec![
            ("AccessKeyId".to_string(), "test-access-key".to_string()),
            ("param1".to_string(), "value1".to_string()),
            ("param2".to_string(), "value2".to_string()),
            ("t".to_string(), timestamp.to_string()),
            ("n".to_string(), nonce.clone()),
        ];

        params.sort_by(|a, b| a.0.cmp(&b.0));

        let signing_string = params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&");

        let signature = validator.calculate_signature(&signing_string, "test-secret-key");

        println!(
            "URL with signature: /api/url?{}&sign={}",
            signing_string, signature
        );
    }
}
