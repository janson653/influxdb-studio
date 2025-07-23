<template>
  <div class="test-page">
    <div class="test-header">
      <h1>InfluxDB Studio - 测试页面</h1>
      <p>当前版本: {{ version }}</p>
    </div>
    
    <div class="test-content">
      <div class="test-section">
        <h2>功能测试</h2>
        <div class="test-buttons">
          <button @click="testConnection" :disabled="isConnecting">
            {{ isConnecting ? '连接中...' : '测试连接' }}
          </button>
          <button @click="testDatabase" :disabled="isLoadingDatabases">
            {{ isLoadingDatabases ? '加载中...' : '测试数据库' }}
          </button>
          <button @click="testQuery" :disabled="isExecutingQuery">
            {{ isExecutingQuery ? '执行中...' : '测试查询' }}
          </button>
        </div>
      </div>
      
      <div class="test-section">
        <h2>状态信息</h2>
        <div class="status-info">
          <div class="status-item">
            <span class="label">连接状态:</span>
            <span :class="['value', connectionStore.isConnected ? 'connected' : 'disconnected']">
              {{ connectionStore.isConnected ? '已连接' : '未连接' }}
            </span>
          </div>
          <div class="status-item">
            <span class="label">活跃连接:</span>
            <span class="value">{{ connectionStore.activeConnectionConfig?.name || '无' }}</span>
          </div>
          <div class="status-item">
            <span class="label">数据库数量:</span>
            <span class="value">{{ databaseStore.databases.length }}</span>
          </div>
          <div class="status-item">
            <span class="label">查询历史:</span>
            <span class="value">{{ queryStore.queryHistory.length }}</span>
          </div>
        </div>
      </div>
      
      <div class="test-section">
        <h2>IDE布局预览</h2>
        <div class="ide-preview">
          <IDELayout>
            <template #database-tree>
              <div class="preview-tree">
                <div class="tree-item">
                  <span class="tree-icon icon-database"></span>
                  <span>测试数据库</span>
                </div>
                <div class="tree-item">
                  <span class="tree-icon icon-table"></span>
                  <span>测试表</span>
                </div>
              </div>
            </template>
            
            <template #content>
              <div class="preview-editor">
                <h3>SQL编辑器预览</h3>
                <textarea 
                  class="sql-editor" 
                  v-model="testQuery"
                  placeholder="输入SQL查询..."
                ></textarea>
              </div>
            </template>
            
            <template #results-table>
              <div class="preview-results">
                <h3>查询结果预览</h3>
                <div class="results-table">
                  <table>
                    <thead>
                      <tr>
                        <th>时间</th>
                        <th>值</th>
                        <th>标签</th>
                      </tr>
                    </thead>
                    <tbody>
                      <tr>
                        <td>2024-01-01 12:00:00</td>
                        <td>42.5</td>
                        <td>temperature</td>
                      </tr>
                      <tr>
                        <td>2024-01-01 12:01:00</td>
                        <td>43.2</td>
                        <td>temperature</td>
                      </tr>
                    </tbody>
                  </table>
                </div>
              </div>
            </template>
          </IDELayout>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { ElMessage } from 'element-plus'
import { useConnectionStore } from '../stores/connectionStore'
import { useDatabaseStore } from '../stores/databaseStore'
import { useQueryStore } from '../stores/queryStore'
import IDELayout from '../components/Layout/IDELayout.vue'

// 版本信息
const version = ref('0.3.10')

// Stores
const connectionStore = useConnectionStore()
const databaseStore = useDatabaseStore()
const queryStore = useQueryStore()

// 响应式数据
const isConnecting = ref(false)
const isLoadingDatabases = ref(false)
const isExecutingQuery = ref(false)
const testQuery = ref('SELECT * FROM "test_measurement" LIMIT 10')

// 测试方法
const testConnection = async () => {
  isConnecting.value = true
  try {
    // 加载保存的连接
    connectionStore.loadConnections()
    
    if (connectionStore.connections.length === 0) {
      ElMessage.info('暂无保存的连接，请先创建连接')
    } else {
      ElMessage.success(`找到 ${connectionStore.connections.length} 个保存的连接`)
    }
  } catch (error) {
    ElMessage.error(`连接测试失败: ${error}`)
  } finally {
    isConnecting.value = false
  }
}

const testDatabase = async () => {
  if (!connectionStore.isConnected) {
    ElMessage.warning('请先连接数据库')
    return
  }
  
  isLoadingDatabases.value = true
  try {
    await databaseStore.fetchDatabases()
    ElMessage.success(`成功加载 ${databaseStore.databases.length} 个数据库`)
  } catch (error) {
    ElMessage.error(`数据库加载失败: ${error}`)
  } finally {
    isLoadingDatabases.value = false
  }
}

