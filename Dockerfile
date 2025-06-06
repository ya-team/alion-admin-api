ARG RUST_VERSION=1.86.0
ARG APP_NAME=server
ARG ALPINE_VERSION=3.21
ARG APP_PORT=10001
ARG APP_USER=appuser
ARG APP_UID=10001
ARG TZ=Asia/Shanghai

#################################################
# 构建阶段 - 编译Rust项目
#################################################
FROM rust:${RUST_VERSION}-alpine AS build
ARG APP_NAME
WORKDIR /app

# 安装必要的构建工具和依赖
# clang/lld: 用于更快的链接
# musl-dev: 提供C标准库
# openssl-dev/openssl-libs-static: SSL支持(同时包含动态和静态库)
RUN apk add --no-cache \
    clang lld musl-dev \
    git pkgconfig \
    openssl-dev openssl-libs-static

# 复制项目文件到容器中
COPY . .

# 构建可执行文件
# 使用Docker缓存挂载提升后续构建速度
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/app/target \
    cargo build --release --bin ${APP_NAME} --no-default-features && \
    cp target/release/${APP_NAME} /bin/server && \
    strip /bin/server

#################################################
# 运行阶段 - 创建精简的运行环境
#################################################
FROM alpine:${ALPINE_VERSION} AS final

# 参数传递到运行阶段
ARG APP_USER
ARG APP_UID
ARG TZ
ARG APP_PORT

# 配置容器环境
ENV TZ=${TZ} \
    LANG=en_US.UTF-8 \
    RUST_ENV=production

# 配置运行环境
# - 安装运行时依赖
# - 创建低权限用户
# - 创建必要的目录结构
RUN apk add --no-cache openssl ca-certificates tzdata && \
    adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${APP_UID}" \
    ${APP_USER} && \
    mkdir -p /app/server/resources && \
    chown -R ${APP_USER}:${APP_USER} /app

# 从构建阶段复制应用及配置文件
COPY --from=build /bin/server /bin/
COPY --from=build --chown=${APP_USER}:${APP_USER} /app/server/resources/application.yaml /app/server/resources/
COPY --from=build --chown=${APP_USER}:${APP_USER} /app/server/resources/ip2region.xdb /app/server/resources/
COPY --from=build --chown=${APP_USER}:${APP_USER} /app/server/resources/rbac_model.conf /app/server/resources/

# 设置工作目录和用户
WORKDIR /app
USER ${APP_USER}
EXPOSE ${APP_PORT}

# 启动服务
CMD ["/bin/server"]
