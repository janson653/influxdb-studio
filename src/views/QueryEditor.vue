<template>
  <div class="query-editor">
    <el-container>
      <el-header class="header">
        <div class="header-left">
          <h2>查询编辑器</h2>
          <div v-if="activeConnection" class="connection-info">
            <el-tag type="success">
              {{ activeConnection.name }}
            </el-tag>
            <el-tag :type="getVersionTagType(activeConnection.version)" size="small">
              {{ activeConnection.version }}
            </el-tag>
          </div>
        </div>
        <div class="header-right">
          <el-select 
            v-model="selectedDatabase" 
            :placeholder="getDatabasePlaceholder()"
            style="width: 150px; margin-right: 10px;"
            @change="handleDatabaseChange"
          >
            <el-option
              v-for="db in databases"
              :key="db"
              :label="db"
              :value="db"
            />
          </el-select>
          <el-button @click="executeQuery" :loading="isExecuting" type="primary">
            <el-icon><VideoPlay /></el-icon>
            执行查询
          </el-button>
        </div>
      </el-header>
      
      <el-main>
        <div v-if="!activeConnection" class="no-connection">
          <el-empty description="请先连接到数据库">
            <el-button type="primary" @click="$router.push('/connection')">
              去连接管理
            </el-button>
          </el-empty>
        </div>
        
        <div v-else class="editor-content">
          <el-row :gutter="20">
            <el-col :span="12">
              <!-- 查询编辑器 -->
              <el-card>
                <template #header>
                  <span>查询编辑器 ({{ getQueryLanguageName() }})</span>
                  <div style="float: right;">
                    <el-button size="small" @click="clearQuery">清空</el-button>
                    <el-button size="small" @click="showQueryHistory">历史</el-button>
                  </div>
                </template>
                
                <div class="editor-container">
                  <MonacoEditor
                    v-model="currentQuery"
                    :language="getQueryLanguage()"
                    :options="editorOptions"
                    @change="handleQueryChange"
                  />
                </div>
                
                <div class="editor-toolbar">
                  <el-button-group>
                    <el-button size="small" @click="insertQuery(getSelectQuery())">
                      SELECT
                    </el-button>
                    <el-button size="small" @click="insertQuery(getShowDatabasesQuery())">
                      {{ getShowDatabasesText() }}
                    </el-button>
                    <el-button size="small" @click="insertQuery(getShowMeasurementsQuery())">
                      {{ getShowMeasurementsText() }}
                    </el-button>
                    <el-button size="small" @click="insertQuery(getShowSeriesQuery())">
                      {{ getShowSeriesText() }}
                    </el-button>
                  </el-button-group>
                </div>
              </el-card>
            </el-col>
            
            <el-col :span="12">
              <!-- 查询结果 -->
              <el-card>
                <template #header>
                  <span>查询结果</span>
                  <div style="float: right;">
                    <el-button 
                      v-if="queryResults" 
                      size="small" 
                      @click="exportResults"
                    >
                      导出
                    </el-button>
                    <el-button 
                      v-if="queryResults" 
                      size="small" 
                      @click="clearResults"
                    >
                      清空
                    </el-button>
                  </div>
                </template>
                
                <div v-if="!queryResults" class="no-results">
                  <el-empty description="暂无查询结果" />
                </div>
                
                <div v-else-if="queryResults.error" class="error-results">
                  <el-alert
                    :title="queryResults.error"
                    type="error"
                    show-icon
                    :closable="false"
                  />
                </div>
                
                <div v-else class="results-content">
                  <div class="results-info">
                    <el-tag type="info">
                      执行时间: {{ queryResults.execution_time }}ms
                    </el-tag>
                    <el-tag type="success">
                      结果数量: {{ getTotalResults() }}
                    </el-tag>
                  </div>
                  
                  <el-tabs v-model="activeResultTab" style="margin-top: 15px;">
                    <el-tab-pane label="表格视图" name="table">
                      <div class="table-container">
                        <el-table 
                          :data="tableData" 
                          style="width: 100%"
                          max-height="400"
                          border
                        >
                          <el-table-column
                            v-for="column in tableColumns"
                            :key="column"
                            :prop="column"
                            :label="column"
                            show-overflow-tooltip
                          />
                        </el-table>
                      </div>
                    </el-tab-pane>
                    
                    <el-tab-pane label="JSON视图" name="json">
                      <div class="json-container">
                        <pre>{{ JSON.stringify(queryResults, null, 2) }}</pre>
                      </div>
                    </el-tab-pane>
                  </el-tabs>
                </div>
              </el-card>
            </el-col>
          </el-row>
        </div>
      </el-main>
    </el-container>
    
    <!-- 查询历史对话框 -->
    <el-dialog v-model="showHistory" title="查询历史" width="800px">
      <el-table :data="recentQueries" style="width: 100%">
        <el-table-column prop="query" label="查询语句" show-overflow-tooltip />
        <el-table-column prop="database" label="数据库" width="120" />
        <el-table-column prop="executionTime" label="执行时间" width="100">
          <template #default="{ row }">
            {{ row.executionTime }}ms
          </template>
        </el-table-column>
        <el-table-column prop="timestamp" label="时间" width="150">
          <template #default="{ row }">
            {{ formatTime(row.timestamp) }}
          </template>
        </el-table-column>
        <el-table-column label="操作" width="100">
          <template #default="{ row }">
            <el-button size="small" @click="loadQuery(row)">
              加载
            </el-button>
          </template>
        </el-table-column>
      </el-table>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useRoute } from 'vue-router'