const testQuery = async () => {
  if (!connectionStore.isConnected) {
    ElMessage.warning('请先连接数据库')
    return
  }
  
  if (!databaseStore.selectedDatabase) {
    ElMessage.warning('请先选择数据库')
    return
  }
  
  isExecutingQuery.value = true
  try {
    const result = await queryStore.executeQuery(
      testQuery.value,
      databaseStore.selectedDatabase,
      connectionStore.activeConnectionId || ''
    )
    
    if (result) {
      ElMessage.success('查询执行成功')
    }
  } catch (error) {
    ElMessage.error(`查询执行失败: ${error}`)
  } finally {
    isExecutingQuery.value = false
  }
}

// 生命周期
onMounted(() => {
  // 初始化
  connectionStore.loadConnections()
  queryStore.loadQueryHistory()
})
</script>

<style scoped>
.test-page {
  padding: 20px;
  background-color: var(--ide-bg-primary);
  color: var(--ide-text-primary);
  min-height: 100vh;
}

.test-header {
  text-align: center;
  margin-bottom: 30px;
}

.test-header h1 {
  color: var(--ide-accent-primary);
  font-size: 32px;
  margin-bottom: 10px;
}

.test-header p {
  color: var(--ide-text-secondary);
  font-size: 16px;
}

.test-content {
  max-width: 1200px;
  margin: 0 auto;
}

.test-section {
  margin-bottom: 30px;
  padding: 20px;
  background-color: var(--ide-bg-secondary);
  border-radius: 8px;
  border: 1px solid var(--ide-border);
}

.test-section h2 {
  color: var(--ide-accent-green);
  font-size: 24px;
  margin-bottom: 15px;
}

.test-buttons {
  display: flex;
  gap: 10px;
  flex-wrap: wrap;
}

.test-buttons button {
  background-color: var(--ide-accent-primary);
  color: white;
  border: none;
  padding: 10px 20px;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
}

.test-buttons button:hover:not(:disabled) {
  background-color: var(--ide-accent-secondary);
}

.test-buttons button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.status-info {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 15px;
}

.status-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px;
  background-color: var(--ide-bg-tertiary);
  border-radius: 4px;
}

.status-item .label {
  color: var(--ide-text-secondary);
  font-size: 14px;
}

.status-item .value {
  font-weight: 500;
  font-size: 14px;
}

.status-item .value.connected {
  color: var(--ide-accent-primary);
}

.status-item .value.disconnected {
  color: var(--ide-error);
}

.ide-preview {
  border: 2px solid var(--ide-accent-primary);
  border-radius: 8px;
  overflow: hidden;
  height: 600px;
}

.preview-tree {
  padding: 10px;
}

.preview-editor {
  padding: 20px;
  height: 100%;
  display: flex;
  flex-direction: column;
}

.preview-editor h3 {
  margin-bottom: 15px;
  color: var(--ide-text-primary);
}

.sql-editor {
  flex: 1;
  background-color: var(--ide-bg-primary);
  color: var(--ide-text-primary);
  border: 1px solid var(--ide-border);
  padding: 10px;
  font-family: var(--ide-font-mono);
  font-size: 14px;
  line-height: 1.5;
  resize: none;
  outline: none;
}

.preview-results {
  padding: 20px;
  height: 100%;
  display: flex;
  flex-direction: column;
}

.preview-results h3 {
  margin-bottom: 15px;
  color: var(--ide-text-primary);
}

.results-table {
  flex: 1;
  overflow: auto;
}

.results-table table {
  width: 100%;
  border-collapse: collapse;
  font-size: 12px;
}

.results-table th {
  background-color: var(--ide-bg-secondary);
  padding: 8px;
  text-align: left;
  border-bottom: 1px solid var(--ide-border);
  font-weight: normal;
  position: sticky;
  top: 0;
}

.results-table td {
  padding: 6px 8px;
  border-bottom: 1px solid var(--ide-bg-secondary);
}

.results-table tr:hover {
  background-color: var(--ide-bg-secondary);
}

.tree-item {
  padding: 3px 5px;
  cursor: pointer;
  border-radius: 3px;
  display: flex;
  align-items: center;
  gap: 5px;
  font-size: 13px;
}

.tree-item:hover {
  background-color: var(--ide-bg-tertiary);
}

.tree-icon {
  width: 16px;
  height: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.icon-database { background-color: var(--ide-accent-primary); border-radius: 2px; }
.icon-table { background-color: var(--ide-accent-orange); border-radius: 2px; }
</style> 