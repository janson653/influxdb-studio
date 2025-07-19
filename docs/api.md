# API 文档

## 概述

InfluxDB Studio 采用前后端分离架构，前端使用 Vue 3 + TypeScript，后端使用 Rust + Tauri。前后端通过 Tauri 的 IPC (Inter-Process Communication) 机制进行通信。

## 通信机制

### Tauri IPC

前端通过 `@tauri-apps/api` 的 `invoke` 函数调用后端注册的 Rust 命令：

```typescript
import { invoke } from '@tauri-apps/api/core'

// 调用后端命令
const result = await invoke('command_name', { 
  param1: 'value1',
  param2: 'value2' 
})
```

### 错误处理

所有 API 调用都返回统一的响应格式：

```typescript
interface ApiResponse<T> {
  success: boolean;
  data?: T;
  error?: string;
}
```

## 连接管理 API

### 连接配置

#### ConnectionProfile

```typescript
interface ConnectionProfile {
  id: string;
  name: string;
  version: 'v1' | 'v2' | 'v3';
  config: InfluxDBV1Config | InfluxDBV2Config | InfluxDBV3Config;
  createdAt: string;
  updatedAt: string;
}
```

#### InfluxDBV1Config

```typescript
interface InfluxDBV1Config {
  host: string;
  port: number;
  username?: string;
  password?: string;
  database: string;
  ssl: boolean;
}
```

#### InfluxDBV2Config

```typescript
interface InfluxDBV2Config {
  host: string;
  port: number;
  token: string;
  organization: string;
  ssl: boolean;
}
```

#### InfluxDBV3Config

```typescript
interface InfluxDBV3Config {
  host: string;
  port: number;
  token: string;
  database: string;
  ssl: boolean;
}
```

### 连接管理命令

#### connect_to_database

连接到 InfluxDB 数据库。

**参数**:
```typescript
{
  profile: ConnectionProfile
}
```

**返回**:
```typescript
{
  success: boolean;
  data?: {
    connectionId: string;
    message: string;
  };
  error?: string;
}
```

**示例**:
```typescript
const result = await invoke('connect_to_database', {
  profile: {
    id: 'conn-1',
    name: 'My InfluxDB',
    version: 'v2',
    config: {
      host: 'localhost',
      port: 8086,
      token: 'my-token',
      organization: 'my-org',
      ssl: false
    }
  }
});
```

#### test_connection

测试 InfluxDB 连接。

**参数**:
```typescript
{
  profile: ConnectionProfile
}
```

**返回**:
```typescript
{
  success: boolean;
  data?: {
    message: string;
    version: string;
  };
  error?: string;
}
```

#### disconnect_from_database

断开 InfluxDB 连接。

**参数**:
```typescript
{
  connectionId: string
}
```

**返回**:
```typescript
{
  success: boolean;
  data?: {
    message: string;
  };
  error?: string;
}
```

## 查询 API

### 查询执行

#### execute_query

执行 InfluxDB 查询。

**参数**:
```typescript
{
  connectionId: string;
  query: string;
  database?: string; // v1 版本需要
}
```

**返回**:
```typescript
{
  success: boolean;
  data?: QueryResult;
  error?: string;
}
```

#### QueryResult

```typescript
interface QueryResult {
  series: Series[];
  error?: string;
  executionTime: number;
}

interface Series {
  name: string;
  columns: string[];
  values: any[][];
  tags?: Record<string, string>;
}
```

**示例**:
```typescript
const result = await invoke('execute_query', {
  connectionId: 'conn-1',
  query: 'SELECT * FROM cpu_usage LIMIT 10',
  database: 'telegraf' // v1 版本需要
});
```

### 元数据查询

#### get_databases

获取数据库列表（v1 版本）。

**参数**:
```typescript
{
  connectionId: string;
}
```

**返回**:
```typescript
{
  success: boolean;
  data?: DatabaseInfo[];
  error?: string;
}
```

#### get_buckets

获取 Bucket 列表（v2/v3 版本）。

**参数**:
```typescript
{
  connectionId: string;
}
```

**返回**:
```typescript
{
  success: boolean;
  data?: BucketInfo[];
  error?: string;
}
```

#### get_measurements

获取测量列表。

**参数**:
```typescript
{
  connectionId: string;
  database?: string; // v1 版本需要
}
```

**返回**:
```typescript
{
  success: boolean;
  data?: MeasurementInfo[];
  error?: string;
}
```

#### get_tag_keys

获取标签键列表。

**参数**:
```typescript
{
  connectionId: string;
  database?: string; // v1 版本需要
  measurement?: string;
}
```

**返回**:
```typescript
{
  success: boolean;
  data?: string[];
  error?: string;
}
```

#### get_tag_values

获取标签值列表。

**参数**:
```typescript
{
  connectionId: string;
  database?: string; // v1 版本需要
  measurement?: string;
  tagKey: string;
}
```

