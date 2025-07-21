# 故障排除指南

## 生产环境中表不显示问题

### 问题描述
在开发环境中使用 `start_tauri.sh` 脚本启动程序时，点击左侧 sidebar 中的数据库后可以正常显示表，但打包生成的应用程序运行后，点击数据库不显示表。

### 可能的原因

#### 1. 连接状态问题
- **现象**：后端连接 ID 在生产环境中丢失
- **原因**：连接状态管理在生产环境中可能存在问题
- **解决方案**：
  - 检查连接状态是否正确保存
  - 验证 `backendConnectionId` 是否有效
  - 确保连接建立后状态正确同步

#### 2. API 调用失败
- **现象**：`get_measurements` 命令调用失败
- **原因**：生产环境中 Tauri 命令可能无法正确执行
- **解决方案**：
  - 检查后端日志输出
  - 验证命令参数是否正确传递
  - 确认网络连接和权限设置

#### 3. 前端状态同步问题
- **现象**：数据库树数据更新失败
- **原因**：Vue 响应式数据在生产环境中可能存在问题
- **解决方案**：
  - 检查数据绑定是否正确
  - 验证组件生命周期
  - 确保状态更新触发重新渲染

### 诊断步骤

#### 步骤 1：启用调试模式
```bash
# 使用调试脚本启动生产环境应用
./scripts/debug-production.sh
```

#### 步骤 2：检查控制台输出
1. 打开应用后，按 `F12` 打开开发者工具
2. 查看控制台是否有错误信息
3. 检查网络请求是否成功

#### 步骤 3：使用调试面板
1. 在应用中按 `Ctrl+Shift+D` 打开调试面板
2. 查看连接状态和数据库树状态
3. 检查日志信息

#### 步骤 4：验证后端连接
1. 确认数据库连接是否成功建立
2. 检查 `backendConnectionId` 是否存在
3. 验证 `get_databases` 命令是否返回数据

### 常见解决方案

#### 方案 1：重新建立连接
```javascript
// 在前端控制台执行
const connectionStore = useConnectionStore()
await connectionStore.disconnectFrom(activeConnection.id)
await connectionStore.connectTo(activeConnection.id)
```

#### 方案 2：强制刷新数据
```javascript
// 在前端控制台执行
const databaseExplorer = document.querySelector('.database-explorer')
// 触发刷新按钮点击
databaseExplorer.querySelector('.refresh-btn').click()
```

#### 方案 3：检查网络连接
```bash
# 测试数据库连接
curl -i http://your-influxdb-host:8086/ping
```

### 预防措施

#### 1. 增强错误处理
- 在所有 API 调用中添加错误处理
- 提供用户友好的错误提示
- 记录详细的错误日志

#### 2. 状态验证
- 在关键操作前验证连接状态
- 确保数据同步正确
- 添加状态检查机制

#### 3. 测试策略
- 在多个环境中测试应用
- 使用不同的数据库配置
- 模拟网络异常情况

### 调试工具使用

#### 1. 调试面板
- 按 `Ctrl+Shift+D` 打开调试面板
- 查看实时日志和状态信息
- 导出调试信息进行分析

#### 2. 控制台调试
```javascript
// 检查连接状态
console.log('连接状态:', connectionStore.connectionStatus)

// 检查数据库树数据
console.log('数据库树:', databaseTreeData.value)

// 检查活跃连接
console.log('活跃连接:', connectionStore.activeConnectionConfig)
```

#### 3. 后端日志
```bash
# 查看后端日志
RUST_LOG=debug ./src-tauri/target/release/influxdb-studio
```

### 联系支持

如果问题仍然存在，请提供以下信息：

1. **环境信息**：
   - 操作系统版本
   - Tauri 版本
   - InfluxDB 版本

2. **错误日志**：
   - 前端控制台错误
   - 后端日志输出
   - 调试面板信息

3. **复现步骤**：
   - 详细的操作步骤
   - 期望结果和实际结果
   - 问题出现的频率

4. **配置信息**：
   - 数据库连接配置
   - 应用配置参数
   - 网络环境信息 