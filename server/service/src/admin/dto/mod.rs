/// 管理后台数据传输对象(DTO)模块
/// 
/// 该模块定义了管理后台服务中使用的数据传输对象，用于：
/// - 请求参数验证和转换
/// - 响应数据封装和格式化
/// - 服务间数据传递
/// 
/// # 主要组件
/// 
/// ## 认证相关DTO
/// * `LoginRequest`: 登录请求参数
/// * `LoginResponse`: 登录响应数据
/// * `TokenResponse`: 令牌响应数据
/// 
/// ## 用户相关DTO
/// * `UserCreateRequest`: 用户创建请求
/// * `UserUpdateRequest`: 用户更新请求
/// * `UserResponse`: 用户信息响应
/// 
/// ## 角色相关DTO
/// * `RoleCreateRequest`: 角色创建请求
/// * `RoleUpdateRequest`: 角色更新请求
/// * `RoleResponse`: 角色信息响应
/// 
/// # 使用示例
/// 
/// use server_service::admin::dto::*;
/// 
/// // 创建登录请求
/// let login_req = LoginRequest {
///     username: "admin".to_string(),
///     password: "password".to_string(),
/// };
/// 
/// // 创建用户请求
/// let user_req = UserCreateRequest {
///     username: "new_user".to_string(),
///     email: "user@example.com".to_string(),
///     // ...其他字段
/// };
/// 

pub mod sys_auth_dto;
