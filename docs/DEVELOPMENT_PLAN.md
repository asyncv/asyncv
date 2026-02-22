# asyncv 开发计划

## 项目概述

`asyncv` 是一个常驻后台的 AI 助手服务，负责：

- 通过消息平台连接器连接各类消息机器人
- 通过抽象的 LLM Provider 接口连接不同 LLM 服务
- 通过运行时服务对外提供 MCP/HTTP/gRPC 调用接口
- 提供独立的 Studio Web UI，用于对不同 agent 与配置进行管理

本开发计划列出里程碑、主要任务、验收标准与初步时间估算。

## 目标里程碑（高层）

1. 项目初始化与本地开发环境 (1 周)
2. 核心运行时与 CLI (2 周)
3. 消息平台连接器框架 (2 周)
4. LLM Provider 抽象与至少一个适配器 (2 周)
5. MCP API 设计与实现 (1.5 周)
6. Studio 基础页面与 CRUD (2 周)
7. 测试、CI、文档与发布准备 (1.5 周)

> 总计：约 10-12 周（可并行化，多人协作可缩短）

## 详细任务拆分

- 环境与工具
  - 安装 Rust toolchain（与 `rust-toolchain` 保持一致）
  - 配置 IDE/格式化（rustfmt, clippy）
  - 本地运行：`cargo build` / `cargo test`

- 核心（`asyncv_core`）
  - 定义抽象类型：Agent、Task、Message、Session
  - 提供异步调度、持久化接口（可插拔）、事件总线
  - 验收：单元测试覆盖核心流程

 - 运行时服务（`asyncv_service`）
  - 生命周期管理、插件加载、配置文件解析
  - Expose gRPC/HTTP/MCP 运行时 API（供 agent 与客户端调用运行时功能）与受限管理接口（仅供管理员/运维执行授权操作）
  - 验收：能启动、加载配置、返回健康状态
  - 设计原则：运行时服务专注于应用运行时与业务逻辑，禁止承载或直接暴露专用于 Studio 的内部配置 API。Studio 的配置/管理功能通过自身的配置存储实现，并仅通过受限管理通道（带鉴权的同步/事件机制）与运行时服务交互，避免将 UI/配置逻辑耦合进运行时二进制。

- 命令行工具（`asyncv_cli`）
  - 常用命令：`start`、`studio`、`status`、`deploy` 等
  - 交互式配置与密钥管理
  - 验收：基本命令可操作守护进程

- 消息平台连接器
  - 设计统一的 connector trait（接收/发送/事件回调）
  - 实现示例：Slack / Telegram / 微信（占位或最少一项）
  - 验收：消息收发循环正常，能在 core 中产生事件

- LLM Provider 抽象
  - 定义 Provider trait（对话、生成、流式输出）
  - 实现至少一个适配器（OpenAI/本地 LLama/其他可选）
  - 验收：能通过 Provider 完成一次对话并返回结果

- Studio（前端）
  - 说明：配置/管理功能属于 Studio 的一部分，不单独拆分 `config-service`。
  - 独立服务（推荐）：简单界面包括 Agent 列表、配置页面、日志查看器等。
  - API 与边界：Studio 不应依赖守护进程提供 studio-only 的内部 API。配置 CRUD 由 Studio 内部的配置管理模块负责持久化，并通过受限且可鉴权的管理接口（或事件发布/订阅机制）将变更同步到守护进程。守护进程仅提供运行时所需的外部 API 和受限管理端点（仅限经过授权的操作）。
  - 部署选项：
    - 推荐：将 Studio 作为独立 Web 服务，与守护进程分离，互相通过安全网络 API 交互；
    - 可选：将 Studio 静态文件托管到独立静态服务器（仍保持 API 分离），但避免把 UI 承载进守护进程主二进制内。
  - 验收：可通过 UI 完成 Agent 的增删改查，且这些操作通过 Studio 自身的配置管理模块（持久化）并结合受限管理接口（同步/生效）完成；daemon 不直接暴露 studio-only API。

- 测试与 CI
  - 单元测试、集成测试（模拟 Provider/Connector）
  - CI 流程：`cargo check`、`cargo test`、clippy、fmt 检查
  - 验收：CI 绿灯、覆盖关键路径测试

- 文档与示例
  - 更新 `README.md`、添加 `docs/DEVELOPMENT_PLAN.md` 与使用示例
  - 提供快速上手示例（启动命令、配置样例）

## 验收标准（每个里程碑）

- 可复现的本地构建与测试
- 核心流程完整（消息->Agent->LLM->回复）
- 基础 API 与 UI 可交互
- 完整的 CI 流程

## 开发流程建议

