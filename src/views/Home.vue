<template>
  <div class="home-container">
    <IDELayout>
      <!-- 数据库浏览器插槽 -->
      <template #database-tree>
        <DatabaseExplorer
          :selected-database="selectedDatabase"
          :selected-measurement="selectedMeasurement"
          @database-selected="handleDatabaseSelected"
          @measurement-selected="handleMeasurementSelected"
          @query-requested="handleQueryRequested"
        />
      </template>
      
      <!-- 内容区域插槽 -->
      <template #content>
        <div class="content-layout">
          <!-- SQL编辑器 -->
          <div class="editor-section">
            <SqlEditor
              :selected-database="selectedDatabase"
              :selected-measurement="selectedMeasurement"
              @query-executed="handleQueryExecuted"
            />
          </div>
          
          <!-- 查询结果 -->
          <div class="results-section">
            <QueryResults
              :results="queryResults"
              :is-loading="isExecuting"
              :output-logs="outputLogs"
              @refresh="handleRefreshResults"
              @export="handleExportResults"
            />
          </div>
        </div>
      </template>
    </IDELayout>
    
    <!-- 连接管理对话框 -->
    <ConnectionManager
      v-model="showConnectionManager"
      @connection-selected="handleConnectionSelected"
    />
    
    <!-- 连接状态指示器 -->
    <div 
      v-if="!isConnected" 
      class="connection-prompt"
    >
      <div class="prompt-content">
        <div class="prompt-icon">🔌</div>
        <div class="prompt-text">
          <h3>未连接到数据库</h3>
          <p v-if="connectionStore.connections.length === 0">
            欢迎使用 InfluxDB Studio！请创建您的第一个数据库连接
          </p>
          <p v-else>
            请选择要连接的数据库 ({{ connectionStore.connections.length }} 个保存的连接)
          </p>
        </div>
        <div class="prompt-actions">
          <button class="ide-btn ide-btn-primary" @click="showConnectionManager = true">
            {{ connectionStore.connections.length === 0 ? '创建连接' : '选择连接' }}
          </button>
          <button v-if="connectionStore.connections.length > 0" class="ide-btn" @click="handleQuickConnect">
            快速连接
          </button>
        </div>
        <div class="prompt-help">
          <p><strong>快速开始：</strong></p>
          <ul>
            <li>确保 InfluxDB 服务正在运行</li>
            <li>准备好数据库地址、端口和认证信息</li>
            <li>点击"创建连接"或"选择连接"开始使用</li>
          </ul>
          
          <!-- 配色测试 -->
          <details class="color-test">
            <summary>配色对比度测试</summary>
            <div class="color-samples">
              <div class="color-sample">
                <span class="color-label">主要文字</span>
                <span class="color-text primary">这是主要文字颜色</span>
              </div>
              <div class="color-sample">
                <span class="color-label">次要文字</span>
                <span class="color-text secondary">这是次要文字颜色</span>
              </div>
              <div class="color-sample">
                <span class="color-label">占位符文字</span>
                <span class="color-text placeholder">这是占位符文字颜色</span>
              </div>
              <div class="color-sample">
                <span class="color-label">按钮文字</span>
                <button class="ide-btn ide-btn-primary">主要按钮</button>
                <button class="ide-btn">普通按钮</button>
              </div>
              <div class="color-sample">
                <span class="color-label">搜索框</span>
                <input class="ide-input" placeholder="搜索数据库或表..." style="width: 200px;" />
              </div>
              <div class="color-sample">
                <span class="color-label">数据库项</span>
                <div class="mock-database-item">
                  <span class="mock-icon">📊</span>
                  <span class="mock-name">test_database</span>
                  <span class="mock-count">(5)</span>
                </div>
              </div>
              <div class="color-sample">
                <span class="color-label">测量值项</span>
                <div class="mock-measurement-item">
                  <span class="mock-icon">📈</span>
                  <span class="mock-name">cpu_usage</span>
                </div>
              </div>
            </div>
          </details>
          
          <!-- 调试信息 -->
          <details class="debug-info">
            <summary>调试信息</summary>
            <pre>{{ JSON.stringify(debugInfo, null, 2) }}</pre>
          </details>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { ElMessage } from 'element-plus'
