# Alion Admin API 文档

## 目录

1. [认证与授权](#1-认证与授权-authentication--authorization)
2. [用户管理](#2-用户管理-user-management)
3. [角色管理](#3-角色管理-role-management)
4. [菜单管理](#4-菜单管理-menu-management)
5. [组织管理](#5-组织管理-organization-management)
6. [系统管理](#6-系统管理-system-management)
7. [沙箱环境](#7-沙箱环境-sandbox)
8. [通用说明](#通用说明)

## 1. 认证与授权 (Authentication & Authorization)

### 1.1 认证 API (`sys_authentication_api.rs`)

#### 1.1.1 用户登录
```http
POST /api/auth/login
```
**接口描述**：
- 用户登录接口，用于验证用户身份并获取访问令牌
- 登录成功后会返回访问令牌（token）和刷新令牌（refresh_token）
- 登录信息会被记录到登录日志中

**请求头**：
```
Content-Type: application/json
Accept: */*
```

**请求体**:
```json
{
    "username": "string",  // 用户名，长度至少5个字符
    "password": "string"     // 密码，长度至少6个字符
}
```

**参数说明**：
| 参数名 | 类型 | 必填 | 说明 |
|--------|------|------|------|
| username | string | 是 | 用户名，长度至少5个字符 |
| password | string | 是 | 密码，长度至少6个字符 |

**响应**:
```json
{
    "code": 200,            // 状态码：200表示成功
    "data": {
        "token": "string",  // JWT访问令牌，用于后续接口认证
        "refresh_token": "string"  // 刷新令牌，用于获取新的访问令牌
    }
}
```

**响应说明**：
| 字段 | 类型 | 说明 |
|------|------|------|
| code | number | 状态码，200表示成功 |
| data.token | string | JWT访问令牌，用于后续接口认证 |
| data.refresh_token | string | 刷新令牌，用于获取新的访问令牌 |

**错误码**：
| 状态码 | 说明 |
|--------|------|
| 400 | 请求参数错误（如：参数格式不正确、参数验证失败） |
| 401 | 认证失败（如：用户名或密码错误） |
| 500 | 服务器内部错误 |

**示例**：
```bash
# 请求示例
curl --location --request POST 'http://127.0.0.1:9528/api/auth/login' \
--header 'Content-Type: application/json' \
--data-raw '{
    "username": "admin",
    "password": "123456"
}'

# 成功响应示例
{
    "code": 200,
    "data": {
        "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
        "refresh_token": "01HNYVZR2P8Q4XKJ..."
    }
}

# 失败响应示例
{
    "code": 400,
    "data": null,
    "msg": "Failed to deserialize the JSON body into the target type: missing field `identifier`",
    "success": false
}
```

**注意事项**：
1. 登录成功后，需要在后续请求的 Header 中携带 token：
   ```
   Authorization: Bearer <token>
   ```
2. token 有效期为 7200 秒（2小时）
3. 登录失败次数过多可能会导致账号被锁定
4. 登录信息会被记录，包括：IP地址、登录时间、设备信息等

#### 1.1.2 获取用户信息
```http
GET /api/auth/userInfo
```
**响应**:
```json
{
    "code": 200,
    "data": {
        "user_id": "string",
        "user_name": "string",
        "roles": ["string"]
    }
}
```

#### 1.1.3 获取用户路由
```http
GET /api/auth/getUserRoutes
```
**响应**:
```json
{
    "code": 200,
    "data": {
        "routes": [
            {
                "path": "string",
                "component": "string",
                "name": "string",
                "meta": {
                    "title": "string",
                    "icon": "string",
                    "order": "number"
                },
                "children": []
            }
        ]
    }
}
```

#### 1.1.4 分配角色权限
```http
POST /api/auth/assignPermission
```
**请求体**:
```json
{
    "domain": "string",
    "role_id": "string",
    "permissions": [
        {
            "path": "string",
            "method": "string"
        }
    ]
}
```

#### 1.1.5 分配角色路由
```http
POST /api/auth/assignRoutes
```
**请求体**:
```json
{
    "domain": "string",
    "role_id": "string",
    "route_ids": ["string"]
}
```

### 1.2 访问密钥 API (`sys_access_key_api.rs`)

#### 1.2.1 创建访问密钥
```http
POST /api/auth/accessKey
```
**请求体**:
```json
{
    "name": "string",
    "expire_time": "string"
}
```

#### 1.2.2 获取访问密钥列表
```http
GET /api/auth/accessKey
```

#### 1.2.3 删除访问密钥
```http
DELETE /api/auth/accessKey/{id}
```

## 2. 用户管理 (User Management)

### 2.1 用户 API (`sys_user_api.rs`)

#### 2.1.1 创建用户
```http
POST /api/user
```
**请求体**:
```json
{
    "username": "string",
    "password": "string",
    "nick_name": "string",
    "email": "string",
    "phone_number": "string",
    "avatar": "string",
    "domain": "string"
}
```

#### 2.1.2 更新用户
```http
PUT /api/user/{id}
```
**请求体**:
```json
{
    "nick_name": "string",
    "email": "string",
    "phone_number": "string",
    "avatar": "string",
    "status": "enabled|disabled"
}
```

#### 2.1.3 删除用户
```http
DELETE /api/user/{id}
```

#### 2.1.4 获取用户列表
```http
GET /api/user
```
**查询参数**:
- `page`: 页码
- `size`: 每页大小
- `username`: 用户名（可选）
- `status`: 状态（可选）

#### 2.1.5 重置密码
```http
PUT /api/user/{id}/password
```
**请求体**:
```json
{
    "password": "string"
}
```

## 3. 角色管理 (Role Management)

### 3.1 角色 API (`sys_role_api.rs`)

#### 3.1.1 创建角色
```http
POST /api/role
```
**请求体**:
```json
{
    "name": "string",
    "code": "string",
    "description": "string",
    "domain": "string"
}
```

#### 3.1.2 更新角色
```http
PUT /api/role/{id}
```
**请求体**:
```json
{
    "name": "string",
    "description": "string",
    "status": "enabled|disabled"
}
```

#### 3.1.3 删除角色
```http
DELETE /api/role/{id}
```

#### 3.1.4 获取角色列表
```http
GET /api/role
```
**查询参数**:
- `page`: 页码
- `size`: 每页大小
- `name`: 角色名称（可选）
- `status`: 状态（可选）

## 4. 菜单管理 (Menu Management)

### 4.1 菜单 API (`sys_menu_api.rs`)

#### 4.1.1 创建菜单
```http
POST /api/menu
```
**请求体**:
```json
{
    "name": "string",
    "path": "string",
    "component": "string",
    "parent_id": "string",
    "icon": "string",
    "order": "number",
    "status": "enabled|disabled"
}
```

#### 4.1.2 更新菜单
```http
PUT /api/menu/{id}
```
**请求体**:
```json
{
    "name": "string",
    "path": "string",
    "component": "string",
    "icon": "string",
    "order": "number",
    "status": "enabled|disabled"
}
```

#### 4.1.3 删除菜单
```http
DELETE /api/menu/{id}
```

#### 4.1.4 获取菜单列表
```http
GET /api/menu
```
**查询参数**:
- `name`: 菜单名称（可选）
- `status`: 状态（可选）

## 5. 组织管理 (Organization Management)

### 5.1 组织 API (`sys_organization_api.rs`)

#### 5.1.1 创建组织
```http
POST /api/organization
```
**请求体**:
```json
{
    "name": "string",
    "parent_id": "string",
    "order": "number",
    "status": "enabled|disabled"
}
```

#### 5.1.2 更新组织
```http
PUT /api/organization/{id}
```
**请求体**:
```json
{
    "name": "string",
    "order": "number",
    "status": "enabled|disabled"
}
```

#### 5.1.3 删除组织
```http
DELETE /api/organization/{id}
```

#### 5.1.4 获取组织列表
```http
GET /api/organization
```
**查询参数**:
- `name`: 组织名称（可选）
- `status`: 状态（可选）

## 6. 系统管理 (System Management)

### 6.1 域管理 API (`sys_domain_api.rs`)

#### 6.1.1 创建域
```http
POST /api/domain
```
**请求体**:
```json
{
    "name": "string",
    "code": "string",
    "description": "string"
}
```

#### 6.1.2 更新域
```http
PUT /api/domain/{id}
```
**请求体**:
```json
{
    "name": "string",
    "description": "string"
}
```

#### 6.1.3 删除域
```http
DELETE /api/domain/{id}
```

#### 6.1.4 获取域列表
```http
GET /api/domain
```

### 6.2 端点管理 API (`sys_endpoint_api.rs`)

#### 6.2.1 创建端点
```http
POST /api/endpoint
```
**请求体**:
```json
{
    "path": "string",
    "method": "string",
    "description": "string"
}
```

#### 6.2.2 更新端点
```http
PUT /api/endpoint/{id}
```
**请求体**:
```json
{
    "path": "string",
    "method": "string",
    "description": "string"
}
```

#### 6.2.3 删除端点
```http
DELETE /api/endpoint/{id}
```

#### 6.2.4 获取端点列表
```http
GET /api/endpoint
```
**查询参数**:
- `path`: 路径（可选）
- `method`: 方法（可选）

### 6.3 日志管理

#### 6.3.1 操作日志 API (`sys_operation_log_api.rs`)
```http
GET /api/operation-log
```
**查询参数**:
- `page`: 页码
- `size`: 每页大小
- `username`: 用户名（可选）
- `operation`: 操作类型（可选）
- `start_time`: 开始时间（可选）
- `end_time`: 结束时间（可选）

#### 6.3.2 登录日志 API (`sys_login_log_api.rs`)
```http
GET /api/login-log
```
**查询参数**:
- `page`: 页码
- `size`: 每页大小
- `username`: 用户名（可选）
- `status`: 状态（可选）
- `start_time`: 开始时间（可选）
- `end_time`: 结束时间（可选）

## 7. 沙箱环境 (Sandbox)

### 7.1 沙箱 API (`sys_sandbox_api.rs`)
```http
GET /api/sandbox/test
```
用于测试环境的基本接口。

## 通用说明

### 响应格式
所有接口的响应都遵循以下格式：
```json
{
    "code": 200,       // 状态码
    "data": {},        // 响应数据
    "message": "string" // 响应消息（可选）
}
```

### 认证方式
除了登录接口外，所有接口都需要在请求头中携带 token：
```
Authorization: Bearer <token>
```

### 分页参数
所有列表接口都支持分页，使用以下参数：
- `page`: 页码（从1开始）
- `size`: 每页大小

### 状态码
- 200: 成功
- 400: 请求参数错误
- 401: 未认证
- 403: 无权限
- 404: 资源不存在
- 500: 服务器内部错误

### 数据状态
系统中的状态字段通常使用以下值：
- `enabled`: 启用
- `disabled`: 禁用

> **所有接口路径均已统一加上 `/api` 前缀。** 