/**
 * 域名相关输出参数定义
 * 
 * 包含域名信息的输出结构体。
 */

use sea_orm::FromQueryResult;

/**
 * 域名输出参数
 * 
 * 用于返回域名的详细信息。
 */
#[derive(Debug, FromQueryResult)]
pub struct DomainOutput {
    /** 域名ID */
    pub id: String,
    /** 域名编码 */
    pub code: String,
    /** 域名名称 */
    pub name: String,
    /** 域名描述 */
    pub description: Option<String>,
}
