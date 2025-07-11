# InfluxDB 2.x 配置指南

## 🔧 根据你的 Docker Compose 配置

从你的 `docker-compose.yml` 文件中，我们可以看到：

```yaml
environment:
  - DOCKER_INFLUXDB_INIT_USERNAME=admin
  - DOCKER_INFLUXDB_INIT_PASSWORD=password123
  - DOCKER_INFLUXDB_INIT_ORG=myorg
  - DOCKER_INFLUXDB_INIT_BUCKET=mybucket
  - DOCKER_INFLUXDB_INIT_ADMIN_TOKEN=my-super-secret-auth-token
```

## 📋 正确的连接配置

在应用中创建连接时，请使用以下配置：

### 基本信息
- **连接名称**: InfluxDB 2.x 本地
- **主机地址**: localhost
- **端口**: 8086
- **SSL/TLS**: 禁用

### InfluxDB 2.x 配置（重要！）
- **Token**: `my-super-secret-auth-token`
- **组织(Org)**: `myorg`
- **存储桶(Bucket)**: `mybucket`

### 旧版本字段（可以留空）
- **用户名**: 留空或填 `admin`
- **密码**: 留空或填 `password123`
- **默认数据库**: 留空

## 🚀 测试步骤

1. **启动 InfluxDB**
   ```bash
   docker-compose up -d influxdb
   ```

2. **验证服务运行**
   ```bash
   curl http://localhost:8086/ping
   ```

3. **在应用中创建连接**
   - 使用上述配置创建新连接
   - 点击"连接"按钮
   - 应该显示"连接成功"

4. **测试查询**
   - 转到查询编辑器
   - 执行查询：`SHOW DATABASES`（会自动转换为 Flux 查询）
   - 应该看到存储桶列表

## 🔍 关键差异

### InfluxDB 1.x vs 2.x
| 项目 | 1.x | 2.x |
|------|-----|-----|
| **认证** | 用户名/密码 | Token |
| **查询语言** | SQL-like | Flux |
| **数据组织** | Database/Measurement | Org/Bucket/Measurement |
| **API端点** | `/query` | `/api/v2/query` |

### 查询转换示例
```sql
-- 1.x SQL
SELECT * FROM cpu_usage LIMIT 10

-- 2.x Flux (自动转换)
from(bucket: "mybucket")
  |> range(start: -1h)
  |> filter(fn: (r) => r._measurement == "cpu_usage")
  |> limit(n: 10)
```

## 🛠️ 故障排除

### 常见错误
1. **401 Unauthorized**: Token 错误或缺失
2. **404 Not Found**: 组织或存储桶不存在
3. **400 Bad Request**: Flux 查询语法错误

### 调试命令
```bash
# 测试 Token 认证
curl -H "Authorization: Token my-super-secret-auth-token" \
  http://localhost:8086/api/v2/buckets

# 查看组织
curl -H "Authorization: Token my-super-secret-auth-token" \
  http://localhost:8086/api/v2/orgs
```

## 🎯 预期结果

配置正确后，你应该能够：
- ✅ 成功连接到 InfluxDB 2.x
- ✅ 查看存储桶列表
- ✅ 执行基本查询
- ✅ 查看查询结果

如果仍有问题，请检查应用日志中的详细错误信息。 