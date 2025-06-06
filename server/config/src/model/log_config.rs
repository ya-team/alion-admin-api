/**
 * 日志配置模块
 * 
 * 定义了应用程序的日志记录参数
 */

use serde::Deserialize;

/**
 * 日志配置结构体
 * 
 * 包含日志记录所需的所有参数，包括：
 * - 日志级别
 * - 输出目标
 * - 格式化选项
 */
#[derive(Deserialize, Debug, Clone)]
pub struct LogConfig {
    /**
     * 日志级别
     * 
     * 控制日志记录的详细程度
     * 可选值：trace, debug, info, warn, error
     */
    pub level: String,

    /**
     * 是否启用控制台输出
     * 
     * 控制是否将日志输出到控制台
     */
    pub enable_console: bool,

    /**
     * 是否启用文件输出
     * 
     * 控制是否将日志写入文件
     */
    pub enable_file: bool,

    /**
     * 日志文件路径
     * 
     * 日志文件的存储路径
     * 仅在启用文件输出时有效
     */
    pub file_path: Option<String>,

    /**
     * 是否启用异步日志
     * 
     * 控制是否使用异步方式记录日志
     * 可以提高性能，但可能丢失部分日志
     */
    pub enable_async: bool,

    /**
     * 异步缓冲区大小
     * 
     * 异步日志的缓冲区大小
     * 仅在启用异步日志时有效
     */
    pub async_buffer_size: Option<usize>,

    /**
     * 是否启用结构化日志
     * 
     * 控制是否使用JSON格式记录日志
     * 便于日志分析和处理
     */
    pub enable_structured: bool,

    /**
     * 是否启用彩色输出
     * 
     * 控制是否在控制台使用彩色输出
     * 仅在启用控制台输出时有效
     */
    pub enable_colors: bool,

    /**
     * 是否启用时间戳
     * 
     * 控制是否在日志中包含时间戳
     */
    pub enable_timestamp: bool,

    /**
     * 时间戳格式
     * 
     * 日志中时间戳的格式
     * 仅在启用时间戳时有效
     */
    pub timestamp_format: Option<String>,

    /**
     * 是否启用线程ID
     * 
     * 控制是否在日志中包含线程ID
     */
    pub enable_thread_id: bool,

    /**
     * 是否启用模块路径
     * 
     * 控制是否在日志中包含模块路径
     */
    pub enable_module_path: bool,

    /**
     * 是否启用行号
     * 
     * 控制是否在日志中包含行号
     */
    pub enable_line_number: bool,

    /**
     * 是否启用文件轮转
     * 
     * 控制是否启用日志文件轮转
     * 仅在启用文件输出时有效
     */
    pub enable_rotation: bool,

    /**
     * 轮转配置
     * 
     * 日志文件轮转的具体参数
     * 仅在启用文件轮转时有效
     */
    pub rotation: Option<LogRotationConfig>,
}

/**
 * 日志轮转配置结构体
 * 
 * 定义了日志文件轮转的具体参数
 */
#[derive(Deserialize, Debug, Clone)]
pub struct LogRotationConfig {
    /**
     * 最大文件大小（字节）
     * 
     * 单个日志文件的最大大小
     * 超过此大小将触发轮转
     */
    pub max_size: u64,

    /**
     * 最大文件数量
     * 
     * 保留的最大日志文件数量
     * 超过此数量将删除最旧的日志文件
     */
    pub max_files: usize,

    /**
     * 是否压缩旧文件
     * 
     * 控制是否压缩轮转后的旧日志文件
     */
    pub compress: bool,

    /**
     * 轮转策略
     * 
     * 日志文件轮转的策略
     * 支持按大小、时间或两者结合
     */
    pub strategy: RotationStrategy,
}

/**
 * 轮转策略枚举
 * 
 * 定义了日志文件轮转的策略
 */
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum RotationStrategy {
    /**
     * 按大小轮转
     * 
     * 当日志文件达到指定大小时进行轮转
     */
    Size,

    /**
     * 按时间轮转
     * 
     * 按指定的时间间隔进行轮转
     */
    Time,

    /**
     * 混合轮转
     * 
     * 同时考虑文件大小和时间间隔
     * 任一条件满足即触发轮转
     */
    Hybrid,
} 