- 分支策略：`main`（稳定）、`develop`（日常合并）、feature/*
- 代码审查：每次合并 require 1-2 个 review
- Issue 与 PR 模板：快速记录设计/测试要点

## 初始本地运行步骤（快速）

```bash
rustup toolchain install $(cat rust-toolchain)
cargo build --workspace
cargo test --workspace
```

## 下一步（短期）

1. 完成本地环境检查与 CI 基础（现在进行：环境搭建）
2. 设计 `asyncv_core` 的 public API（接口草案）
3. 开始实现守护进程基本启动/配置

---
*文档由开发计划脚本自动生成，保持此文件与 `README.md` 同步。*

## 模块化划分与技术栈

为便于实现与并行开发，按下列模块划分代码与职责。每个模块列出主要任务、建议技术栈与验收标准。

- `asyncv_core`（核心库）
  - 职责：定义 `Agent`、`Task` 等抽象；
  - 主要任务：抽象接口设计、事件流实现、持久化 trait、核心单元测试。
  - 技术栈：Rust（async/await、tokio）、serde、anyhow、thiserror。
  - 验收：单元测试覆盖主要流程，API 文档草案。

- `asyncv_service`（运行时服务）
  - 职责：运行 `asyncv_core`，加载 connector/provider 插件，提供对外的运行时 API（含 MCP/HTTP/gRPC）与受限管理端点。
  - 主要任务：服务生命周期、插件加载（动态或静态特征）、HTTP/gRPC/MCP 服务框架、健康检查与指标、鉴权骨架。
  - 技术栈：Rust、异步运行时（tokio）、HTTP/gRPC 框架（如 axum/tonic）、prometheus 导出器、数据库访问库（如 sqlx）。
  - 建议存储：SQLite（作为统一存储，兼顾开发与小型/中型生产场景）。为简化开发与迁移，新增单独 package `db_migrations`（或命名为 `migrations`）用于管理数据库结构与 migration，`studio` 与 CI 流程依赖该包以执行/检查迁移。可选 Redis 用作短期缓存/队列。
  - 验收：能启动并对外提供健康/运行时端点，加载并运行一个 connector 示例。

- `asyncv_cli`（命令行工具）
  - 职责：开发者/运维工具：启动/停止服务、查询状态、触发操作、快速交互式调试。
  - 主要任务：CLI 子命令、与 `asyncv_service` 的管理 API 客户端实现、配置与凭据管理、终端友好输出。
  - 技术栈：Rust、clap、reqwest/tonic 客户端、indicatif（可选进度条）。
  - 验收：基础命令可通过 REST/gRPC 操作服务并返回结果。

- `connectors/*`（消息平台适配器）
  - 职责：实现与外部消息平台的接入（接收事件、发送消息、回调处理）。
  - 主要任务：抽象 `Connector` trait、实现至少一个示例（例如 Slack 或 Telegram）、认证与重试策略。
  - 技术栈：Rust、平台 SDK（若有）、tokio、serde。
  - 验收：能在本地模拟环境中完成完整收发循环并在 core 中产生活动事件。

- `providers/*`（LLM Provider 适配器）
  - 职责：抽象 LLM 接口（同步/流式），实现具体适配器（OpenAI、local LLM 等）。
  - 主要任务：Provider trait、请求合并/限流策略、流式响应支持、测试适配器。
  - 技术栈：Rust、reqwest 或 grpc 客户端、tokio-stream、serde。
  - 验收：通过 provider 完成一次对话请求并获得合理返回。

- `db_migrations`（数据库迁移/结构管理 package）
  - 职责：管理 SQLite schema 与 migration 脚本，为开发、Studio 与 CI 提供一致的迁移运行与版本检查接口。
  - 主要任务：定义初始 schema、编写 migration 脚本、提供 migration runner（库 + CLI）、生成 schema 版本报告供 CI/Studio 校验。
  - 技术栈：Rust（可基于 `sqlx` 的迁移机制或使用 `refinery`/`barrel` 等迁移库）、sqlite（rusqlite/sqlx）、serde 用于导出/导入样例数据。
  - 验收：在本地与 CI 中能顺利执行迁移、回滚（可选）并保证 schema 版本一致；`studio` 可以调用该包的 API/CLI 来初始化或检查本地开发 DB。

- `studio`（Web UI + 配置管理，独立服务）
  - 职责：面向管理员/配置者的 UI（Agent 管理、配置编辑、日志与审计视图）；同时承载配置数据的存储、版本化与审计（CRUD）。
  - 主要任务：实现前端界面（Agent 列表、配置编辑、日志视图）；在 `studio` 内实现配置 CRUD、鉴权与变更事件推送（Webhook/消息队列，或通过 `asyncv_service` 的受限管理通道同步变更）；并使用 `db_migrations` 管理 SQLite schema，保证开发与 CI 中 schema 一致性。
  - 技术栈：前端使用 React tanstack + tailwindcss. 数据存储使用 SQLite。`studio` 对外通过安全 API 与 `asyncv_service` 交互，避免将 Studio 特有的内部 API 嵌入守护进程主二进制中。
  - 验收：能通过 UI/API 完成 Agent 与配置的增删改查；配置变更能被安全地版本化、审计并同步到 `asyncv_service`（通过受限管理接口或事件机制）。

- `infra/ops`（部署与运维）
  - 职责：打包与发布流程、CI/CD 配置与维护、CI 中运行测试与迁移验证、生成发布制品（tar/zip/cargo package）。无需设计或维护 Kubernetes/Docker 运行/日志/监控方案。
  - 技术栈：GitHub Actions（或其他 CI）、cargo-release、cargo package、简单的 artifact 存储（GitHub Releases/内部存储）、docker 构建脚本仅作为可选构件（非必须）。
  - 验收：CI 能够自动执行 `cargo check`/`cargo test`、运行 migration 检查、并产出可下载的发布制品；不要求部署到 k8s 或配置集中式日志系统。

- `testing-ci`（测试与 CI）
  - 职责：自动化测试与质量检查：单元、集成、端到端、契约测试。
  - 主要任务：模拟 provider/connector 的测试套件、CI 流水线配置（check、test、clippy、fmt）、coverage 报告。
  - 技术栈：`cargo test`、GitHub Actions、docker-compose（用于集成测试支撑服务）。
  - 验收：CI 流水线能自动执行并对关键路径通过测试。

---
上面的模块划分旨在支持并行开发、清晰所有权与部署弹性。后续可以将这些模块化内容同步到 `README.md`，并在 `docs/RFC/0001-module-boundaries.md` 等文档中沉淀为可审阅的设计说明。
