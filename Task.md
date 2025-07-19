# InfluxDB 多版本支持开发任务清单

本文档根据功能规划，列出了支持 InfluxDB v1.x, v2.x 所需的开发任务。

---

## UI/UX 设计示意图

### 连接配置对话框 (`ConnectionDialog.vue`)

为了支持多版本，我们将表单设计为动态响应式。

```
+----------------------------------------------------------+
| 新建连接 / 编辑连接                                [ X ] |
+----------------------------------------------------------+
|                                                          |
|  连接名称: [ My Home Server          ]                   |
|                                                          |
|  InfluxDB 版本: [ v1.x ▼]  (v1.x | v2.x | v3.x)         |
|                                                          |
|  +----------------------------------------------------+  |
|  |         ▼ 根据所选版本动态显示的表单 ▼             |  |
|  +----------------------------------------------------+  |
|  |                                                    |  |
|  |  [当选择 v1.x 时]                                  |  |
|  |  Host 地址:  [ http://localhost:8086 ]              |  |
|  |  用户名:    [ admin                 ] (可选)       |  |
|  |  密码:      [ ************          ] (可选)       |  |
|  |  数据库:    [ telegraf              ]              |  |
|  |                                                    |  |
|  |  [当选择 v2.x 时]                                  |  |
|  |  Host 地址:  [ http://localhost:8086 ]              |  |
|  |  Token:     [ my-super-secret-token ]              |  |
|  |  组织(Org): [ my-org                ]              |  |
|  |                                                    |  |
|  |  [当选择 v3.x 时]                                  |  |
|  |  Host 地址:  [ https://us-east-1-1.aws... ]         |  |
|  |  Token:     [ my-super-secret-token ]              |  |
|  |  数据库:    [ my-database           ]              |  |
|  |                                                    |  |
|  +----------------------------------------------------+  |
|                                                          |
|                                     [ 测试连接 ] [ 保存 ] |
+----------------------------------------------------------+
```

### 连接管理器 (`ConnectionManager.vue`)

在列表中为每个连接添加版本标签。

```
+-------------------------------------------------------------------+
| 我的连接                                                          |
+-------------------------------------------------------------------+
|                                                                   |
|  My Home Server      (v1.x)      [ 连接 ] [ 编辑 ] [ 删除 ]         |
|  ---------------------------------------------------------------  |
|  InfluxDB Cloud EU   (v3.x)      [ 连接 ] [ 编辑 ] [ 删除 ]         |
|  ---------------------------------------------------------------  |
|  Staging Cluster     (v2.x)      [ 连接 ] [ 编辑 ] [ 删除 ]         |
|                                                                   |
+-------------------------------------------------------------------+
```

---

## 开发任务清单

### 第一阶段：核心数据结构与UI框架

- [ ] **1.1: (Frontend) 定义新的数据模型**
    - 在 `src/stores/` 或 `src/types/` 目录下创建新的 TypeScript 类型定义。
    - 定义 `InfluxDBVersion` 枚举。
    - 定义 `InfluxDBV1Config`, `InfluxDBV2Config`, `InfluxDBV3Config` 接口。
    - 定义统一的 `ConnectionProfile` 类型。

- [ ] **1.2: (Frontend) 更新 Pinia Store (`connectionStore.ts`)**
    - 将 store 的 state 从管理旧的连接对象列表，重构为管理 `ConnectionProfile[]`。
    - 更新 getters，例如 `activeConnection`，确保其能正确返回 `ConnectionProfile`。
    - 重构 actions (`addConnection`, `updateConnection`, `deleteConnection`) 以处理新的数据结构。

- [ ] **1.3: (Frontend) 重构连接对话框 (`ConnectionDialog.vue`)**
    - 添加 "InfluxDB Version" 选择器。
    - 使用 `v-if` 或 `v-show` 根据版本选择器的值，动态渲染对应的表单输入字段。
    - 更新 `save` 方法，使其能构建并提交正确的 `ConnectionProfile` 对象。

- [ ] **1.4: (Frontend) 更新连接管理器 (`ConnectionManager.vue`)**
    - 在连接列表中循环渲染时，读取每个连接的 `version` 属性���
    - 添加一个视觉元素 (如 `el-tag` 或 `badge`) 来显示版本号。

### 第二阶段：后端逻辑实现 (Tauri/Rust)

- [ ] **2.1: (Backend) 添加 Rust 依赖**
    - 在 `src-tauri/Cargo.toml` 的 `[dependencies]` 部分添加 `influxdb3_client`。
    - 确认 `influxdb` crate (用于 v1/v2) 已存在且版本合适。
    - 添加 `serde_json` 用于处理动态配置。

- [ ] **2.2: (Backend) 定义 Rust 数据结构 (`src-tauri/src/models.rs`)**
    - 创建与前端 `ConnectionProfile` 对应的 Rust struct，并派生 `serde::Deserialize`。
    - 使用 `serde_json::Value` 或枚举来处理 `config` 字段，以便进行后续的反序列化。

- [ ] **2.3: (Backend) 创建模块化服务 (`src-tauri/src/influxdb.rs`)**
    - 将现有逻辑拆分为子模块，例如 `v1`, `v2`, `v3`。
    - 在每个模块中，实现各自的 `execute_query`, `list_databases` (或 `list_buckets`) 等函数。
    - 这些函数应接受各自版本的 `Config` 结构体作为参数。

- [ ] **2.4: (Backend) 重构 Tauri 命令 (`src-tauri/src/commands.rs`)**
    - 修改现有的 tauri command (如 `execute_query`)，使其接受 `ConnectionProfile` 作为参数。
    - 在命令函数内部，使用 `match` 语句根据 `profile.version` 来调用对应模块的服务函数。
    - 在 `match` 内部，将 `profile.config` (`serde_json::Value`) 反序列化为具体版本的 `Config` 结构体。

### 第三阶段：功能整合与适配

- [ ] **3.1: (Frontend) 适配查询编辑器 (`QueryEditor.vue`)**
    - 根据当前激活连接的 `version`，动态判断查询语言 (`InfluxQL`, `Flux`, `SQL`)。
    - 将判断出的语言传递给 `MonacoEditor.vue` 组件，以实现正确的语法高亮。
    - 可能需要为 `MonacoEditor.vue` 添加一个 `language` prop。

- [ ] **3.2: (Frontend) 适配数据浏览器 (`DatabaseExplorer.vue`)**
    - 获取元数据（数据库/Bucket列表等）的逻辑需要变为条件式。
    - 根据激活连接的 `version`，调用不同的 Tauri 命令来获取数据。
    - 在UI上显示正确的术语（例如，根据版本显示 "Databases" 或 "Buckets"）。

- [ ] **3.3: (Testing) 全面测试**
    - 准备 v1, v2, v3 三个版本的 InfluxDB 测试实例。
    - 对每个版本，完整测试以下流程：
        - [ ] 创建连接
        - [ ] 编辑连接
        - [ ] 测试连接
        - [ ] 成功连接并加载数据浏览器
        - [ ] 执行查询并获取结果
        - [ ] 删除连接
