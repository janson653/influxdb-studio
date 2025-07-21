# InfluxDB Studio 开发计划

## 当前状态

InfluxDB Studio 目前仅支持 **InfluxDB v1.x** 版本。v2.x 和 v3.x 支持正在开发中。

## 版本支持计划

### ✅ 已完成
- [x] InfluxDB v1.x 完整支持
  - [x] 连接管理
  - [x] 查询执行
  - [x] 数据写入
  - [x] 数据库管理
  - [x] 测量值管理

### 🚧 开发中
- [ ] InfluxDB v2.x 支持
  - [ ] 连接配置（Token、Org、Bucket）
  - [ ] Flux 查询语言支持
  - [ ] 存储桶管理
  - [ ] 组织管理
  - [ ] 数据写入 API

### 📋 计划中
- [ ] InfluxDB v3.x 支持
  - [ ] 连接配置（Token、Database）
  - [ ] SQL 查询支持
  - [ ] 数据库管理
  - [ ] 表管理
  - [ ] 数据写入 API

## 技术实现细节

### v2.x 支持实现要点

1. **连接配置**
   - 使用 Token 认证替代用户名/密码
   - 支持组织（Org）和存储桶（Bucket）配置
   - 实现 v2.x 特定的连接验证

2. **查询语言**
   - 集成 Flux 查询语言支持
   - 实现 Flux 语法高亮和自动补全
   - 支持 Flux 查询验证和格式化

3. **数据管理**
   - 存储桶列表和管理
   - 组织管理功能
   - v2.x 特定的数据写入格式

### v3.x 支持实现要点

1. **连接配置**
   - 使用 Token 认证
   - 支持数据库配置（类似 v1.x）
   - 实现 v3.x 特定的连接验证

2. **查询语言**
   - 支持 SQL 查询（InfluxDB 3.x 新特性）
   - 保持 InfluxQL 兼容性
   - 实现 SQL 语法高亮和验证

3. **数据管理**
   - 数据库和表管理
   - v3.x 特定的数据写入格式
   - 支持新的数据模型

## 开发时间线

### 第一阶段：v2.x 支持（预计 2-3 个月）
- 第 1 个月：连接管理和基础 API
- 第 2 个月：Flux 查询支持
- 第 3 个月：UI 适配和测试

### 第二阶段：v3.x 支持（预计 2-3 个月）
- 第 1 个月：连接管理和 SQL 支持
- 第 2 个月：数据管理功能
- 第 3 个月：UI 适配和测试

## 贡献指南

如果您想参与 v2.x 或 v3.x 支持的开发，请：

1. 查看 [开发指南](./development.md)
2. 了解项目架构 [技术架构](./architecture.md)
3. 提交 Issue 讨论具体实现方案
4. 创建 Pull Request 贡献代码

## 相关资源

- [InfluxDB v2.x 文档](https://docs.influxdata.com/influxdb/v2.7/)
- [InfluxDB v3.x 文档](https://docs.influxdata.com/influxdb/v3.0/)
- [Flux 查询语言](https://docs.influxdata.com/flux/)
- [InfluxDB SQL 参考](https://docs.influxdata.com/influxdb/v3.0/query-data/sql/) 