import { useConnectionStore } from '../stores/connectionStore'
import { useDatabaseStore } from '../stores/databaseStore'
import { useQueryStore } from '../stores/queryStore'
import { useMeasurementStore } from '../stores/measurementStore'
import IDELayout from '../components/Layout/IDELayout.vue'
import DatabaseExplorer from '../components/Common/DatabaseExplorer.vue'
import SqlEditor from '../components/Query/SqlEditor.vue'
import QueryResults from '../components/Query/QueryResults.vue'
import ConnectionManager from '../components/Connection/ConnectionManager.vue'

// 状态管理
const connectionStore = useConnectionStore()
const databaseStore = useDatabaseStore()
const queryStore = useQueryStore()
const measurementStore = useMeasurementStore()

// 响应式数据
const selectedDatabase = ref<string | null>(null)
const selectedMeasurement = ref<string | null>(null)
const showConnectionManager = ref(false)
const outputLogs = ref<any[]>([])

// 计算属性
const isConnected = computed(() => {
  // 检查是否有活跃连接且状态为已连接
  const hasActiveConnection = connectionStore.activeConnectionId !== null
  const isConnectionActive = connectionStore.isConnected
  
  console.log('连接状态检查:', {
    hasActiveConnection,
    isConnectionActive,
    activeConnectionId: connectionStore.activeConnectionId,
    connectionStatus: connectionStore.connectionStatus,
    connectionsCount: connectionStore.connections.length
  })
  
  return hasActiveConnection && isConnectionActive
})

// 调试信息
const debugInfo = computed(() => ({
  connectionsCount: connectionStore.connections.length,
  activeConnectionId: connectionStore.activeConnectionId,
  isConnected: connectionStore.isConnected,
  connectionStatus: connectionStore.connectionStatus,
  activeConnectionConfig: connectionStore.activeConnectionConfig
}))
const isExecuting = computed(() => queryStore.isExecuting)
const queryResults = computed(() => queryStore.queryResults)

// 方法
const handleDatabaseSelected = async (databaseName: string) => {
  selectedDatabase.value = databaseName
  selectedMeasurement.value = null
  console.log('选中数据库:', databaseName)
  
  // 加载数据库详细信息
  try {
    await databaseStore.fetchDatabaseInfo(databaseName)
    addOutputLog('info', `已选择数据库: ${databaseName}`)
  } catch (error: any) {
    addOutputLog('error', `加载数据库信息失败: ${error.message}`)
  }
}

const handleMeasurementSelected = async (databaseName: string, measurementName: string) => {
  selectedDatabase.value = databaseName
  selectedMeasurement.value = measurementName
  console.log('选中测量值:', databaseName, measurementName)
  
  // 加载测量值详细信息
  try {
    await measurementStore.fetchMeasurementInfo(measurementName)
    addOutputLog('info', `已选择测量值: ${measurementName}`)
  } catch (error: any) {
    addOutputLog('error', `加载测量值信息失败: ${error.message}`)
  }
}

const handleQueryRequested = (databaseName: string, measurementName: string) => {
  selectedDatabase.value = databaseName
  selectedMeasurement.value = measurementName
  // 自动生成查询语句
  const query = `SELECT * FROM "${measurementName}" LIMIT 100`
  console.log('请求查询:', query)
  addOutputLog('info', `生成查询: ${query}`)
}

const handleQueryExecuted = async (result: any) => {
  addOutputLog('success', '查询执行成功', {
    executionTime: result.execution_time,
    seriesCount: result.series.length,
    totalRows: result.series.reduce((total: number, series: any) => total + series.values.length, 0)
  })
}

const handleRefreshResults = () => {
  // 重新执行当前查询
  if (queryStore.currentQuery && selectedDatabase.value) {
    queryStore.executeQuery(
      queryStore.currentQuery,
      selectedDatabase.value,
      connectionStore.activeConnectionId || ''
    )
  }
}

const handleExportResults = (data: any) => {
  addOutputLog('info', '查询结果已导出')
}