import { ElMessage } from 'element-plus'
import { VideoPlay } from '@element-plus/icons-vue'
import { invoke } from '@tauri-apps/api/core'
import { useConnectionStore } from '../stores/connectionStore'
import { useQueryStore } from '../stores/queryStore'
import type { QueryResult } from '../stores/queryStore'
import { InfluxDBVersion } from '../types/influxdb'
import MonacoEditor from '../components/Common/MonacoEditor.vue'

// 路由
const route = useRoute()

// 状态管理
const connectionStore = useConnectionStore()
const queryStore = useQueryStore()

// 响应式数据
const currentQuery = ref('SELECT * FROM measurement LIMIT 10')
const selectedDatabase = ref('')
const isExecuting = ref(false)
const queryResults = ref<QueryResult | null>(null)
const activeResultTab = ref('table')
const showHistory = ref(false)

// 数据库列表
const databases = ref<string[]>([])

// Monaco Editor 配置
const editorOptions = {
  theme: 'vs-dark',
  fontSize: 14,
  minimap: { enabled: false },
  scrollBeyondLastLine: false,
  automaticLayout: true,
  wordWrap: 'on' as const
}

// 计算属性
const activeConnection = computed(() => connectionStore.activeConnectionConfig)
const recentQueries = computed(() => queryStore.recentQueries)

const tableData = computed(() => {
  if (!queryResults.value?.series || queryResults.value.series.length === 0) {
    return []
  }
  
  const data: any[] = []
  queryResults.value.series.forEach(series => {
    series.values.forEach(row => {
      const obj: any = {}
      series.columns.forEach((col, index) => {
        obj[col] = row[index]
      })
      data.push(obj)
    })
  })
  
  return data
})

const tableColumns = computed(() => {
  if (!queryResults.value?.series || queryResults.value.series.length === 0) {
    return []
  }
  
  return queryResults.value.series[0]?.columns || []
})

// 方法
const getVersionTagType = (version: string) => {
  switch (version) {
    case InfluxDBVersion.V1: return 'info'
    case InfluxDBVersion.V2: return 'warning'
    case InfluxDBVersion.V3: return 'success'
    default: return 'info'
  }
}

const getQueryLanguage = () => {
  if (!activeConnection.value) return 'sql'
  
  switch (activeConnection.value.version) {
    case InfluxDBVersion.V1: return 'sql'
    case InfluxDBVersion.V2: return 'flux'
    case InfluxDBVersion.V3: return 'sql'
    default: return 'sql'
  }
}

const getQueryLanguageName = () => {
  if (!activeConnection.value) return 'SQL'
  
  switch (activeConnection.value.version) {
    case InfluxDBVersion.V1: return 'InfluxQL'
    case InfluxDBVersion.V2: return 'Flux'
    case InfluxDBVersion.V3: return 'SQL'
    default: return 'SQL'
  }
}

