/**
 * 接口相关输出参数定义
 * 
 * 包含接口树形结构的输出结构体。
 */

use serde::Serialize;

/**
 * 接口树形结构输出参数
 * 
 * 用于返回接口的树形结构信息。
 */
#[derive(Debug, Serialize, Clone)]
pub struct EndpointTree {
    /** 接口ID */
    pub id: String,
    /** 接口路径 */
    pub path: String,
    /** 请求方法 */
    pub method: String,
    /** 接口动作 */
    pub action: String,
    /** 资源名称 */
    pub resource: String,
    /** 控制器名称 */
    pub controller: String,
    /** 接口描述 */
    pub summary: Option<String>,
    /** 子接口列表 */
    pub children: Option<Vec<EndpointTree>>,
}
