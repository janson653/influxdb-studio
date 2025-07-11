# InfluxDB 认证问题调试指南

## 🔧 问题症状
```
查询执行失败: Error: 查询错误: HTTP 401 Unauthorized: {"code":"unauthorized","message":"unauthorized access"}
```

## 🔍 问题诊断

### 1. 检查 InfluxDB 服务器配置
你的 InfluxDB 服务器可能启用了认证。检查 InfluxDB 配置文件：

```bash
# 查看 InfluxDB 配置
influxd config

# 或者查看配置文件
cat /etc/influxdb/influxdb.conf
```

关键配置项：
```ini
[http]
  auth-enabled = true  # 如果为 true，则需要认证
```

### 2. 检查连接配置
确保在连接配置中提供了正确的用户名和密码：

- **用户名**：不能为空
- **密码**：不能为空
- **权限**：用户必须有访问指定数据库的权限

### 3. 验证用户权限
连接到 InfluxDB 并检查用户权限：

```bash
# 使用 influx CLI 连接
influx -username <用户名> -password <密码>

# 查看用户权限
SHOW USERS

# 查看数据库权限
SHOW GRANTS FOR <用户名>
```

## 🛠️ 解决方案

### 方案1：提供正确的认证信息
1. 打开连接管理页面
2. 编辑连接配置
3. 填写正确的用户名和密码
4. 保存并重新连接

### 方案2：创建 InfluxDB 用户（如果没有）
```sql
-- 创建管理员用户
CREATE USER "admin" WITH PASSWORD 'password' WITH ALL PRIVILEGES

-- 创建普通用户
CREATE USER "myuser" WITH PASSWORD 'mypassword'

-- 授予数据库权限
GRANT ALL ON "mydatabase" TO "myuser"
```

### 方案3：禁用 InfluxDB 认证（不推荐用于生产环境）
修改 InfluxDB 配置文件：
```ini
[http]
  auth-enabled = false
```
然后重启 InfluxDB 服务。

## 🧪 测试步骤

1. **验证连接配置**
   - 确保用户名和密码不为空
   - 测试连接（可能会成功，因为 ping 不需要认证）

2. **验证查询权限**
   - 尝试执行简单查询：`SHOW DATABASES`
   - 检查是否还有 401 错误

3. **检查日志**
   - 查看应用日志中的认证信息
   - 确认用户名密码被正确传递

## 📋 常见问题

### Q: 连接测试成功但查询失败？
A: `ping` 端点通常不需要认证，但 `query` 端点需要。这是正常的。

### Q: 用户名密码正确但仍然失败？
A: 检查用户是否有访问特定数据库的权限。

### Q: 如何查看传递给 InfluxDB 的参数？
A: 查看应用日志，我们已经添加了调试信息。

## 🔧 调试命令

```bash
# 检查 InfluxDB 是否运行
ps aux | grep influxd

# 检查 InfluxDB 端口
netstat -tlnp | grep 8086

# 测试 HTTP 连接
curl -i http://localhost:8086/ping

# 测试认证查询
curl -i -XPOST 'http://localhost:8086/query' \
  --data-urlencode "q=SHOW DATABASES" \
  --data-urlencode "u=username" \
  --data-urlencode "p=password"
```

## 🎯 预期结果

修复后应该能够：
- ✅ 成功连接到 InfluxDB
- ✅ 执行查询不再返回 401 错误
- ✅ 正常显示数据库列表和查询结果 