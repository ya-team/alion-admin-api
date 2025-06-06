/**
 * 响应模块
 * 
 * 该模块提供了统一的响应处理功能，用于规范化API响应格式。
 * 主要功能包括：
 * - 响应格式定义
 * - 成功响应处理
 * - 错误响应处理
 * - 响应状态码管理
 * 
 * # 主要组件
 * 
 * ## Res
 * 统一响应结构，包含以下字段：
 * - code: 响应状态码
 * - message: 响应消息
 * - data: 响应数据
 * 
 * ## ResBuilder
 * 响应构建器，用于创建响应：
 * - 设置状态码
 * - 设置消息
 * - 设置数据
 */

use std::{fmt::Debug, string::ToString};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

use crate::web::page::PaginatedData;

/**
 * 统一响应结构
 * 
 * 用于规范化API响应格式，包含状态码、消息和数据。
 * 
 * # 类型参数
 * 
 * * `T`: 响应数据的类型，必须实现Serialize trait
 */
#[derive(Debug, Serialize, Deserialize)]
pub struct Res<T: Serialize> {
    /**
     * 响应状态码
     * 
     * 用于表示响应的状态，如成功、失败等
     */
    pub code: i32,

    /**
     * 响应消息
     * 
     * 用于描述响应的详细信息
     */
    pub message: String,

    /**
     * 响应数据
     * 
     * 响应的具体数据内容
     */
    pub data: Option<T>,
}

/**
 * 响应构建器
 * 
 * 用于创建统一响应结构的构建器
 */
pub struct ResBuilder<T: Serialize> {
    /**
     * 响应状态码
     */
    code: i32,

    /**
     * 响应消息
     */
    message: String,

    /**
     * 响应数据
     */
    data: Option<T>,
}

impl<T: Serialize> ResBuilder<T> {
    /**
     * 创建响应构建器
     * 
     * # 参数
     * 
     * * `code` - 响应状态码
     * * `message` - 响应消息
     * 
     * # 返回值
     * 
     * 返回响应构建器实例
     */
    pub fn new(code: i32, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            data: None,
        }
    }

    /**
     * 设置响应数据
     * 
     * # 参数
     * 
     * * `data` - 响应数据
     * 
     * # 返回值
     * 
     * 返回响应构建器实例
     */
    pub fn data(mut self, data: T) -> Self {
        self.data = Some(data);
        self
    }

    /**
     * 构建响应
     * 
     * # 返回值
     * 
     * 返回统一响应结构
     */
    pub fn build(self) -> Res<T> {
        Res {
            code: self.code,
            message: self.message,
            data: self.data,
        }
    }
}

impl<T: Serialize> Res<T> {
    /**
     * 创建成功响应
     * 
     * # 参数
     * 
     * * `message` - 响应消息
     * 
     * # 返回值
     * 
     * 返回响应构建器实例
     */
    pub fn ok(message: impl Into<String>) -> ResBuilder<T> {
        ResBuilder::new(0, message)
    }

    /**
     * 创建错误响应
     * 
     * # 参数
     * 
     * * `code` - 错误码
     * * `message` - 错误消息
     * 
     * # 返回值
     * 
     * 返回响应构建器实例
     */
    pub fn err(code: i32, message: impl Into<String>) -> ResBuilder<T> {
        ResBuilder::new(code, message)
    }
}

impl<T: Serialize> IntoResponse for Res<T> {
    /**
     * 将响应转换为HTTP响应
     * 
     * # 返回值
     * 
     * 返回HTTP响应
     */
    fn into_response(self) -> Response {
        let status = if self.code == 0 {
            StatusCode::OK
        } else {
            StatusCode::BAD_REQUEST
        };

        let json = serde_json::to_string(&self).unwrap_or_else(|_| {
            serde_json::to_string(&Res::<()>::new_error(500, "Failed to serialize response")).unwrap()
        });

        Response::builder()
            .status(status)
            .header("content-type", "application/json")
            .body(json.into())
            .unwrap()
    }
}

#[allow(dead_code)]
impl<T: Serialize> Res<T> {
    /**
     * 创建分页数据响应
     * 
     * 创建一个包含分页数据的成功响应。
     * 使用200状态码和默认的成功消息。
     * 
     * # 参数
     * * `data` - 分页数据
     * 
     * # 返回
     * * `Res<PaginatedData<T>>` - 分页数据响应
     */
    pub fn new_paginated(data: PaginatedData<T>) -> Res<PaginatedData<T>> {
        Res {
            code: StatusCode::OK.as_u16() as i32,
            data: Some(data),
            message: "success".to_string(),
        }
    }

    /**
     * 创建成功响应
     * 
     * 创建一个包含数据和自定义消息的成功响应。
     * 使用200状态码。
     * 
     * # 参数
     * * `data` - 响应数据
     * * `msg` - 响应消息
     * 
     * # 返回
     * * `Self` - 成功响应
     */
    pub fn new_success(data: T, msg: &str) -> Self {
        Self {
            code: StatusCode::OK.as_u16() as i32,
            data: Some(data),
            message: msg.to_string(),
        }
    }

    /**
     * 创建错误响应
     * 
     * 创建一个包含错误码和消息的错误响应。
     * 
     * # 参数
     * * `code` - 错误码
     * * `msg` - 错误消息
     * 
     * # 返回
     * * `Self` - 错误响应
     */
    pub fn new_error(code: u16, msg: &str) -> Self {
        Self {
            code: code as i32,
            data: None,
            message: msg.to_string(),
        }
    }

    /**
     * 创建消息响应
     * 
     * 创建一个只包含消息的响应。
     * 使用200状态码。
     * 
     * # 参数
     * * `msg` - 响应消息
     * 
     * # 返回
     * * `Self` - 消息响应
     */
    pub fn new_message(msg: &str) -> Self {
        Self {
            code: StatusCode::OK.as_u16() as i32,
            data: None,
            message: msg.to_string(),
        }
    }

    /**
     * 创建数据响应
     * 
     * 创建一个只包含数据的响应。
     * 使用200状态码和默认的成功消息。
     * 
     * # 参数
     * * `data` - 响应数据
     * 
     * # 返回
     * * `Self` - 数据响应
     */
    pub fn new_data(data: T) -> Self {
        Self {
            code: StatusCode::OK.as_u16() as i32,
            data: Some(data),
            message: "success".to_string(),
        }
    }
}
