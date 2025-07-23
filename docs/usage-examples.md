# InfluxDB Studio 使用示例

本文档提供了 InfluxDB Studio 的详细使用示例，帮助用户快速上手各种功能。

## 🚀 快速开始

### 1. 启动测试环境

```bash
# 使用快速启动脚本
./scripts/quick-start.sh

# 或手动启动 Docker 环境
docker-compose up influxdb-v1 -d
```

### 2. 连接配置

- **主机**: localhost
- **端口**: 8086
- **数据库**: testdb
- **用户名**: (留空)
- **密码**: (留空)

## 📊 数据库操作示例

### 查看数据库信息

```sql
-- 查看所有数据库
SHOW DATABASES

-- 查看当前数据库的测量值
SHOW MEASUREMENTS

-- 查看测量值的字段
SHOW FIELD KEYS FROM "cpu_usage"

-- 查看测量值的标签
SHOW TAG KEYS FROM "cpu_usage"

-- 查看标签值
SHOW TAG VALUES FROM "cpu_usage" WITH KEY = "host"
```

### 基本查询操作

```sql
-- 查询所有数据
SELECT * FROM "cpu_usage" LIMIT 10

-- 查询特定时间范围的数据
SELECT * FROM "cpu_usage" 
WHERE time > now() - 1h

-- 查询特定标签的数据
SELECT * FROM "cpu_usage" 
WHERE host = 'server01'

-- 查询多个测量值
SELECT * FROM "cpu_usage", "memory_usage" 
WHERE time > now() - 30m
```

### 聚合查询

```sql
-- 计算平均值
SELECT mean(value) FROM "cpu_usage" 
WHERE time > now() - 1h

-- 按时间分组聚合
SELECT mean(value) FROM "cpu_usage" 
WHERE time > now() - 1h 
GROUP BY time(5m)

-- 按标签分组聚合
SELECT mean(value) FROM "cpu_usage" 
WHERE time > now() - 1h 
GROUP BY host

-- 多字段聚合
SELECT mean(value), max(value), min(value) 
FROM "cpu_usage" 
WHERE time > now() - 1h 
GROUP BY time(5m), host
```

### 复杂查询示例

```sql
-- 计算 CPU 使用率趋势
SELECT mean(value) as cpu_avg, 
       stddev(value) as cpu_stddev
FROM "cpu_usage" 
WHERE time > now() - 24h 
GROUP BY time(1h), host

-- 内存使用率分析
SELECT mean(value) as memory_avg,
       percentile(value, 95) as memory_p95
FROM "memory_usage" 
WHERE time > now() - 7d 
GROUP BY time(1h), host

-- 网络流量统计
SELECT sum(bytes_in) as total_in,
       sum(bytes_out) as total_out
FROM "network_traffic" 
WHERE time > now() - 1h 
GROUP BY time(5m), host, interface
```

## 🔧 数据管理操作

### 创建数据库

```sql
-- 创建新数据库
CREATE DATABASE my_database

-- 创建保留策略
CREATE RETENTION POLICY "one_week" ON "my_database" 
DURATION 7d REPLICATION 1 DEFAULT
```

### 插入数据

```sql
-- 插入单条数据
INSERT cpu_usage,host=server03,region=us-central value=0.75

-- 插入多条数据
INSERT cpu_usage,host=server03,region=us-central value=0.75 1640995200000000000
INSERT cpu_usage,host=server03,region=us-central value=0.76 1640995260000000000
INSERT cpu_usage,host=server03,region=us-central value=0.77 1640995320000000000

-- 插入多字段数据
INSERT network_traffic,host=server03,interface=eth0 
bytes_in=1024,bytes_out=2048,packets_in=100,packets_out=200
```

### 删除数据

```sql
-- 删除特定时间范围的数据
DELETE FROM "cpu_usage" 
WHERE time < now() - 30d

-- 删除特定标签的数据
DELETE FROM "cpu_usage" 
WHERE host = 'old_server'
```