const getDatabasePlaceholder = () => {
  if (!activeConnection.value) return '选择数据库'
  
  switch (activeConnection.value.version) {
    case InfluxDBVersion.V1: return '选择数据库'
    case InfluxDBVersion.V2: return '选择存储桶'
    case InfluxDBVersion.V3: return '选择数据库'
    default: return '选择数据库'
  }
}

const getSelectQuery = () => {
  if (!activeConnection.value) return 'SELECT * FROM measurement LIMIT 10'
  
  switch (activeConnection.value.version) {
    case InfluxDBVersion.V1: return 'SELECT * FROM measurement LIMIT 10'
    case InfluxDBVersion.V2: return 'from(bucket: "mybucket")\n  |> range(start: -1h)\n  |> limit(n: 10)'
    case InfluxDBVersion.V3: return 'SELECT * FROM measurement LIMIT 10'
    default: return 'SELECT * FROM measurement LIMIT 10'
  }
}

const getShowDatabasesQuery = () => {
  if (!activeConnection.value) return 'SHOW DATABASES'
  
  switch (activeConnection.value.version) {
    case InfluxDBVersion.V1: return 'SHOW DATABASES'
    case InfluxDBVersion.V2: return 'buckets()'
    case InfluxDBVersion.V3: return 'SHOW DATABASES'
    default: return 'SHOW DATABASES'
  }
}

const getShowDatabasesText = () => {
  if (!activeConnection.value) return 'SHOW DB'
  
  switch (activeConnection.value.version) {
    case InfluxDBVersion.V1: return 'SHOW DB'
    case InfluxDBVersion.V2: return 'SHOW BUCKETS'
    case InfluxDBVersion.V3: return 'SHOW DB'
    default: return 'SHOW DB'
  }
}

const getShowMeasurementsQuery = () => {
  if (!activeConnection.value) return 'SHOW MEASUREMENTS'
  
  switch (activeConnection.value.version) {
    case InfluxDBVersion.V1: return 'SHOW MEASUREMENTS'
    case InfluxDBVersion.V2: return 'import "influxdata/influxdb/schema"\nschema.measurements(bucket: "mybucket")'
    case InfluxDBVersion.V3: return 'SHOW TABLES'
    default: return 'SHOW MEASUREMENTS'
  }
}

const getShowMeasurementsText = () => {
  if (!activeConnection.value) return 'SHOW MEASUREMENTS'
  
  switch (activeConnection.value.version) {
    case InfluxDBVersion.V1: return 'SHOW MEASUREMENTS'
    case InfluxDBVersion.V2: return 'SHOW MEASUREMENTS'
    case InfluxDBVersion.V3: return 'SHOW TABLES'
    default: return 'SHOW MEASUREMENTS'
  }
}

const getShowSeriesQuery = () => {
  if (!activeConnection.value) return 'SHOW SERIES'
  
  switch (activeConnection.value.version) {
    case InfluxDBVersion.V1: return 'SHOW SERIES'
    case InfluxDBVersion.V2: return 'import "influxdata/influxdb/schema"\nschema.tagKeys(bucket: "mybucket")'
    case InfluxDBVersion.V3: return 'SHOW COLUMNS'
    default: return 'SHOW SERIES'
  }
}

const getShowSeriesText = () => {
  if (!activeConnection.value) return 'SHOW SERIES'
  
  switch (activeConnection.value.version) {
    case InfluxDBVersion.V1: return 'SHOW SERIES'
    case InfluxDBVersion.V2: return 'SHOW TAG KEYS'
    case InfluxDBVersion.V3: return 'SHOW COLUMNS'
    default: return 'SHOW SERIES'
  }
}

const handleDatabaseChange = (database: string) => {
  selectedDatabase.value = database
}

const handleQueryChange = (value: string) => {
  currentQuery.value = value
}

const executeQuery = async () => {
  if (!activeConnection.value || !selectedDatabase.value) {
    ElMessage.warning('请先选择数据库')
    return
  }
  
  if (!currentQuery.value.trim()) {
    ElMessage.warning('请输入查询语句')
    return
  }
  
  // 获取后端连接ID
  const connectionStatus = connectionStore.connectionStatus[activeConnection.value.id]
  const backendConnectionId = connectionStatus?.backendConnectionId
  
  if (!backendConnectionId) {
    ElMessage.error('连接未建立，请先连接到数据库')
    return
  }
  
  try {
    isExecuting.value = true
    const result = await queryStore.executeQuery(
      currentQuery.value,
      selectedDatabase.value,
      backendConnectionId
    )
    
    queryResults.value = result
    ElMessage.success('查询执行成功')
  } catch (error) {
    ElMessage.error(`查询执行失败: ${error}`)
  } finally {
    isExecuting.value = false
  }
}