**返回**:
```typescript
{
  success: boolean;
  data?: string[];
  error?: string;
}
```

#### get_field_keys

获取字段键列表。

**参数**:
```typescript
{
  connectionId: string;
  database?: string; // v1 版本需要
  measurement?: string;
}
```

**返回**:
```typescript
{
  success: boolean;
  data?: FieldInfo[];
  error?: string;
}
```

## 数据模型

### 基础数据结构

#### DatabaseInfo

```typescript
interface DatabaseInfo {
  name: string;
  retentionPolicies?: RetentionPolicy[];
}
```

#### BucketInfo

```typescript
interface BucketInfo {
  name: string;
  id: string;
  retentionRules?: RetentionRule[];
}
```

#### MeasurementInfo

```typescript
interface MeasurementInfo {
  name: string;
  tags?: Record<string, string>;
}
```

#### FieldInfo

```typescript
interface FieldInfo {
  name: string;
  type: 'float' | 'integer' | 'string' | 'boolean';
}
```

#### RetentionPolicy

```typescript
interface RetentionPolicy {
  name: string;
  duration: string;
  replication: number;
  default: boolean;
}
```

#### RetentionRule

```typescript
interface RetentionRule {
  type: 'expire';
  everySeconds: number;
}
```

## 错误处理

### 错误类型

```typescript
enum ErrorType {
  CONNECTION_FAILED = 'connection_failed',
  AUTHENTICATION_FAILED = 'authentication_failed',
  QUERY_FAILED = 'query_failed',
  INVALID_PARAMETER = 'invalid_parameter',
  NETWORK_ERROR = 'network_error',
  TIMEOUT = 'timeout',
  UNKNOWN = 'unknown'
}
```

### 错误响应格式

```typescript
interface ApiError {
  type: ErrorType;
  message: string;
  details?: any;
  code?: number;
}
```

### 错误处理示例

```typescript
try {
  const result = await invoke('execute_query', {
    connectionId: 'conn-1',
    query: 'SELECT * FROM cpu_usage'
  });
  
  if (result.success) {
    // 处理成功响应
    console.log('查询结果:', result.data);
  } else {
    // 处理错误
    console.error('查询失败:', result.error);
  }
} catch (error) {
  // 处理异常
  console.error('API 调用异常:', error);
}
```

## 状态管理

### 连接状态

```typescript
interface ConnectionState {
  connections: ConnectionProfile[];
  activeConnectionId: string | null;
  connectionStatus: Record<string, 'connected' | 'disconnected' | 'error'>;
}
```

### 查询状态

```typescript
interface QueryState {
  queryResults: QueryResult[];
  queryHistory: QueryHistoryItem[];
  isLoading: boolean;
  error: string | null;
}

interface QueryHistoryItem {
  id: string;
  query: string;
  timestamp: string;
  executionTime: number;
  success: boolean;
}
```

## 事件系统

### 前端事件

前端可以通过 `@tauri-apps/api/event` 监听后端事件：

```typescript
import { listen } from '@tauri-apps/api/event'

// 监听连接状态变化
await listen('connection-status-changed', (event) => {
  console.log('连接状态变化:', event.payload);
});

// 监听查询进度
await listen('query-progress', (event) => {
  console.log('查询进度:', event.payload);
});
```

### 后端事件

后端可以通过 `emit` 发送事件到前端：

```rust
// 发送连接状态变化事件
app.emit("connection-status-changed", &status).unwrap();

// 发送查询进度事件
app.emit("query-progress", &progress).unwrap();
```

## 性能优化

### 查询优化

1. **分页查询**: 对于大数据集，使用 LIMIT 和 OFFSET
2. **时间范围**: 使用时间范围过滤减少数据量
3. **字段选择**: 只选择需要的字段
4. **索引优化**: 确保查询使用了适当的索引

### 连接池

后端维护连接池以提高性能：

```rust
// 连接池配置
pub struct ConnectionPool {
    connections: HashMap<String, InfluxDBService>,
    max_connections: usize,
}
```

### 缓存策略

1. **元数据缓存**: 缓存数据库、测量等元数据
2. **查询结果缓存**: 缓存常用查询结果
3. **连接缓存**: 复用数据库连接

## 安全考虑

### 认证信息

1. **本地存储**: 连接凭据加密存储在本地
2. **内存安全**: 敏感信息不在日志中输出
3. **传输安全**: 使用 HTTPS 进行网络通信

### 输入验证

1. **SQL 注入防护**: 验证和清理查询输入
2. **参数验证**: 验证所有 API 参数
3. **权限检查**: 检查用户权限

### 错误信息

1. **信息脱敏**: 不在错误信息中暴露敏感数据
2. **日志安全**: 安全地记录错误日志
3. **用户友好**: 提供用户友好的错误提示 