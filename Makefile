# 项目基础配置
# ==============
# PROJECT_NAME: 项目名称，用于 Docker 容器命名
# REDIS_PROJECT_NAME: Redis 集群项目名称
# DEPLOY_DIR: 部署相关文件目录
# REDIS_COMPOSE_FILE: Redis 集群的 docker-compose 配置文件
# MIGRATION_DIR: 数据库迁移文件目录
PROJECT_NAME := alion-admin
REDIS_PROJECT_NAME := $(PROJECT_NAME)-redis
DEPLOY_DIR := deploy
REDIS_COMPOSE_FILE := docker-compose-redis-cluster.yml
MIGRATION_DIR := migration/src

# 代码格式化
# ==========
# 使用 rustfmt 格式化所有 Rust 源代码文件
fmt:
	cargo fmt --all

# 开发环境命令
# ============
# 运行开发服务器
run-server:
	cargo run --bin server

# 数据库迁移命令
# ==============
# run-migration: 运行迁移工具
# migrate-up: 执行所有待执行的迁移
# migrate-down: 回滚最后一次迁移
run-migration:
	cargo run --bin migration

migrate-up:
	cargo run --bin migration -- up

migrate-down:
	cargo run --bin migration -- down

# 构建和测试
# ==========
# build: 构建发布版本，禁用默认特性
# test: 运行所有测试
# clean: 清理所有构建产物
build:
	cargo build --bin server --release --no-default-features

test:
	cargo test

clean:
	cargo clean

# Docker 服务管理
# ==============
# 定义通用的 docker-compose 命令模板
# $(1) 将被替换为具体的 docker-compose 命令
define docker_compose_cmd
	cd $(DEPLOY_DIR) && docker-compose -p $(PROJECT_NAME) $(1)
endef

# 定义 Redis 集群的 docker-compose 命令模板
define redis_cluster_cmd
	cd $(DEPLOY_DIR) && docker-compose -p $(REDIS_PROJECT_NAME) -f $(REDIS_COMPOSE_FILE) $(1)
endef

# 基础服务命令
# -----------
# docker-up: 启动所有基础服务
# docker-down: 停止所有基础服务
# docker-down-v: 停止服务并删除数据卷
# docker-ps: 查看服务运行状态
# docker-logs: 查看服务日志
docker-up:
	$(call docker_compose_cmd,up -d)

docker-down:
	$(call docker_compose_cmd,down)

docker-down-v:
	$(call docker_compose_cmd,down -v)

docker-ps:
	$(call docker_compose_cmd,ps)

docker-logs:
	$(call docker_compose_cmd,logs -f)

# Redis 集群管理
# -------------
# redis-cluster-up: 启动 Redis 集群
# redis-cluster-down: 停止 Redis 集群
# redis-cluster-down-v: 停止集群并删除数据卷
# redis-cluster-ps: 查看集群节点状态
# redis-cluster-logs: 查看集群日志
# redis-cluster-info: 查看集群信息
# redis-cluster-nodes: 查看集群节点信息
redis-cluster-up:
	$(call redis_cluster_cmd,up -d)

redis-cluster-down:
	$(call redis_cluster_cmd,down)

redis-cluster-down-v:
	$(call redis_cluster_cmd,down -v)

redis-cluster-ps:
	$(call redis_cluster_cmd,ps)

redis-cluster-logs:
	$(call redis_cluster_cmd,logs -f)

redis-cluster-info:
	@docker exec -it redis_1 sh -c 'echo 123456 | redis-cli -h 127.0.0.1 -p 7001 --user alion --askpass cluster info'

redis-cluster-nodes:
	@docker exec -it redis_1 sh -c 'echo 123456 | redis-cli -h 127.0.0.1 -p 7001 --user alion --askpass cluster nodes'

# 默认任务
# =======
# 默认执行代码格式化并启动开发服务器
.PHONY: default
default: fmt run-server

# 声明所有任务为伪目标
# ==================
# 防止与同名文件冲突，确保任务总是执行
.PHONY: fmt run-server run-migration migrate-up migrate-down build test clean
	docker-up docker-down docker-down-v docker-ps docker-logs
	redis-cluster-up redis-cluster-down redis-cluster-down-v redis-cluster-ps redis-cluster-logs redis-cluster-info redis-cluster-nodes
	generate-schema-migration generate-data-migration

# 迁移文件生成
# ===========
# 检查迁移名称参数的通用函数
# $(1): 迁移类型描述
# $(2): 目标命令
# $(3): 参数名称示例
define check_name_param
	@if [ -z "$(name)" ]; then
		echo "Error: Please provide a name for the $(1) migration.";
		echo "Usage: make $(2) name=$(3)";
		exit 1;
	fi
endef

# 生成表结构迁移文件
# 用法: make generate-schema-migration name=table_name
# 示例: make generate-schema-migration name=sys_role
# 说明: 生成创建新表的迁移文件
generate-schema-migration:
	$(call check_name_param,schema,generate-schema-migration,table_name)
	sea-orm-cli migrate generate --migration-dir $(MIGRATION_DIR)/schemas create_$(name)

# 生成数据迁移文件
# 用法: make generate-data-migration name=insert_default_data
# 示例: make generate-data-migration name=insert_default_data
# 说明: 生成插入数据的迁移文件
generate-data-migration:
	$(call check_name_param,data,generate-data-migration,insert_default_data)
	sea-orm-cli migrate generate --migration-dir $(MIGRATION_DIR)/datas insert_$(name)