const handleConnectionSelected = async (connectionId: string) => {
  try {
    const success = await connectionStore.connectTo(connectionId)
    if (success) {
      showConnectionManager.value = false
      await databaseStore.fetchDatabases()
      addOutputLog('success', '数据库连接成功')
    }
  } catch (error: any) {
    addOutputLog('error', `连接失败: ${error.message}`)
  }
}

const handleQuickConnect = async () => {
  if (connectionStore.connections.length === 0) {
    showConnectionManager.value = true
    return
  }
  
  // 尝试连接第一个可用的连接
  const firstConnection = connectionStore.connections[0]
  if (firstConnection) {
    addOutputLog('info', `尝试快速连接到: ${firstConnection.name}`)
    try {
      const success = await connectionStore.connectTo(firstConnection.id)
      if (success) {
        await databaseStore.fetchDatabases()
        addOutputLog('success', `快速连接成功: ${firstConnection.name}`)
      } else {
        addOutputLog('warning', '快速连接失败，请手动选择连接')
        showConnectionManager.value = true
      }
    } catch (error: any) {
      console.error('快速连接失败:', error)
      addOutputLog('error', `快速连接失败: ${error.message || '未知错误'}`)
      showConnectionManager.value = true
    }
  }
}

const addOutputLog = (type: 'info' | 'success' | 'error' | 'warning', message: string, data?: any) => {
  outputLogs.value.unshift({
    type,
    message,
    data,
    timestamp: new Date()
  })
  
  // 限制日志数量
  if (outputLogs.value.length > 100) {
    outputLogs.value = outputLogs.value.slice(0, 100)
  }
}

// 监听连接状态变化
watch(() => connectionStore.isConnected, (connected) => {
  if (connected) {
    addOutputLog('success', '数据库已连接')
    // 自动刷新数据库列表
    databaseStore.fetchDatabases()
  } else {
    addOutputLog('warning', '数据库连接已断开')
    selectedDatabase.value = null
    selectedMeasurement.value = null
  }
})

// 监听查询执行状态
watch(() => queryStore.isExecuting, (executing) => {
  if (executing) {
    addOutputLog('info', '正在执行查询...')
  }
})

// 生命周期
onMounted(async () => {
  console.log('Home组件挂载，开始初始化连接...')
  
  // 加载保存的连接
  connectionStore.loadConnections()
  
  // 检查是否有保存的连接
  if (connectionStore.connections.length === 0) {
    addOutputLog('info', '欢迎使用 InfluxDB Studio，请创建并连接数据库')
    return
  }
  
  // 如果有活跃连接，自动连接
  const activeConnection = connectionStore.activeConnectionConfig
  if (activeConnection) {
    addOutputLog('info', `尝试自动连接到: ${activeConnection.name}`)
    try {
      const success = await connectionStore.connectTo(activeConnection.id)
      if (success) {
        await databaseStore.fetchDatabases()
        addOutputLog('success', `自动连接成功: ${activeConnection.name}`)
      } else {
        addOutputLog('warning', '自动连接失败，请手动连接数据库')
      }
    } catch (error: any) {
      console.error('自动连接失败:', error)
      addOutputLog('error', `自动连接失败: ${error.message || '未知错误'}`)
    }
  } else {
    // 如果没有活跃连接，但有保存的连接，提示用户选择
    addOutputLog('info', `发现 ${connectionStore.connections.length} 个保存的连接，请选择要连接的数据库`)
  }
})
</script>

<style scoped>
.home-container {
  height: 100vh;
  width: 100vw;
  overflow: hidden;
  position: relative;
}

.content-layout {
  display: flex;
  flex-direction: column;
  height: 100%;
}

.editor-section {
  flex: 1;
  min-height: 300px;
  border-bottom: 1px solid var(--ide-border);
}

.results-section {
  flex: 1;
  min-height: 200px;
}

.connection-prompt {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(30, 30, 46, 0.95);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  cursor: pointer;
  transition: all 0.3s ease;
}

.connection-prompt:hover {
  background-color: rgba(30, 30, 46, 0.98);
}

.prompt-content {
  text-align: center;
  color: var(--ide-text-primary);
  padding: 40px;
  border-radius: var(--ide-border-radius-lg);
  background-color: var(--ide-bg-secondary);
  border: 1px solid var(--ide-border);
  max-width: 400px;
  box-shadow: var(--ide-shadow-lg);
}

