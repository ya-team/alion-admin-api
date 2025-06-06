/// 响应处理模块
/// 
/// 该模块提供了统一的API响应格式处理功能，包括：
/// - 成功响应：返回成功状态和数据
/// - 错误响应：返回错误状态和消息
/// - 分页数据响应：返回分页查询结果
/// - 消息响应：返回操作结果消息
/// 
/// # 主要组件
/// 
/// ## Res
/// 统一响应结构，包含：
/// - 状态码：HTTP状态码
/// - 数据：可选的响应数据
/// - 消息：响应消息
/// - 成功标志：表示请求是否成功
/// 
/// # 使用示例
/// 
/// 
/// // 创建成功响应
/// let success_res = Res::new_success(data, "操作成功");
/// 
/// // 创建错误响应
/// let error_res = Res::new_error(400, "参数错误");
/// 
/// // 创建分页响应
/// let page_res = Res::new_paginated(paginated_data);
/// 
/// // 创建消息响应
/// let msg_res = Res::new_message("操作完成");
/// 

use std::{fmt::Debug, string::ToString};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

use crate::web::page::PaginatedData;

/// 统一响应结构体
/// 
/// 用于统一API响应格式，支持多种类型的响应数据。
/// 实现了序列化和调试功能，可以直接转换为JSON响应。
/// 
/// # 类型参数
/// 
/// * `T`: 响应数据的类型，必须实现Serialize trait
/// 
/// # 字段
/// 
/// * `code`: HTTP状态码
/// * `data`: 可选的响应数据
/// * `msg`: 响应消息
/// * `success`: 请求是否成功
#[derive(Debug, Serialize, Default)]
pub struct Res<T> {
    /// HTTP状态码
    pub code: u16,
    /// 响应数据
    pub data: Option<T>,
    /// 响应消息
    pub msg: String,
    /// 是否成功
    pub success: bool,
}

#[allow(dead_code)]
impl<T: Serialize> Res<T> {
    /// 创建分页数据响应
    /// 
    /// 创建一个包含分页数据的成功响应。
    /// 使用200状态码和默认的成功消息。
    /// 
    /// # 参数
    /// * `data` - 分页数据
    /// 
    /// # 返回
    /// * `Res<PaginatedData<T>>` - 分页数据响应
    pub fn new_paginated(data: PaginatedData<T>) -> Res<PaginatedData<T>> {
        Res {
            code: StatusCode::OK.as_u16(),
            data: Some(data),
            msg: "success".to_string(),
            success: true,
        }
    }

    /// 创建成功响应
    /// 
    /// 创建一个包含数据和自定义消息的成功响应。
    /// 使用200状态码。
    /// 
    /// # 参数
    /// * `data` - 响应数据
    /// * `msg` - 响应消息
    /// 
    /// # 返回
    /// * `Self` - 成功响应
    pub fn new_success(data: T, msg: &str) -> Self {
        Self {
            code: StatusCode::OK.as_u16(),
            data: Some(data),
            msg: msg.to_string(),
            success: true,
        }
    }

    /// 创建错误响应
    /// 
    /// 创建一个包含错误状态码和消息的错误响应。
    /// 不包含数据，success字段为false。
    /// 
    /// # 参数
    /// * `code` - HTTP状态码
    /// * `msg` - 错误消息
    /// 
    /// # 返回
    /// * `Self` - 错误响应
    pub fn new_error(code: u16, msg: &str) -> Self {
        Self {
            code,
            data: None,
            msg: msg.to_string(),
            success: false,
        }
    }

    /// 创建消息响应
    /// 
    /// 创建一个只包含消息的成功响应。
    /// 使用200状态码，不包含数据。
    /// 
    /// # 参数
    /// * `msg` - 响应消息
    /// 
    /// # 返回
    /// * `Self` - 消息响应
    pub fn new_message(msg: &str) -> Self {
        Self {
            code: StatusCode::OK.as_u16(),
            data: None,
            msg: msg.to_string(),
            success: true,
        }
    }

    /// 创建数据响应
    /// 
    /// 创建一个只包含数据的成功响应。
    /// 使用200状态码和默认的成功消息。
    /// 
    /// # 参数
    /// * `data` - 响应数据
    /// 
    /// # 返回
    /// * `Self` - 数据响应
    pub fn new_data(data: T) -> Self {
        Self {
            code: StatusCode::OK.as_u16(),
            data: Some(data),
            msg: "success".to_string(),
            success: true,
        }
    }
}

impl<T> IntoResponse for Res<T>
where
    T: Serialize + Send + Sync + Debug + 'static,
{
    /// 将响应转换为HTTP响应
    /// 
    /// 将Res结构体转换为JSON格式的HTTP响应。
    /// 
    /// # 返回
    /// * `Response` - HTTP响应
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
