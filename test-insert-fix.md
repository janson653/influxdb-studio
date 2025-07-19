# INSERT 语句修复测试指南

## 问题描述
原始错误：`Error: Query error: HTTP 400 Bad Request: {"error":"error parsing query: found INSERT, expected SELECT, DELETE, SHOW, CREATE, DROP, EXPLAIN, GRANT, REVOKE, ALTER, SET, KILL at line 1, char 1"}`

## 修复内容

### 1. 添加了查询语法验证器
- 文件：`src/utils/queryValidator.ts`
- 功能：验证 InfluxQL 和 Flux 查询语法
- 支持：INSERT、SELECT、SHOW、CREATE、DELETE 等语句

### 2. 更新了查询编辑器
- 文件：`src/views/QueryEditor.vue`
- 功能：在执行查询前进行语法验证
- 新增：查询示例按钮

## 测试步骤

### 1. 启动应用
```bash
cd /home/janson/codebase/influxdb-ui
npm run dev
```

### 2. 连接 InfluxDB
1. 打开应用
2. 创建或选择一个 InfluxDB v1.x 连接
3. 确保连接成功

### 3. 测试错误的 INSERT 语句
**原始错误语句：**
```sql
INSERT t111,tag_key=tag_value field_key="field_value"
```

**预期结果：**
- 应用会显示语法错误提示
- 提供修正建议
- 不会发送到后端执行

### 4. 测试正确的 INSERT 语句
**修正后的语句：**
```sql
INSERT INTO "your_database" t111,tag_key=tag_value field_key="field_value"
```

**预期结果：**
- 语法验证通过
- 查询成功执行
- 显示执行结果

### 5. 测试查询示例功能
1. 点击"示例"按钮
2. 查看显示的查询示例
3. 选择一个示例进行测试

## 支持的查询类型

### InfluxDB v1.x (InfluxQL)
```sql
-- 查询数据
SELECT * FROM "measurement" LIMIT 10

-- 插入数据
INSERT INTO "database" cpu,host=server1 value=0.64

-- 查看数据库
SHOW DATABASES

-- 查看测量值
SHOW MEASUREMENTS

-- 创建数据库
CREATE DATABASE "new_database"
```

### InfluxDB v2.x (Flux)
```javascript
// 查询数据
from(bucket: "my-bucket")
  |> range(start: -1h)
  |> limit(n: 10)

// 查看存储桶
buckets()

// 查看测量值
import "influxdata/influxdb/schema"
schema.measurements(bucket: "my-bucket")
```

## 验证要点

### 1. 语法验证
- [ ] 错误的 INSERT 语句被正确识别
- [ ] 提供有用的错误提示
- [ ] 提供修正建议

### 2. 查询执行
- [ ] 正确的 INSERT 语句能正常执行
- [ ] 查询结果正确显示
- [ ] 执行时间正确计算

### 3. 用户体验
- [ ] 错误提示清晰易懂
- [ ] 示例功能正常工作
- [ ] 界面响应流畅

## 常见问题排查

### 1. 端口占用问题
如果遇到端口 1420 被占用：
```bash
# 查找占用端口的进程
lsof -i :1420

# 杀死进程
kill -9 <PID>

# 或者使用不同端口
npm run dev -- --port 1421
```

### 2. 数据库连接问题
- 确保 InfluxDB 服务正在运行
- 检查连接配置是否正确
- 验证数据库名称是否存在

### 3. 权限问题
- 确保用户有执行 INSERT 的权限
- 检查数据库的写入权限

## 预期改进效果

1. **错误预防**：在执行前验证语法，避免无效查询
2. **用户体验**：提供清晰的错误提示和修正建议
3. **学习辅助**：通过示例功能帮助用户学习正确的语法
4. **开发效率**：减少因语法错误导致的调试时间 