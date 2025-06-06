/**
 * 端点管理API
 * 
 * 提供端点管理的相关接口，包括：
 * - 分页查询端点列表
 * - 获取角色的授权端点
 * - 获取端点树形结构
 */
use std::{collections::BTreeMap, sync::Arc};

use axum::{
    extract::{Path, Query},
    Extension,
};
use axum_casbin::{casbin::MgmtApi, CasbinAxumLayer};
use server_core::web::{auth::User, error::AppError, page::PaginatedData, res::Res};
use server_service::admin::{
    EndpointPageRequest, EndpointTree, SysEndpointModel, SysEndpointService, TEndpointService,
};

pub struct SysEndpointApi;

impl SysEndpointApi {
    /**
     * 分页查询端点列表
     * 
     * # 参数
     * - params: 分页查询参数
     * - service: 端点服务实例
     * 
     * # 返回
     * 返回分页后的端点列表数据
     */
    pub async fn get_paginated_endpoints(
        Query(params): Query<EndpointPageRequest>,
        Extension(service): Extension<Arc<SysEndpointService>>,
    ) -> Result<Res<PaginatedData<SysEndpointModel>>, AppError> {
        service
            .find_paginated_endpoints(params)
            .await
            .map(Res::new_data)
    }

    /**
     * 获取角色的授权端点
     * 
     * # 参数
     * - role_code: 角色代码
     * - user: 当前认证用户信息
     * - cache_enforcer: Casbin执行器
     * 
     * # 返回
     * 返回角色被授权的端点列表
     */
    pub async fn get_auth_endpoints(
        Path(role_code): Path<String>,
        Extension(user): Extension<User>,
        Extension(mut cache_enforcer): Extension<CasbinAxumLayer>,
    ) -> Result<Res<Vec<BTreeMap<String, String>>>, AppError> {
        let enforcer = cache_enforcer.get_enforcer();
        let enforcer_read = enforcer.read().await;

        let policies = enforcer_read.get_filtered_policy(0, vec![role_code, user.domain()]);

        let formatted_policies = policies
            .into_iter()
            .map(|policy| {
                let mut values = policy.into_iter();
                let mut map = BTreeMap::new();
                map.insert("v0".to_string(), values.next().unwrap_or_default());
                map.insert("v1".to_string(), values.next().unwrap_or_default());
                map.insert("v2".to_string(), values.next().unwrap_or_default());
                map.insert("v3".to_string(), values.next().unwrap_or_default());
                map
            })
            .collect();

        Ok(Res::new_data(formatted_policies))
    }

    /**
     * 获取端点树形结构
     * 
     * # 参数
     * - service: 端点服务实例
     * 
     * # 返回
     * 返回端点的树形结构数据
     */
    pub async fn tree_endpoint(
        Extension(service): Extension<Arc<SysEndpointService>>,
    ) -> Result<Res<Vec<EndpointTree>>, AppError> {
        service.tree_endpoint().await.map(Res::new_data)
    }
}
