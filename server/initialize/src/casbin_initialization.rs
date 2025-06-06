/**
 * Casbin权限控制初始化模块
 * 
 * 本模块负责初始化Casbin权限控制系统，包括：
 * - 加载RBAC模型配置
 * - 创建数据库适配器
 * - 初始化Casbin中间件
 */

use std::error::Error;

use axum_casbin::CasbinAxumLayer;
use casbin::DefaultModel;
use sea_orm::Database;
use sea_orm_adapter::SeaOrmAdapter;

use crate::project_info;

/**
 * 初始化Casbin权限控制系统
 * 
 * # 参数
 * - model_path: RBAC模型配置文件路径
 * - db_url: 数据库连接URL
 * 
 * # 返回
 * - 成功：返回CasbinAxumLayer实例
 * - 失败：返回错误信息
 * 
 * # 处理流程
 * 1. 从文件加载RBAC模型
 * 2. 创建数据库连接
 * 3. 初始化数据库适配器
 * 4. 创建Casbin中间件
 */
pub async fn initialize_casbin(
    model_path: &str,
    db_url: &str,
) -> Result<CasbinAxumLayer, Box<dyn Error>> {
    project_info!("Initializing Casbin with model: {}", model_path);
    let model = DefaultModel::from_file(model_path).await?;
    let db = Database::connect(db_url).await?;
    let adapter = SeaOrmAdapter::new(db).await?;

    let casbin_axum_layer = CasbinAxumLayer::new(model, adapter).await?;
    project_info!("Casbin initialization completed successfully");
    Ok(casbin_axum_layer)
}
