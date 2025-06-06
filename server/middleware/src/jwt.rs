/**
 * JWT认证中间件模块
 * 
 * 本模块提供了基于JWT（JSON Web Token）的认证中间件功能，
 * 用于验证请求中的JWT令牌并提取用户信息。
 * 
 * 主要功能：
 * - 从请求头中提取Bearer令牌
 * - 验证令牌的有效性
 * - 解析令牌中的用户信息
 * - 将用户信息注入到请求上下文中
 */

use axum::{
    body::Body, extract::Request, http::StatusCode, middleware::Next, response::IntoResponse,
};
use axum_casbin::CasbinVals;
use headers::{authorization::Bearer, Authorization, HeaderMapExt};
use server_core::web::{auth::User, jwt::JwtUtils, res::Res};

/**
 * JWT认证中间件
 * 
 * 验证请求中的JWT令牌，并将用户信息注入到请求上下文中。
 * 
 * # 参数
 * - req: 原始HTTP请求
 * - next: 下一个中间件或处理函数
 * - audience: JWT令牌的目标受众
 * 
 * # 返回
 * - 如果令牌有效，返回下一个中间件的响应
 * - 如果令牌无效或缺失，返回401 Unauthorized错误
 * 
 * # 处理流程
 * 1. 从请求头中提取Bearer令牌
 * 2. 验证令牌的有效性
 * 3. 解析令牌中的用户信息
 * 4. 将用户信息注入到请求上下文中
 * 5. 调用下一个中间件或处理函数
 */
pub async fn jwt_auth_middleware(
    mut req: Request<Body>,
    next: Next,
    audience: &str,
) -> impl IntoResponse {
    let token = match req.headers().typed_get::<Authorization<Bearer>>() {
        Some(auth) => auth.token().to_string(),
        None => {
            return Res::<String>::new_error(
                StatusCode::UNAUTHORIZED.as_u16(),
                "No token provided or invalid token type",
            )
            .into_response();
        },
    };

    match JwtUtils::validate_token(&token, audience).await {
        Ok(data) => {
            let claims = data.claims;
            let user = User::from(claims);
            let vals = CasbinVals {
                subject: user.subject(),
                domain: Option::from(user.domain()),
            };
            req.extensions_mut().insert(user);
            req.extensions_mut().insert(vals);
            next.run(req).await.into_response()
        },
        Err(err) => {
            Res::<String>::new_error(StatusCode::UNAUTHORIZED.as_u16(), err.to_string().as_str())
                .into_response()
        },
    }
}
