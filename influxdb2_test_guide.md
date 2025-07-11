# InfluxDB 2.x 测试指南

## 🔧 修复内容

### 已解决的问题
1. **编译错误** - 修复了 Rust 借用检查器错误
2. **API 400 错误** - 修复了 InfluxDB 2.x API 调用格式

### 关键改进
- ✅ 使用 URL 参数传递 `org`：`/api/v2/query?org=myorg`
- ✅ 使用正确的 Content-Type：`application/vnd.flux`
- ✅ 直接发送 Flux 查询字符串，不使用 JSON 包装
- ✅ 添加详细的调试日志

## 🧪 测试步骤

### 1. 确保 InfluxDB 2.x 运行
```bash
# 启动 InfluxDB 容器
docker-compose up -d influxdb

# 验证服务状态
curl http://localhost:8086/ping
```

### 2. 创建正确的连接配置
在应用中创建新连接，使用以下配置：

**基本信息**：
- 连接名称：`InfluxDB 2.x 本地`
- 主机地址：`localhost`
- 端口：`8086`
- SSL/TLS：`禁用`

**InfluxDB 2.x 配置**：
- Token：`my-super-secret-auth-token`
- 组织(Org)：`myorg`
- 存储桶(Bucket)：`mybucket`

**旧版本字段**（留空）：
- 用户名：留空
- 密码：留空
- 默认数据库：留空

### 3. 测试连接
1. 点击"连接"按钮
2. 应该显示"连接成功"
3. 查看应用日志中的连接信息

### 4. 测试查询
1. 转到查询编辑器
2. 执行以下查询：
   - `SHOW DATABASES`（显示存储桶）
   - `SHOW MEASUREMENTS`（显示测量值）
   - `SELECT * FROM measurement LIMIT 10`（查询数据）

### 5. 查看调试日志
应用日志应该显示：
```
INFO influxdb_studio::influxdb: InfluxDB version: 2.x
INFO influxdb_studio::influxdb: Org: myorg, Token: ***
INFO influxdb_studio::influxdb: Request URL: http://localhost:8086/api/v2/query?org=myorg
INFO influxdb_studio::influxdb: Flux query: buckets() |> rename(columns: {name: "name"}) |> keep(columns: ["name"])
INFO influxdb_studio::influxdb: Response: [CSV数据]
```

## 🔍 故障排除

### 常见错误及解决方案

#### 1. Token 错误
**错误**：`HTTP 401 Unauthorized`
**解决**：检查 Token 是否正确，应该是 `my-super-secret-auth-token`

#### 2. 组织不存在
**错误**：`HTTP 404 Not Found: organization not found`
**解决**：确保 Org 字段填写的是 `myorg`

#### 3. 存储桶不存在
**错误**：`bucket not found`
**解决**：确保 Bucket 字段填写的是 `mybucket`

#### 4. Flux 查询语法错误
**错误**：`HTTP 400 Bad Request: invalid flux query`
**解决**：查看日志中的 Flux 查询，检查语法是否正确

### 调试命令

```bash
# 测试 Token 认证
curl -H "Authorization: Token my-super-secret-auth-token" \
  http://localhost:8086/api/v2/buckets

# 测试组织
curl -H "Authorization: Token my-super-secret-auth-token" \
  http://localhost:8086/api/v2/orgs

# 测试查询
curl -X POST \
  -H "Authorization: Token my-super-secret-auth-token" \
  -H "Content-Type: application/vnd.flux" \
  -d "buckets()" \
  "http://localhost:8086/api/v2/query?org=myorg"
```

## 🎯 预期结果

修复后应该能够：
- ✅ 成功连接到 InfluxDB 2.x
- ✅ 不再出现 400 Bad Request 错误
- ✅ 查看存储桶列表
- ✅ 执行 Flux 查询
- ✅ 正常显示查询结果

## 📋 查询转换示例

### SQL → Flux 转换
```sql
-- 原始 SQL
SHOW DATABASES

-- 转换为 Flux
buckets() |> rename(columns: {name: "name"}) |> keep(columns: ["name"])
```

```sql
-- 原始 SQL
SELECT * FROM cpu_usage LIMIT 10

-- 转换为 Flux
from(bucket: "mybucket")
  |> range(start: -1h)
  |> filter(fn: (r) => r._measurement == "cpu_usage")
  |> limit(n: 10)
```

如果仍有问题，请查看应用日志中的详细错误信息和调试输出。 