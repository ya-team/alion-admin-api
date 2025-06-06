/// 日志配置模块
/// 
/// 定义了应用程序日志记录的相关参数，包括日志级别、输出格式和存储位置

use serde::Deserialize;

/// 日志配置结构体
/// 
/// 包含日志记录所需的所有参数，包括：
/// - 日志级别
/// - 日志文件配置
/// - 日志格式设置
#[derive(Deserialize, Debug, Clone)]
pub struct LogConfig {
    /// 日志级别
    /// 
    /// 控制日志记录的详细程度
    /// 可选值：
    /// - "error": 只记录错误信息
    /// - "warn": 记录警告和错误信息
    /// - "info": 记录一般信息、警告和错误
    /// - "debug": 记录调试信息、一般信息、警告和错误
    /// - "trace": 记录所有级别的信息
    pub level: String,

    /// 日志文件路径
    /// 
    /// 日志文件的存储位置
    /// 可以是相对路径或绝对路径
    /// 例如：
    /// - "logs/app.log"
    /// - "/var/log/myapp/app.log"
    pub file_path: String,

    /// 是否启用控制台输出
    /// 
    /// 控制是否将日志同时输出到控制台
    /// 在开发环境中通常启用，生产环境可以禁用
    pub enable_console: bool,

    /// 是否启用文件轮转
    /// 
    /// 控制是否在日志文件达到一定大小时进行轮转
    /// 启用后可以防止日志文件过大
    pub enable_rotation: bool,

    /// 单个日志文件最大大小（字节）
    /// 
    /// 当日志文件达到此大小时进行轮转
    /// 仅在启用文件轮转时有效
    pub max_file_size: Option<u64>,

    /// 保留的日志文件数量
    /// 
    /// 控制保留多少个历史日志文件
    /// 仅在启用文件轮转时有效
    pub max_files: Option<u32>,

    /// 日志格式
    /// 
    /// 控制日志输出的格式
    /// 可选值：
    /// - "json": JSON格式，便于机器处理
    /// - "text": 文本格式，便于人工阅读
    pub format: LogFormat,

    /// 是否包含时间戳
    /// 
    /// 控制是否在日志中包含时间戳
    /// 建议在生产环境中启用
    pub include_timestamp: bool,

    /// 是否包含线程ID
    /// 
    /// 控制是否在日志中包含线程ID
    /// 对于多线程应用程序很有用
    pub include_thread_id: bool,

    /// 是否包含文件位置
    /// 
    /// 控制是否在日志中包含代码文件位置
    /// 对于调试很有用，但可能影响性能
    pub include_file_location: bool,
}

/// 日志格式枚举
/// 
/// 定义了日志输出的格式类型
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum LogFormat {
    /// JSON格式
    /// 
    /// 将日志输出为JSON格式，便于机器处理和分析
    /// 例如：
    /// json
    /// {
    ///   "timestamp": "2024-03-20T10:00:00Z",
    ///   "level": "INFO",
    ///   "message": "Application started",
    ///   "thread_id": "123",
    ///   "file": "src/main.rs:10"
    /// }
    /// 
    Json,

    /// 文本格式
    /// 
    /// 将日志输出为人类可读的文本格式
    /// 例如：
    /// 
    /// [2024-03-20T10:00:00Z] INFO [thread-123] src/main.rs:10 - Application started
    /// 
    Text,
} 