const clearQuery = () => {
  currentQuery.value = ''
}

const clearResults = () => {
  queryResults.value = null
  queryStore.clearResults()
}

const showQueryHistory = () => {
  showHistory.value = true
}

const loadQuery = (query: any) => {
  currentQuery.value = query.query
  selectedDatabase.value = query.database
  showHistory.value = false
}

const insertQuery = (query: string) => {
  currentQuery.value = query
}

const exportResults = () => {
  if (!queryResults.value) return
  
  try {
    const dataStr = JSON.stringify(queryResults.value, null, 2)
    const dataBlob = new Blob([dataStr], { type: 'application/json' })
    const url = URL.createObjectURL(dataBlob)
    const link = document.createElement('a')
    link.href = url
    link.download = `query-results-${Date.now()}.json`
    link.click()
    URL.revokeObjectURL(url)
    ElMessage.success('结果已导出')
  } catch (error) {
    ElMessage.error('导出失败')
  }
}

const getTotalResults = () => {
  if (!queryResults.value?.series) return 0
  return queryResults.value.series.reduce((total, series) => total + series.values.length, 0)
}

const formatTime = (timestamp: Date) => {
  return timestamp.toLocaleString()
}

const loadDatabases = async () => {
  if (!activeConnection.value) return
  
  const connectionStatus = connectionStore.connectionStatus[activeConnection.value.id]
  const backendConnectionId = connectionStatus?.backendConnectionId
  
  if (!backendConnectionId) return
  
  try {
    const response = await invoke('get_databases', { 
      connectionId: backendConnectionId 
    }) as any
    
    if (response.success && response.data) {
      databases.value = response.data
      // 如果没有选中数据库，默认选择第一个
      if (!selectedDatabase.value && databases.value.length > 0) {
        selectedDatabase.value = databases.value[0]
      }
    }
  } catch (error) {
    console.error('获取数据库列表失败:', error)
  }
}

// 监听路由参数
watch(() => route.query, (query) => {
  if (query.database) {
    selectedDatabase.value = query.database as string
  }
  if (query.measurement) {
    currentQuery.value = `SELECT * FROM ${query.measurement} LIMIT 10`
  }
}, { immediate: true })

// 监听连接状态变化
watch(() => activeConnection.value, (connection) => {
  if (connection) {
    loadDatabases()
    // 根据版本设置默认查询
    currentQuery.value = getSelectQuery()
  } else {
    databases.value = []
    selectedDatabase.value = ''
  }
}, { immediate: true })

// 监听版本变化，更新查询语言
watch(() => activeConnection.value?.version, (version) => {
  if (version) {
    // 重新设置默认查询
    currentQuery.value = getSelectQuery()
  }
})
</script>

<style scoped>
.query-editor {
  height: 100vh;
  background-color: #f5f5f5;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  background-color: white;
  border-bottom: 1px solid #e4e7ed;
  padding: 0 20px;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 10px;
}

.header-left h2 {
  margin: 0;
  color: #303133;
}

.connection-info {
  display: flex;
  align-items: center;
  gap: 5px;
}

.header-right {
  display: flex;
  align-items: center;
}

.no-connection {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 400px;
}

.editor-content {
  padding: 20px;
}

.editor-container {
  height: 300px;
  border: 1px solid #dcdfe6;
  border-radius: 4px;
}

.editor-toolbar {
  margin-top: 10px;
  padding: 10px 0;
  border-top: 1px solid #f0f0f0;
}

.no-results {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 200px;
}

.error-results {
  padding: 20px;
}

.results-info {
  display: flex;
  gap: 10px;
  margin-bottom: 15px;
}

.table-container {
  max-height: 400px;
  overflow: auto;
}

.json-container {
  max-height: 400px;
  overflow: auto;
  background-color: #f5f5f5;
  padding: 15px;
  border-radius: 4px;
}

.json-container pre {
  margin: 0;
  font-family: 'Courier New', monospace;
  font-size: 12px;
  line-height: 1.4;
}
</style> 