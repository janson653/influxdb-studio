# InfluxDB Studio

一个基于 Tauri + Vue 3 构建的跨平台 InfluxDB 桌面客户端，支持 InfluxDB v1.x、v2.x 和 v3.x。

## ✨ 特性

- 🚀 **跨平台支持**：Windows、macOS、Linux
- 🔌 **多版本兼容**：支持 InfluxDB v1.x
- 💻 **现代化界面**：基于 Vue 3 + Element Plus
- 🔍 **智能查询**：Monaco Editor 提供语法高亮
- 📊 **数据可视化**：内置图表和数据分析功能
- 🔐 **安全连接**：支持多种认证方式

## 🚀 快速开始

### 环境要求

- Node.js 18+
- Rust 1.70+
- pnpm (推荐) 或 npm

### 安装依赖

```bash
# 安装前端依赖
pnpm install

# 安装 Rust 依赖
cd src-tauri && cargo build
```

### 开发模式

```bash
# 启动开发服务器
pnpm tauri dev
```

### 构建应用

```bash
# 构建所有平台
pnpm tauri build
```

## 📚 文档导航

### 开发文档
- [技术架构](./docs/architecture.md) - 详细的技术架构和设计说明
- [开发指南](./docs/development.md) - 开发环境搭建和开发流程
- [API 文档](./docs/api.md) - 前后端接口说明

### 部署文档
- [发布指南](./docs/release.md) - 应用打包和发布流程
- [Flatpak 构建](./docs/flatpak.md) - Linux Flatpak 包构建

### 故障排除
- [常见问题](./docs/troubleshooting.md) - 开发和生产环境问题解决

## 🏗️ 项目结构

```
influxdb-studio/
├── src/                    # 前端源码 (Vue 3 + TypeScript)
├── src-tauri/             # 后端源码 (Rust + Tauri)
├── docs/                  # 项目文档
├── scripts/               # 构建和部署脚本
└── flatpak/              # Flatpak 包配置
```

## 🤝 贡献指南

1. Fork 项目
2. 创建功能分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 打开 Pull Request

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🔗 相关链接

- [InfluxDB 官网](https://www.influxdata.com/)
- [Tauri 框架](https://tauri.app/)
- [Vue.js](https://vuejs.org/)
- [Element Plus](https://element-plus.org/) 