.prompt-icon {
  font-size: 48px;
  margin-bottom: 20px;
  opacity: 0.8;
}

.prompt-text h3 {
  margin: 0 0 10px 0;
  color: var(--ide-text-primary);
  font-size: var(--ide-font-size-lg);
  font-weight: 600;
}

.prompt-text p {
  margin: 0;
  color: var(--ide-text-secondary);
  font-size: var(--ide-font-size-sm);
  line-height: 1.5;
}

.prompt-actions {
  margin-top: 20px;
  display: flex;
  gap: var(--ide-spacing-sm);
  justify-content: center;
}

.prompt-help {
  margin-top: 20px;
  color: var(--ide-text-tertiary);
  font-size: var(--ide-font-size-xs);
  text-align: left;
}

.prompt-help p {
  margin: 0 0 8px 0;
  font-weight: 600;
}

.prompt-help ul {
  margin: 0;
  padding-left: 20px;
}

.prompt-help li {
  margin-bottom: 4px;
  line-height: 1.4;
}

.debug-info {
  margin-top: 15px;
  border-top: 1px solid var(--ide-border);
  padding-top: 15px;
}

.debug-info summary {
  cursor: pointer;
  color: var(--ide-text-tertiary);
  font-size: 11px;
  margin-bottom: 10px;
  font-weight: 500;
}

.debug-info summary:hover {
  color: var(--ide-text-secondary);
}

.debug-info pre {
  background-color: var(--ide-bg-primary);
  border: 1px solid var(--ide-border);
  border-radius: var(--ide-border-radius);
  padding: 10px;
  font-size: 10px;
  color: var(--ide-text-secondary);
  overflow-x: auto;
  max-height: 200px;
  overflow-y: auto;
  font-family: var(--ide-font-mono);
}

/* 配色测试样式 */
.color-test {
  margin-top: 15px;
  border-top: 1px solid var(--ide-border);
  padding-top: 15px;
}

.color-test summary {
  cursor: pointer;
  color: var(--ide-text-tertiary);
  font-size: 11px;
  margin-bottom: 10px;
  font-weight: 500;
}

.color-test summary:hover {
  color: var(--ide-text-secondary);
}

.color-samples {
  display: flex;
  flex-direction: column;
  gap: 8px;
  margin-top: 10px;
}

.color-sample {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 8px;
  background-color: var(--ide-bg-primary);
  border-radius: var(--ide-border-radius);
  border: 1px solid var(--ide-border);
}

.color-label {
  font-size: 10px;
  color: var(--ide-text-tertiary);
  min-width: 80px;
  font-weight: 500;
}

.color-text {
  font-size: 11px;
  font-weight: 500;
}

.color-text.primary {
  color: var(--ide-text-primary);
}

.color-text.secondary {
  color: var(--ide-text-secondary);
}

.color-text.placeholder {
  color: var(--ide-text-tertiary);
}

/* 模拟数据库项样式 */
.mock-database-item,
.mock-measurement-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 12px;
  background-color: var(--ide-bg-primary);
  border: 1px solid var(--ide-border);
  border-radius: var(--ide-border-radius);
  margin-top: 4px;
}

.mock-icon {
  font-size: 14px;
  width: 18px;
  height: 18px;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: var(--ide-accent-primary);
  border-radius: 4px;
  color: white;
  font-weight: bold;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

.mock-name {
  font-size: var(--ide-font-size-sm);
  color: var(--ide-text-primary);
  font-weight: 600;
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.3);
}

.mock-count {
  font-size: var(--ide-font-size-xs);
  color: var(--ide-text-primary);
  background-color: var(--ide-accent-primary);
  padding: 2px 8px;
  border-radius: 12px;
  font-weight: 600;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
}

/* 响应式设计 */
@media (max-width: 768px) {
  .content-layout {
    flex-direction: column;
  }
  
  .editor-section,
  .results-section {
    min-height: 200px;
  }
  
  .prompt-content {
    padding: 20px;
    margin: 20px;
  }
  
  .prompt-icon {
    font-size: 36px;
  }
}
</style> 