# Alion Admin

<!-- markdownlint-disable-next-line -->
<div align="center">

![license](https://img.shields.io/badge/license-Apache--2.0-blue.svg)
![stars](https://img.shields.io/github/stars/alionjs/alion-admin.svg)
![forks](https://img.shields.io/github/forks/alionjs/alion-admin.svg)
![issues](https://img.shields.io/github/issues/alionjs/alion-admin.svg)

[简介](#简介) •
[特性](#特性) •
[项目结构](#项目结构) •
[快速开始](#快速开始) •
[技术栈](#技术栈) •
[贡献指南](#贡献指南) •
[许可证](#许可证)

</div>

## 简介

Alion Admin 是一个基于 Rust 语言开发的现代化后台管理系统脚手架。项目采用 Axum 作为 Web 框架，SeaORM 作为数据库 ORM，提供了完整的后台管理基础功能实现。本项目特别注重性能和安全性，充分发挥了 Rust 语言的优势，为开发者提供一个强类型、高性能的后台管理系统开发基础。

项目实现了基础的 RBAC 权限管理体系，包括用户管理、角色管理、菜单管理等核心功能。无论是作为学习 Rust Web 开发的示例，还是作为实际项目的起点，都是一个理想的选择。

## 特性

- **高性能框架**：基于 Axum 的异步 Web 框架，提供极致的性能表现
- **类型安全**：充分利用 Rust 的类型系统，在编译时捕获潜在错误
- **基础功能**：
  - 用户管理
  - 角色管理
  - 菜单管理
- **权限管理**：
  - 基于 Casbin 的 RBAC 权限控制
  - 支持多种权限模型配置
  - 灵活的策略管理
- **数据库操作**：
  - 使用 SeaORM 提供类型安全的数据库操作
  - 支持数据库迁移
  - 连接池管理
- **安全特性**：
  - JWT 身份认证
  - 密码加密存储
  - CORS 配置
- **开发体验**：
  - 模块化的项目结构
  - 统一的错误处理
  - 完整的日志系统
  - YAML 配置文件支持 (未来将支持 TOML、Properties 等)

## 开发计划

- [x] 操作日志记录
- [ ] 请求限流
- [x] Redis 支持
- [ ] API 文档生成
- [x] 多配置文件格式支持
- [ ] 更多权限模型支持

## 项目结构

```tree
alion-admin/
├── .cargo/               # Cargo 配置
├── axum-casbin/          # Axum Casbin 适配器
├── migration/            # 数据库迁移
├── sea-orm-adapter/      # SeaORM Casbin 适配器
├── server/               # 服务端代码
│   ├── api/              # API 接口定义
│   ├── bin/              # 可执行文件
│   ├── config/           # 配置管理
│   ├── constant/         # 常量定义
│   ├── core/             # 核心功能模块
│   ├── global/           # 全局变量和状态
│   ├── initialize/       # 初始化逻辑
│   ├── middleware/       # 中间件
│   ├── model/            # 数据模型
│   ├── resource/         # 资源文件
│   ├── resources/        # 静态资源
│   ├── router/           # 路由定义
│   ├── service/          # 业务逻辑
│   ├── shared/           # 共享代码
│   └── utils/            # 工具函数
├── xdb/                  # ip2region
```

## 快速开始

### 环境要求

- Rust 1.75.0 或更高版本
- PostgreSQL 13 或更高版本

### 安装步骤

1. 克隆项目

    ```bash
    git clone git@github.com:ya-team/alion-admin-api.git
    cd alion-admin-api
    ```

2. 配置数据库

    ```bash
    # 编辑配置 本地开发环境
    vim server/resources/application-test.yaml
    ```

3. 运行迁移

    ```bash
    # copy .env.example 为 .env
    cp .env.example .env
    # 修改.env文件
    vim .env
    # 运行迁移
    make run-migration
    # 或
    cargo run --bin migration
    ```

4. 启动服务

    ```bash
    make run-server
    # 或
    cargo run --bin server
    ```

服务将在 `http://localhost:9528` 启动

## 技术栈

- **Web 框架**: Axum
- **ORM**: SeaORM
- **数据库**: PostgreSQL
- **认证**: JWT
- **权限**: Casbin
- **日志**: tracing
- **配置**: YAML

## 贡献指南

1. Fork 本仓库
2. 创建您的特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交您的改动 (`git commit -m 'feat: add some feature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启一个 Pull Request

## 许可证

本项目采用 Apache License 2.0 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情

如果您觉得这个项目对您有帮助，请给我们一个 ⭐️，这将鼓励我们持续改进！
