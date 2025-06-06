/**
 * 分页模块
 * 
 * 该模块提供了分页查询的功能，用于处理数据分页和排序。
 * 主要功能包括：
 * - 分页参数处理
 * - 排序参数处理
 * - 分页结果封装
 * - 分页查询构建
 * 
 * # 主要组件
 * 
 * ## PageQuery
 * 分页查询参数，包含以下字段：
 * - current: 当前页码
 * - size: 每页数量
 * - sort_by: 排序字段
 * - sort_order: 排序方向
 * 
 * ## PageResult
 * 分页查询结果，包含以下字段：
 * - total: 总记录数
 * - current: 当前页码
 * - size: 每页数量
 * - total_pages: 总页数
 * - records: 当前页数据
 */

use serde::{Deserialize, Serialize};

/**
 * 分页查询参数
 * 
 * 用于封装分页查询的参数，包括页码、每页数量、排序字段和排序方向。
 */
#[derive(Debug, Serialize, Deserialize)]
pub struct PageQuery {
    /**
     * 当前页码
     * 
     * 从1开始计数
     */
    #[serde(default = "default_page")]
    pub current: u64,

    /**
     * 每页数量
     * 
     * 默认值为10
     */
    #[serde(default = "default_page_size")]
    pub size: u64,

    /**
     * 排序字段
     * 
     * 用于指定按哪个字段排序
     */
    #[serde(default)]
    pub sort_by: Option<String>,

    /**
     * 排序方向
     * 
     * 可选值：asc（升序）或desc（降序）
     */
    #[serde(default)]
    pub sort_order: Option<String>,
}

/**
 * 分页查询结果
 * 
 * 用于封装分页查询的结果，包括总记录数、当前页码、每页数量、总页数和当前页数据。
 */
#[derive(Debug, Serialize)]
pub struct PageResult<T> {
    /**
     * 总记录数
     * 
     * 查询结果的总记录数
     */
    pub total: u64,

    /**
     * 当前页码
     * 
     * 从1开始计数
     */
    pub current: u64,

    /**
     * 每页数量
     * 
     * 每页显示的记录数
     */
    pub size: u64,

    /**
     * 总页数
     * 
     * 根据总记录数和每页数量计算得出
     */
    pub total_pages: u64,

    /**
     * 当前页数据
     * 
     * 当前页的记录数据
     */
    pub records: Vec<T>,
}

/**
 * 分页数据
 * 
 * 用于封装分页查询的结果，是 PageResult 的别名
 */
pub type PaginatedData<T> = PageResult<T>;

/**
 * 分页请求
 * 
 * 用于封装分页查询的参数，是 PageQuery 的别名
 */
pub type PageRequest = PageQuery;

/**
 * 默认页码
 * 
 * 返回默认的页码值：1
 */
fn default_page() -> u64 {
    1
}

/**
 * 默认每页数量
 * 
 * 返回默认的每页数量：10
 */
fn default_page_size() -> u64 {
    10
}

impl PageQuery {
    /**
     * 获取偏移量
     * 
     * 根据页码和每页数量计算数据偏移量
     * 
     * # 返回值
     * 
     * 返回数据偏移量
     */
    pub fn offset(&self) -> u64 {
        (self.current - 1) * self.size
    }

    /**
     * 获取排序方向
     * 
     * 根据排序方向参数返回排序方向字符串
     * 
     * # 返回值
     * 
     * 返回排序方向字符串，默认为"asc"
     */
    pub fn sort_order(&self) -> &str {
        match self.sort_order.as_deref() {
            Some("desc") => "desc",
            _ => "asc",
        }
    }
}

impl<T> PageResult<T> {
    /**
     * 创建分页结果
     * 
     * 根据总记录数、当前页码、每页数量和当前页数据创建分页结果
     * 
     * # 参数
     * 
     * * `total` - 总记录数
     * * `current` - 当前页码
     * * `size` - 每页数量
     * * `records` - 当前页数据
     * 
     * # 返回值
     * 
     * 返回分页结果
     */
    pub fn new(total: u64, current: u64, size: u64, records: Vec<T>) -> Self {
        let total_pages = Self::calculate_total_pages(total, size);
        Self {
            total,
            current,
            size,
            total_pages,
            records,
        }
    }

    pub fn calculate_total_pages(total: u64, size: u64) -> u64 {
        if size == 0 {
            0
        } else {
            (total + size - 1) / size
        }
    }

    pub fn from_parts(total: u64, current: u64, size: u64, records: Vec<T>) -> Self {
        Self::new(total, current, size, records)
    }

    pub fn builder() -> PageResultBuilder<T> {
        PageResultBuilder::new()
    }
}

#[derive(Debug)]
pub struct PageResultBuilder<T> {
    total: u64,
    current: u64,
    size: u64,
    records: Vec<T>,
}

impl<T> PageResultBuilder<T> {
    pub fn new() -> Self {
        Self {
            total: 0,
            current: 1,
            size: 10,
            records: Vec::new(),
        }
    }

    pub fn total(mut self, total: u64) -> Self {
        self.total = total;
        self
    }

    pub fn current(mut self, current: u64) -> Self {
        self.current = current;
        self
    }

    pub fn size(mut self, size: u64) -> Self {
        self.size = size;
        self
    }

    pub fn records(mut self, records: Vec<T>) -> Self {
        self.records = records;
        self
    }

    pub fn build(self) -> PageResult<T> {
        PageResult::new(self.total, self.current, self.size, self.records)
    }
}

impl Default for PageQuery {
    /**
     * 创建默认的分页查询参数
     * 
     * 使用默认的页码（1）和分页大小（10）创建分页查询参数。
     * 
     * # 返回
     * * `Self` - 默认的分页查询参数实例
     */
    fn default() -> Self {
        Self {
            current: default_page(),
            size: default_page_size(),
            sort_by: None,
            sort_order: None,
        }
    }
}

impl<T> Default for PageResult<T> {
    fn default() -> Self {
        Self {
            total: 0,
            current: 1,
            size: 10,
            total_pages: 0,
            records: Vec::new(),
        }
    }
}

#[macro_export]
macro_rules! paginated_data {
    ($total:expr, $current:expr, $size:expr, $records:expr) => {
        $crate::web::page::PaginatedData::new($total, $current, $size, $records)
    };
}
