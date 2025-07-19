# 数据库浏览器问题调试指南

## 问题描述
数据库浏览器显示"暂无数据库"，即使连接状态显示"已连接"。

## 已添加的调试日志

### 前端日志 (浏览器控制台)
- `[FE] connectToDatabase: 开始连接` - 连接开始
- `[FE] connectToDatabase: 连接结果` - 连接结果
- `[FE] 连接状态变化监听器` - 连接状态变化
- `[FE] loadDatabases: 开始获取数据库列表` - 数据加载开始
- `[FE] loadDatabases: 后端响应` - 后端响应
- `[FE] loadDatabases: 成功获取数据库列表` - 成功获取数据

### 后端日志 (终端输出)
- `[BE] connect_to_database called` - 连接命令调用
- `[BE] Successfully created service` - 服务创建成功
- `[BE] Storing connection in map` - 存储连接
- `[BE] get_databases called` - 获取数据库命令调用
- `[BE] Current connections in map` - 当前连接映射
- `[BE] InfluxDBV1Service::get_databases called` - InfluxDB 服务调用
- `[BE] Executing query: 'SHOW DATABASES'` - 执行查询
- `[BE] Making HTTP request to` - HTTP 请求
- `[BE] HTTP response status` - HTTP 响应状态
- `[BE] Response text` - 响应内容

## 调试步骤

### 1. 检查前端连接状态
1. 打开浏览器开发者工具 (F12)
2. 进入 Console 标签页
3. 在数据库浏览器页面点击"连接数据库"
4. 查看控制台输出的连接日志

### 2. 检查后端连接状态
1. 查看终端输出的后端日志
2. 确认是否调用了 `connect_to_database`
3. 确认是否成功创建了服务
4. 确认连接是否存储在连接映射中

### 3. 检查数据库查询
1. 连接成功后，查看是否调用了 `get_databases`
2. 确认 HTTP 请求的 URL 和参数
3. 查看 InfluxDB 的响应内容
4. 确认响应解析是否正确

### 4. 常见问题排查

#### 问题 1: 连接未建立
**现象**: 前端显示"连接未建立"
**检查点**:
- `backendConnectionId` 是否存在
- 连接状态是否为 `connected`
- 后端连接映射中是否有对应连接

#### 问题 2: HTTP 请求失败
**现象**: 后端日志显示 HTTP 请求失败
**检查点**:
- InfluxDB 服务是否运行
- 连接配置是否正确 (host, port, username, password)
- 网络连接是否正常

#### 问题 3: 查询响应解析错误
**现象**: HTTP 请求成功但解析失败
**检查点**:
- InfluxDB 版本是否支持 `SHOW DATABASES`
- 响应格式是否符合预期
- 用户权限是否足够

#### 问题 4: 数据格式问题
**现象**: 解析成功但数据库列表为空
**检查点**:
- InfluxDB 中是否真的没有数据库
- 查询结果的数据结构是否正确
- 数据库名称是否在正确的列位置

## 预期日志流程

### 正常流程
```
[FE] connectToDatabase: 开始连接
[BE] connect_to_database called
[BE] Successfully created service
[BE] Storing connection in map
[FE] connectToDatabase: 连接结果
[FE] 连接状态变化监听器
[FE] 连接成功，开始加载数据库列表
[FE] loadDatabases: 开始获取数据库列表
[BE] get_databases called
[BE] InfluxDBV1Service::get_databases called
[BE] Executing query: 'SHOW DATABASES'
[BE] Making HTTP request to
[BE] HTTP response status: 200
[BE] Response text: {"results":[{"series":[{"name":"databases","columns":["name"],"values":[["testdb"]]}]}]}
[BE] Query succeeded, parsed 1 series
[BE] get_databases completed, found 1 databases: ["testdb"]
[FE] loadDatabases: 后端响应
[FE] loadDatabases: 成功获取数据库列表
[FE] loadDatabases: 构建的树形数据
```

### 异常流程示例
```
[FE] connectToDatabase: 开始连接
[BE] connect_to_database called
[BE] Failed to create service: Connection refused
[FE] connectToDatabase: 连接结果
[FE] connectToDatabase: 连接异常
```

## 下一步操作

根据日志输出，确定问题所在的具体环节，然后针对性解决：

1. **连接问题**: 检查 InfluxDB 服务状态和连接配置
2. **权限问题**: 检查用户权限和认证信息
3. **查询问题**: 检查 InfluxDB 版本和查询语法
4. **数据问题**: 检查数据库中是否真的存在数据 