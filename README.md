# InfluxDB Studio

一个基于 Tauri + Vue 3 的现代化 InfluxDB 管理工具，提供直观的图形界面来管理 InfluxDB 数据库、执行查询和可视化数据。

## 🚀 功能特性

### 核心功能
- **数据库连接管理** - 支持 InfluxDB v1.x、v2.x、v3.x 版本
- **数据库浏览器** - 可视化浏览数据库、测量值和标签
- **SQL 编辑器** - 基于 Monaco Editor 的智能 SQL 编辑器
- **查询结果展示** - 表格化展示查询结果，支持排序和分页
- **查询历史** - 自动保存查询历史，支持快速重执行
- **数据导出** - 支持查询结果导出为 JSON 格式

### 技术特性
- **现代化 UI** - 基于 Element Plus 的美观界面
- **响应式设计** - 支持桌面和移动设备
- **主题系统** - 支持浅色/深色主题切换
- **快捷键支持** - 完整的键盘快捷键操作
- **错误处理** - 完善的错误提示和处理机制

## 📋 系统要求

- **操作系统**: Windows 10+, macOS 10.15+, Linux (Ubuntu 18.04+)
- **内存**: 最少 4GB RAM
- **存储**: 最少 500MB 可用空间
- **网络**: 需要连接到 InfluxDB 服务器

## 🛠️ 开发环境搭建

### 前置要求
- Node.js 18+ 
- Rust 1.70+
- pnpm (推荐) 或 npm

### 安装依赖
```bash
# 克隆项目
git clone https://github.com/your-username/influxdb-studio.git
cd influxdb-studio

# 安装前端依赖
pnpm install

# 安装 Rust 依赖
cd src-tauri
cargo build
cd ..
```

### 启动开发服务器
```bash
# 启动前端开发服务器
pnpm dev

# 启动 Tauri 开发模式
pnpm tauri dev
```

## 🐳 使用 Docker 启动测试环境

### 启动 InfluxDB 测试服务
```bash
# 启动 InfluxDB v1.x 服务
docker-compose up influxdb-v1 -d

# 查看服务状态
docker-compose ps

# 查看日志
docker-compose logs influxdb-v1
```

### 测试连接配置
- **主机**: localhost
- **端口**: 8086
- **数据库**: testdb
- **用户名**: (留空，匿名访问)
- **密码**: (留空，匿名访问)

## 📖 使用指南

### 1. 连接数据库
1. 启动应用后，点击"管理连接"按钮
2. 点击"新建连接"创建新的数据库连接
3. 填写连接信息：
   - 连接名称：给连接起一个易记的名字
   - InfluxDB 版本：选择对应的版本
   - 主机地址：InfluxDB 服务器地址
   - 端口：InfluxDB 服务端口
   - 数据库：要连接的数据库名称
4. 点击"测试连接"验证连接是否正常
5. 点击"创建"保存连接配置

### 2. 浏览数据库
1. 连接成功后，左侧会显示数据库列表
2. 点击数据库名称展开查看测量值
3. 点击测量值名称查看详细信息
4. 右键点击数据库或测量值查看更多操作

### 3. 执行查询
1. 在 SQL 编辑器中输入 InfluxQL 查询语句
2. 选择要查询的数据库
3. 点击"运行"按钮执行查询
4. 在下方结果区域查看查询结果

### 4. 常用查询示例

#### 查看所有数据库
```sql
SHOW DATABASES
```

#### 查看数据库中的测量值
```sql
SHOW MEASUREMENTS
```

#### 查询时间序列数据
```sql
SELECT * FROM "cpu_usage" WHERE time > now() - 1h
```

#### 聚合查询
```sql
SELECT mean(value) FROM "cpu_usage" 
WHERE time > now() - 1h 
GROUP BY time(5m), host
```

#### 插入数据
```sql
INSERT cpu_usage,host=server03,region=us-central value=0.75
```

## 🎨 界面预览

### 主界面
- 左侧：数据库浏览器
- 中央：SQL 编辑器
- 下方：查询结果展示

### 功能区域
- **顶部工具栏**：连接状态、项目选择
- **数据库树**：数据库和测量值浏览
- **编辑器标签页**：多标签页 SQL 编辑
- **结果标签页**：表格、图表、输出日志

## 🔧 配置说明

### 连接配置
支持以下 InfluxDB 版本：
- **v1.x**: 传统版本，使用用户名/密码认证
- **v2.x**: 新版本，使用 Token 认证 (开发中)
- **v3.x**: 最新版本，兼容 v2.x (开发中)

### 主题配置
- 支持浅色和深色主题
- 可自定义 IDE 风格界面
- 响应式布局适配不同屏幕

## 🐛 故障排除

### 常见问题

#### 连接失败
1. 检查 InfluxDB 服务是否启动
2. 验证主机地址和端口是否正确
3. 确认防火墙设置允许连接
4. 检查网络连接状态

#### 查询执行失败
1. 确认选择了正确的数据库
2. 检查 InfluxQL 语法是否正确
3. 验证测量值名称是否存在
4. 查看错误日志获取详细信息

#### 应用启动失败
1. 检查 Node.js 和 Rust 版本
2. 重新安装依赖：`pnpm install`
3. 清理缓存：`pnpm clean`
4. 查看控制台错误信息

### 日志查看
- 应用日志：查看浏览器开发者工具控制台
- 后端日志：查看 Tauri 应用日志
- 数据库日志：查看 InfluxDB 服务日志

## 🤝 贡献指南

### 开发流程
1. Fork 项目到你的 GitHub 账户
2. 创建功能分支：`git checkout -b feature/your-feature`
3. 提交更改：`git commit -am 'Add some feature'`
4. 推送分支：`git push origin feature/your-feature`
5. 创建 Pull Request

### 代码规范
- 使用 TypeScript 进行类型检查
- 遵循 ESLint 和 Prettier 配置
- 编写单元测试覆盖新功能
- 更新文档说明新功能

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🙏 致谢

- [Tauri](https://tauri.app/) - 跨平台桌面应用框架
- [Vue 3](https://vuejs.org/) - 渐进式 JavaScript 框架
- [Element Plus](https://element-plus.org/) - Vue 3 组件库
- [Monaco Editor](https://microsoft.github.io/monaco-editor/) - 代码编辑器
- [InfluxDB](https://influxdata.com/) - 时间序列数据库

## 📞 联系方式

- 项目主页：https://github.com/your-username/influxdb-studio
- 问题反馈：https://github.com/your-username/influxdb-studio/issues
- 邮箱：your-email@example.com

---

**注意**: 本项目仍在积极开发中，部分功能可能不稳定。建议在生产环境使用前充分测试。 