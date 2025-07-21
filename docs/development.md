# 开发指南

## 开发环境搭建

### 环境要求

- **Node.js**: 18.0.0 或更高版本
- **Rust**: 1.70.0 或更高版本
- **包管理器**: pnpm (推荐) 或 npm
- **操作系统**: Windows、macOS 或 Linux

### 安装步骤

1. **安装 Node.js**
   ```bash
   # 使用 nvm 安装 Node.js
   curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
   nvm install 18
   nvm use 18
   ```

2. **安装 Rust**
   ```bash
   # 安装 Rust
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

3. **安装 pnpm**
   ```bash
   npm install -g pnpm
   ```

4. **克隆项目**
   ```bash
   git clone https://github.com/your-username/influxdb-studio.git
   cd influxdb-studio
   ```

5. **安装依赖**
   ```bash
   # 安装前端依赖
   pnpm install
   
   # 安装 Rust 依赖
   cd src-tauri && cargo build
   ```

## 开发工作流

### 启动开发服务器

```bash
# 启动 Tauri 开发环境
pnpm tauri dev
```

### 构建应用

```bash
# 构建开发版本
pnpm tauri build --debug

# 构建生产版本
pnpm tauri build
```

### 代码检查

```bash
# 前端代码检查
pnpm lint

# Rust 代码检查
cd src-tauri && cargo check

# Rust 代码格式化
cd src-tauri && cargo fmt

# Rust 代码检查 (更严格)
cd src-tauri && cargo clippy
```

## 多版本 InfluxDB 支持开发

### 开发任务清单

#### 第一阶段：核心数据结构与UI框架

- [ ] **1.1: 定义新的数据模型**
  - 在 `src/types/` 目录下创建 TypeScript 类型定义
  - 定义 `InfluxDBVersion` 枚举
  - 定义各版本的配置接口
  - 定义统一的 `ConnectionProfile` 类型

- [ ] **1.2: 更新 Pinia Store**
  - 重构 `connectionStore.ts` 管理新的连接对象
  - 更新 getters 和 actions
  - 确保与后端数据结构一致

- [ ] **1.3: 重构连接对话框**
  - 添加版本选择器
  - 动态渲染对应版本的表单字段
  - 更新保存逻辑

- [ ] **1.4: 更新连接管理器**
  - 在列表中显示版本标签
  - 添加版本相关的视觉元素

#### 第二阶段：后端逻辑实现

- [ ] **2.1: 添加 Rust 依赖**
  - 在 `Cargo.toml` 中添加 `influxdb3_client`
  - 确认现有依赖版本兼容性

- [ ] **2.2: 定义 Rust 数据结构**
  - 创建与前端对应的 Rust struct
  - 使用 `serde` 进行序列化
  - 处理动态配置字段

- [ ] **2.3: 创建模块化服务**
  - 将现有逻辑拆分为 v1、v2、v3 模块
  - 实现各版本的客户端服务
  - 统一接口设计

- [ ] **2.4: 重构 Tauri 命令**
  - 修改现有命令接受新的参数结构
  - 根据版本调用对应模块
  - 处理配置反序列化

#### 第三阶段：功能整合与适配

- [ ] **3.1: 适配查询编辑器**
  - 根据版本动态判断查询语言
  - 配置 Monaco Editor 语法高亮
  - 支持不同版本的查询语法

- [ ] **3.2: 适配数据浏览器**
  - 根据版本调用不同的元数据接口
  - 在UI上显示正确的术语
  - 处理版本特定的数据结构

- [ ] **3.3: 全面测试**
  - 准备各版本的测试实例
  - 完整测试连接和查询流程
  - 验证版本兼容性

### UI/UX 设计

#### 连接配置对话框

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

## 开发环境故障排除

### 常见问题

#### 1. Tauri 开发环境启动失败

**问题**: 开发环境启动失败

**解决方案**:
```bash
# 清理 Vite 缓存
rm -rf node_modules/.vite

# 重新启动开发环境
pnpm tauri dev
```

#### 2. 端口占用问题

```bash
# 检查端口占用
lsof -ti:1420

# 释放端口
lsof -ti:1420 | xargs kill -9
```

#### 3. 依赖安装失败

```bash
# 清理缓存
pnpm store prune
rm -rf node_modules
pnpm install

# Rust 依赖问题
cd src-tauri
cargo clean
cargo build
```

### 开发环境优化

#### Vite 配置优化

在 `vite.config.ts` 中添加文件监视器忽略规则：

```typescript
export default defineConfig({
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      ignored: [
        "**/src-tauri/**",
        "**/node_modules/**"
      ],
    },
  },
  // ... 其他配置
});
```

#### 自动化脚本

创建开发环境准备脚本：

```bash
#!/bin/bash
# scripts/dev-setup.sh



# 清理缓存
rm -rf node_modules/.vite

# 启动开发环境
pnpm tauri dev
```

## 代码规范

### TypeScript 规范

- 使用 TypeScript 严格模式
- 优先使用接口定义数据结构
- 使用类型别名提高代码可读性
- 避免使用 `any` 类型

### Rust 规范

- 遵循 Rust 官方编码规范
- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 进行代码检查
- 编写单元测试和集成测试

### Vue 组件规范

- 使用 Composition API
- 组件名使用 PascalCase
- Props 定义使用 TypeScript 接口
- 事件名使用 kebab-case

## 测试指南

### 单元测试

```bash
# 前端测试
pnpm test

# Rust 测试
cd src-tauri && cargo test
```

### 集成测试

```bash
# 启动测试数据库
docker-compose up -d influxdb

# 运行集成测试
pnpm test:integration
```

### 手动测试

1. **连接测试**: 测试不同版本的 InfluxDB 连接
2. **查询测试**: 验证查询语法和结果解析
3. **UI 测试**: 检查界面响应和用户体验
4. **性能测试**: 测试大数据量查询性能

## 调试技巧

### 前端调试

- 使用 Vue DevTools 调试组件状态
- 使用浏览器开发者工具调试网络请求
- 使用 `console.log` 和断点调试

### 后端调试

- 使用 `println!` 或 `dbg!` 宏输出调试信息
- 使用 `cargo run --bin debug` 运行调试版本
- 使用 Rust Analyzer 进行代码分析

### 网络调试

- 使用 `curl` 测试 InfluxDB API
- 使用浏览器开发者工具查看网络请求
- 使用 Wireshark 分析网络流量 