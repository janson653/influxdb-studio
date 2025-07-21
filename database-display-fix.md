# 数据库侧边栏显示问题修复方案

## 问题分析

基于代码分析，数据库侧边栏显示不出来可能由以下几个原因导致：

### 1. 连接状态问题
- 连接状态不是 `'connected'`
- 缺少 `backendConnectionId`
- 连接配置不完整

### 2. 数据加载问题
- `loadDatabases` 函数提前返回
- 后端API调用失败
- 数据解析错误

### 3. 组件渲染问题
- Vue组件状态不正确
- DOM元素未正确渲染
- 条件渲染逻辑错误

## 修复方案

### 方案1: 增强连接状态检查

在 `MainLayout.vue` 中修改连接状态检查逻辑：

```typescript
// 修改前
const isConnected = computed(() => connectionStatus.value?.status === 'connected');

// 修改后
const isConnected = computed(() => {
  const status = connectionStatus.value;
  return status?.status === 'connected' && status?.backendConnectionId;
});
```

### 方案2: 改进数据加载逻辑

在 `loadDatabases` 函数中添加更多调试信息和错误处理：

```typescript
const loadDatabases = async () => {
  console.log('[DEBUG] loadDatabases called');
  
  const backendId = connectionStatus.value?.backendConnectionId;
  console.log('[DEBUG] backendConnectionId:', backendId);
  
  if (!backendId) {
    console.error('[DEBUG] No backendConnectionId found');
    return;
  }
  
  try {
    console.log('[DEBUG] Calling get_databases with connectionId:', backendId);
    const response = await invoke('get_databases', { connectionId: backendId }) as any;
    console.log('[DEBUG] get_databases response:', response);
    
    if (response.success) {
      const databases = response.data.map((db: string) => ({ 
        id: db, 
        name: db, 
        type: 'database', 
        children: [] 
      }));
      console.log('[DEBUG] Processed databases:', databases);
      databaseTreeData.value = databases;
    } else {
      console.error('[DEBUG] get_databases failed:', response.error);
      databaseTreeData.value = [];
    }
  } catch (error) {
    console.error('[DEBUG] loadDatabases error:', error);
    databaseTreeData.value = [];
  }
};
```

### 方案3: 添加自动重连机制

在连接失败时自动重试：

```typescript
const connectTo = async (id: string) => {
  try {
    const success = await connectionStore.connectTo(id);
    if (success) {
      ElMessage.success('连接成功');
      // 连接成功后立即加载数据库
      await loadDatabases();
    } else {
      ElMessage.error('连接失败');
    }
  } catch (error) {
    ElMessage.error(`连接失败: ${error}`);
  }
};
```

### 方案4: 改进空状态显示

修改空状态的显示逻辑：

```vue
<template>
  <div class="explorer-tree" @contextmenu.prevent="openContextMenu($event, null)">
    <div v-if="!activeConnection" class="empty-state">
      <el-empty description="请先添加连接" :image-size="60" />
    </div>
    <div v-else-if="!isConnected" class="empty-state">
      <el-empty description="请先连接数据库" :image-size="60" />
      <el-button @click="connectTo(activeConnection.id)" size="small">
        连接数据库
      </el-button>
    </div>
    <div v-else-if="isLoading" class="empty-state">
      <el-empty description="正在加载..." :image-size="60" />
    </div>
    <div v-else-if="databaseTreeData.length === 0" class="empty-state">
      <el-empty description="没有找到数据库" :image-size="60" />
    </div>
    <el-tree
      v-else
      :data="databaseTreeData"
      :props="{ children: 'children', label: 'name' }"
      @node-click="handleDatabaseClick"
    >
      <!-- ... -->
    </el-tree>
  </div>
</template>
```

## 调试步骤

### 1. 检查连接状态
在浏览器控制台中运行：
```javascript
// 获取连接store
const store = window.__VUE_APP__.config.globalProperties.$pinia._s.get('connection');
console.log('连接状态:', store.connectionStatus);
console.log('活跃连接:', store.activeConnectionConfig);
```

### 2. 检查数据库数据
```javascript
// 获取MainLayout组件
const mainLayout = document.querySelector('.main-layout').__vueParentComponent;
console.log('数据库树数据:', mainLayout.ctx.databaseTreeData);
```

### 3. 手动触发数据库加载
```javascript
// 手动调用数据库加载
const { invoke } = await import('@tauri-apps/api/core');
const response = await invoke('get_databases', { connectionId: 'your-connection-id' });
console.log('数据库响应:', response);
```

## 预防措施

### 1. 添加更多日志
在关键位置添加调试日志，便于问题排查。

### 2. 改进错误处理
为所有可能的错误情况添加适当的处理逻辑。

### 3. 添加状态指示器
在UI中显示当前的状态，让用户知道发生了什么。

### 4. 添加重试机制
在连接失败时提供重试选项。

## 测试验证

修复后应该验证以下功能：

1. ✅ 连接成功后自动显示数据库列表
2. ✅ 连接失败时显示错误信息
3. ✅ 数据库列表为空时显示空状态
4. ✅ 点击数据库节点能正确加载测量值
5. ✅ 刷新功能正常工作
6. ✅ 错误情况下有适当的用户提示 