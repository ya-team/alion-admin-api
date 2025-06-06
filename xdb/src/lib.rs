/**
 * IP地址搜索库
 * 
 * 该库提供了高性能的IP地址搜索功能，主要包含以下模块：
 * - ip_value: IP地址值转换模块，提供统一的IP地址格式转换接口
 * - searcher: IP地址搜索模块，提供高性能的IP地址位置查询功能
 */

mod ip_value;
pub use self::ip_value::ToUIntIP;
pub mod searcher;
pub use searcher::{search_by_ip, searcher_init};
