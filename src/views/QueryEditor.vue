<template>
  <div class="query-editor">
    <el-container>
      <el-header class="header">
        <div class="header-left">
          <h2>查询编辑器</h2>
          <el-tag v-if="activeConnection" type="success">
            {{ activeConnection.name }}
          </el-tag>
        </div>
        <div class="header-right">
          <el-select 
            v-model="selectedDatabase" 
            placeholder="选择数据库"
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
                  <span>查询编辑器</span>
                  <div style="float: right;">
                    <el-button size="small" @click="clearQuery">清空</el-button>
                    <el-button size="small" @click="showQueryHistory">历史</el-button>
                  </div>
                </template>
                
                <div class="editor-container">
                  <MonacoEditor
                    v-model="currentQuery"
                    language="sql"
                    :options="editorOptions"
                    @change="handleQueryChange"
                  />
                </div>
                
                <div class="editor-toolbar">
                  <el-button-group>
                    <el-button size="small" @click="insertQuery('SELECT * FROM measurement LIMIT 10')">
                      SELECT
                    </el-button>
                    <el-button size="small" @click="insertQuery('SHOW DATABASES')">
                      SHOW DB
                    </el-button>
                    <el-button size="small" @click="insertQuery('SHOW MEASUREMENTS')">
                      SHOW MEASUREMENTS
                    </el-button>
                    <el-button size="small" @click="insertQuery('SHOW SERIES')">
                      SHOW SERIES
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
                      执行时间: {{ queryResults.executionTime }}ms
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
import { ref, computed, onMounted, watch } from 'vue'
import { useRoute } from 'vue-router'
import { ElMessage } from 'element-plus'
import { VideoPlay } from '@element-plus/icons-vue'
import { useConnectionStore } from '../stores/connectionStore'
import { useQueryStore } from '../stores/queryStore'
import type { QueryResult } from '../stores/queryStore'
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

// 数据库列表（模拟数据）
const databases = ref(['test_db', 'system', 'monitoring'])

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
  
  try {
    isExecuting.value = true
    const result = await queryStore.executeQuery(
      currentQuery.value,
      selectedDatabase.value,
      activeConnection.value.id
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

// 监听路由参数
watch(() => route.query, (query) => {
  if (query.database) {
    selectedDatabase.value = query.database as string
  }
  if (query.measurement) {
    currentQuery.value = `SELECT * FROM ${query.measurement} LIMIT 10`
  }
}, { immediate: true })

// 生命周期
onMounted(() => {
  if (!activeConnection.value) {
    ElMessage.warning('请先连接到数据库')
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
  gap: 15px;
}

.header-left h2 {
  margin: 0;
  color: #303133;
}

.no-connection {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 400px;
}

.editor-content {
  height: calc(100vh - 80px);
}

.editor-container {
  height: 300px;
  border: 1px solid #e4e7ed;
  border-radius: 4px;
}

.editor-toolbar {
  margin-top: 10px;
  padding: 10px 0;
  border-top: 1px solid #f0f0f0;
}

.no-results {
  padding: 40px 0;
}

.error-results {
  padding: 20px 0;
}

.results-content {
  height: 400px;
  overflow: hidden;
}

.results-info {
  display: flex;
  gap: 10px;
  margin-bottom: 10px;
}

.table-container {
  height: 350px;
  overflow: auto;
}

.json-container {
  height: 350px;
  overflow: auto;
  background-color: #f8f9fa;
  border: 1px solid #e4e7ed;
  border-radius: 4px;
  padding: 10px;
}

.json-container pre {
  margin: 0;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: 12px;
  line-height: 1.4;
}
</style> 