## 📈 数据分析示例

### 性能监控查询

```sql
-- CPU 使用率趋势分析
SELECT mean(value) as cpu_avg,
       max(value) as cpu_max,
       min(value) as cpu_min
FROM "cpu_usage" 
WHERE time > now() - 24h 
GROUP BY time(1h), host
ORDER BY time DESC

-- 内存使用率异常检测
SELECT mean(value) as memory_avg,
       stddev(value) as memory_stddev,
       (mean(value) + 2 * stddev(value)) as upper_threshold
FROM "memory_usage" 
WHERE time > now() - 7d 
GROUP BY time(1h), host
HAVING mean(value) > 80
```

### 容量规划查询

```sql
-- 存储空间使用情况
SELECT count(value) as data_points,
       sum(value) as total_value
FROM "cpu_usage" 
WHERE time > now() - 30d 
GROUP BY host

-- 数据增长趋势
SELECT count(value) as daily_points
FROM "cpu_usage" 
WHERE time > now() - 30d 
GROUP BY time(1d), host
```

### 告警查询

```sql
-- 高 CPU 使用率告警
SELECT host, value, time
FROM "cpu_usage" 
WHERE value > 0.9 
  AND time > now() - 5m
ORDER BY time DESC

-- 内存使用率告警
SELECT host, value, time
FROM "memory_usage" 
WHERE value > 90 
  AND time > now() - 5m
ORDER BY time DESC
```

## 🎯 最佳实践

### 查询优化

1. **使用时间范围过滤**
   ```sql
   -- 好的做法
   SELECT * FROM "cpu_usage" WHERE time > now() - 1h
   
   -- 避免的做法
   SELECT * FROM "cpu_usage"
   ```

2. **合理使用 LIMIT**
   ```sql
   -- 限制返回结果数量
   SELECT * FROM "cpu_usage" LIMIT 1000
   ```

3. **使用适当的聚合函数**
   ```sql
   -- 对于大量数据使用聚合
   SELECT mean(value) FROM "cpu_usage" 
   WHERE time > now() - 1h 
   GROUP BY time(5m)
   ```

### 数据建模

1. **合理设计标签**
   ```sql
   -- 好的标签设计
   INSERT metrics,service=api,version=v1,region=us-west value=42
   
   -- 避免过多标签值
   INSERT metrics,user_id=12345,request_id=abc123 value=42
   ```

2. **使用有意义的测量值名称**
   ```sql
   -- 清晰的命名
   INSERT cpu_usage,host=server01 value=0.75
   INSERT memory_usage,host=server01 value=85.2
   
   -- 避免模糊命名
   INSERT data,type=cpu,host=server01 value=0.75
   ```

## 🔍 故障排除

### 常见查询问题

1. **查询返回空结果**
   - 检查时间范围是否正确
   - 验证测量值名称是否存在
   - 确认标签值是否正确

2. **查询执行缓慢**
   - 添加时间范围过滤
   - 使用聚合函数减少数据量
   - 检查索引是否合理

3. **内存不足错误**
   - 减少查询时间范围
   - 使用 LIMIT 限制结果数量
   - 分批查询大量数据

### 连接问题

1. **连接失败**
   - 检查 InfluxDB 服务状态
   - 验证主机地址和端口
   - 确认防火墙设置

2. **认证失败**
   - 检查用户名和密码
   - 确认用户权限
   - 验证数据库名称

## 📚 更多资源

- [InfluxDB 官方文档](https://docs.influxdata.com/)
- [InfluxQL 语法参考](https://docs.influxdata.com/influxdb/v1.8/query_language/)
- [InfluxDB Studio 项目主页](https://github.com/your-username/influxdb-studio)

---

**提示**: 这些示例基于测试环境中的数据。在实际使用中，请根据您的数据结构和业务需求调整查询语句。 