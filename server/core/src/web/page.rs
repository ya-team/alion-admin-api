/// 分页处理模块
/// 
/// 该模块提供了分页请求和响应的处理功能，包括：
/// - 分页请求参数的处理：支持多种格式的分页参数解析
/// - 分页响应的格式化：统一的分页数据响应格式
/// - 默认分页参数的处理：提供合理的默认值
/// 
/// # 主要组件
/// 
/// ## PageRequest
/// 分页请求参数结构：
/// - 当前页码（current）
/// - 每页大小（size）
/// 
/// ## PaginatedData
/// 分页数据响应结构：
/// - 当前页码
/// - 每页大小
/// - 总记录数
/// - 当前页数据
/// 
/// # 使用示例
/// 
/// 
/// // 创建分页请求
/// let page_request = PageRequest {
///     current: 1,
///     size: 10,
/// };
/// 
/// // 创建分页响应
/// let response = PaginatedData {
///     current: 1,
///     size: 10,
///     total: 100,
///     records: vec![/* 数据记录 */],
/// };
/// 

use serde::{de::Error as DeError, Deserialize, Deserializer, Serialize};

/// 默认分页大小
/// 
/// 当未指定分页大小时使用的默认值
const DEFAULT_PAGE_SIZE: u64 = 10;

/// 默认页码
/// 
/// 当未指定页码时使用的默认值
const DEFAULT_PAGE_NUM: u64 = 1;

/// 分页请求结构体
/// 
/// 用于接收和处理分页请求参数。
/// 支持从查询参数、JSON请求体等多种来源解析分页参数。
/// 
/// # 字段
/// 
/// * `current`: 当前页码，从1开始
/// * `size`: 每页显示的记录数
/// 
/// # 序列化/反序列化
/// 
/// 支持以下格式的输入：
/// - 数字：`{"current": 1, "size": 10}`
/// - 字符串：`{"current": "1", "size": "10"}`
/// - 空值：使用默认值
#[derive(Debug, Serialize, Deserialize)]
pub struct PageRequest {
    /// 当前页码
    /// 
    /// 从1开始计数，默认为1
    #[serde(
        default = "default_current",
        deserialize_with = "deserialize_optional_u64"
    )]
    pub current: u64,
    /// 每页大小
    /// 
    /// 每页显示的记录数，默认为10
    #[serde(
        default = "default_size",
        deserialize_with = "deserialize_optional_u64"
    )]
    pub size: u64,
}

/// 反序列化可选的u64值
/// 
/// 支持以下格式：
/// - 数字：直接解析为u64
/// - 字符串形式的数字：解析字符串为u64
/// - 空字符串：使用默认值
/// - 空值：使用默认值
/// 
/// # 参数
/// * `deserializer` - 反序列化器
/// 
/// # 返回
/// * `Result<u64, D::Error>` - 成功返回解析后的u64值，失败返回错误
/// 
/// # 错误
/// * 当字符串无法解析为数字时返回错误
fn deserialize_optional_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrU64 {
        String(String),
        U64(u64),
    }

    match Option::<StringOrU64>::deserialize(deserializer)? {
        None => Ok(DEFAULT_PAGE_NUM),
        Some(StringOrU64::U64(n)) => Ok(n),
        Some(StringOrU64::String(s)) if s.is_empty() => Ok(DEFAULT_PAGE_NUM),
        Some(StringOrU64::String(s)) => s.parse::<u64>().map_err(DeError::custom),
    }
}

/// 获取默认页码
/// 
/// # 返回
/// * `u64` - 默认页码值（1）
fn default_current() -> u64 {
    DEFAULT_PAGE_NUM
}

/// 获取默认分页大小
/// 
/// # 返回
/// * `u64` - 默认分页大小（10）
fn default_size() -> u64 {
    DEFAULT_PAGE_SIZE
}

impl Default for PageRequest {
    /// 创建默认的分页请求
    /// 
    /// 使用默认的页码（1）和分页大小（10）创建分页请求。
    /// 
    /// # 返回
    /// * `Self` - 默认的分页请求实例
    fn default() -> Self {
        Self {
            current: default_current(),
            size: default_size(),
        }
    }
}

/// 分页数据响应结构体
/// 
/// 用于返回分页查询的结果。
/// 包含分页信息和当前页的数据记录。
/// 
/// # 类型参数
/// 
/// * `T`: 数据记录的类型，必须实现Serialize trait
/// 
/// # 字段
/// 
/// * `current`: 当前页码
/// * `size`: 每页大小
/// * `total`: 总记录数
/// * `records`: 当前页的数据记录
#[derive(Debug, Serialize, Default)]
pub struct PaginatedData<T> {
    /// 当前页码
    pub current: u64,
    /// 每页大小
    pub size: u64,
    /// 总记录数
    pub total: u64,
    /// 当前页的数据记录
    pub records: Vec<T>,
}
