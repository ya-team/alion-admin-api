# 定义项目相关变量
PROJECT_NAME := alion-admin
REDIS_PROJECT_NAME := $(PROJECT_NAME)-redis
DEPLOY_DIR := deploy
REDIS_COMPOSE_FILE := docker-compose-redis-cluster.yml
MIGRATION_DIR := migration/src

# 格式化所有 Rust 代码
fmt:
	cargo fmt --all

# 运行服务器
run-server:
	cargo run --bin server

# 数据库迁移相关命令
run-migration:
	cargo run --bin migration

migrate-up:
	cargo run --bin migration -- up

migrate-down:
	cargo run --bin migration -- down

# 构建项目
build:
	cargo build --bin server --release --no-default-features

# 运行测试
test:
	cargo test

# 清理构建产物
clean:
	cargo clean

# Docker相关命令
# 定义通用的docker-compose命令模板
define docker_compose_cmd
	cd $(DEPLOY_DIR) && docker-compose -p $(PROJECT_NAME) $(1)
endef

define redis_cluster_cmd
	cd $(DEPLOY_DIR) && docker-compose -p $(REDIS_PROJECT_NAME) -f $(REDIS_COMPOSE_FILE) $(1)
endef

# 启动所有服务
docker-up:
	$(call docker_compose_cmd,up -d)

# 停止所有服务
docker-down:
	$(call docker_compose_cmd,down)

# 停止所有服务并删除数据卷
docker-down-v:
	$(call docker_compose_cmd,down -v)

# 查看服务状态
docker-ps:
	$(call docker_compose_cmd,ps)

# 查看服务日志
docker-logs:
	$(call docker_compose_cmd,logs -f)

# Redis 集群相关命令
# 启动 Redis 集群
redis-cluster-up:
	$(call redis_cluster_cmd,up -d)

# 停止 Redis 集群
redis-cluster-down:
	$(call redis_cluster_cmd,down)

# 停止 Redis 集群并删除数据卷
redis-cluster-down-v:
	$(call redis_cluster_cmd,down -v)

# 查看 Redis 集群状态
redis-cluster-ps:
	$(call redis_cluster_cmd,ps)

# 查看 Redis 集群日志
redis-cluster-logs:
	$(call redis_cluster_cmd,logs -f)

# 检查 Redis 集群信息
redis-cluster-info:
	@docker exec -it redis_1 sh -c 'echo 123456 | redis-cli -h 127.0.0.1 -p 7001 --user alion --askpass cluster info'

# 检查 Redis 集群节点
redis-cluster-nodes:
	@docker exec -it redis_1 sh -c 'echo 123456 | redis-cli -h 127.0.0.1 -p 7001 --user alion --askpass cluster nodes'

# 默认任务：格式化代码并运行服务器
.PHONY: default
default: fmt run-server

# 声明所有任务为伪目标
.PHONY: fmt run-server run-migration migrate-up migrate-down build test clean
	docker-up docker-down docker-down-v docker-ps docker-logs
	redis-cluster-up redis-cluster-down redis-cluster-down-v redis-cluster-ps redis-cluster-logs redis-cluster-info redis-cluster-nodes
	generate-schema-migration generate-data-migration

# 迁移文件生成通用函数
define check_name_param
	@if [ -z "$(name)" ]; then
		echo "Error: Please provide a name for the $(1) migration.";
		echo "Usage: make $(2) name=$(3)";
		exit 1;
	fi
endef

# 生成表结构迁移文件
# 用法: make generate-schema-migration name=table_name
# 例如: make generate-schema-migration name=sys_role
generate-schema-migration:
	$(call check_name_param,schema,generate-schema-migration,table_name)
	sea-orm-cli migrate generate --migration-dir $(MIGRATION_DIR)/schemas create_$(name)

# 生成数据迁移文件
generate-data-migration:
	$(call check_name_param,data,generate-data-migration,insert_default_data)
	sea-orm-cli migrate generate --migration-dir $(MIGRATION_DIR)/datas insert_$(name)
