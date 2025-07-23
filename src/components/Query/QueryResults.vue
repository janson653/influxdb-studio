<template>
  <div class="query-results">
    <!-- 结果标签页 -->
    <div class="results-tabs">
      <button 
        v-for="tab in resultTabs" 
        :key="tab.id"
        class="ide-tab" 
        :class="{ active: activeTab === tab.id }"
        @click="setActiveTab(tab.id)"
      >
        <span class="ide-icon" :class="tab.icon"></span>
        {{ tab.name }}
        <span class="ide-badge" v-if="tab.count">{{ tab.count }}</span>
        <button 
          v-if="tab.id !== 'output'"
          class="tab-close" 
          @click.stop="closeTab(tab.id)"
        >
          ×
        </button>
      </button>
      <button class="ide-tab new-tab" @click="createNewTab">
        <span>+</span>
      </button>
    </div>
    
    <!-- 结果工具栏 -->
    <div class="results-toolbar">
      <div class="results-info">
        <span v-if="currentResult">
          {{ getResultInfo(currentResult) }}
        </span>
        <span v-else>暂无结果</span>
      </div>
      <div class="results-actions">
        <button 
          class="ide-btn ide-btn-small"
          @click="refreshResults"
          :disabled="isLoading"
          title="刷新结果"
        >
          <span class="ide-icon ide-icon-refresh" :class="{ 'ide-loading-spinner': isLoading }"></span>
        </button>
        <button 
          class="ide-btn ide-btn-small"
          @click="exportResults"
          :disabled="!currentResult"
          title="导出结果"
        >
          📄
        </button>
        <button 
          class="ide-btn ide-btn-small"
          @click="pinResults"
          :disabled="!currentResult"
          title="固定结果"
        >
          📌
        </button>
        <button 
          class="ide-btn ide-btn-small"
          @click="showChart"
          :disabled="!currentResult"
          title="图表视图"
        >
          📊
        </button>
      </div>
    </div>
    
    <!-- 结果内容 -->
    <div class="results-content">
      <!-- 表格视图 -->
      <div v-if="activeTab === 'table' && currentResult" class="table-view">
        <div class="table-container">
          <table class="ide-table">
            <thead>
              <tr>
                <th 
                  v-for="column in currentResult.columns" 
                  :key="column"
                  @click="sortByColumn(column)"
                  :class="{ 'sortable': true, 'sorted': sortColumn === column }"
                >
                  {{ column }}
                  <span v-if="sortColumn === column" class="sort-indicator">
                    {{ sortDirection === 'asc' ? '↑' : '↓' }}
                  </span>
                </th>
              </tr>
            </thead>
            <tbody>
              <tr 
                v-for="(row, index) in paginatedData" 
                :key="index"
                :class="{ 'selected': selectedRows.has(index) }"
                @click="toggleRowSelection(index)"
              >
                <td 
                  v-for="(value, colIndex) in row" 
                  :key="colIndex"
                  :title="String(value)"
                >
                  {{ formatCellValue(value) }}
                </td>
              </tr>
            </tbody>
          </table>
        </div>
        
        <!-- 分页 -->
        <div v-if="totalPages > 1" class="pagination">
          <button 
            class="ide-btn ide-btn-small"
            @click="previousPage"
            :disabled="currentPage === 1"
          >
            上一页
          </button>
          <span class="page-info">
            {{ currentPage }} / {{ totalPages }}
          </span>
          <button 
            class="ide-btn ide-btn-small"
            @click="nextPage"
            :disabled="currentPage === totalPages"
          >
            下一页
          </button>
        </div>
      </div>
      
      <!-- 图表视图 -->
      <div v-else-if="activeTab === 'chart' && currentResult" class="chart-view">
        <div class="chart-container">
          <div class="chart-placeholder">
            <div class="ide-empty-icon">📊</div>
            <div class="ide-empty-text">图表功能开发中...</div>
          </div>
        </div>
      </div>
      
      <!-- 输出日志 -->
      <div v-else-if="activeTab === 'output'" class="output-view">
        <div class="output-container">
          <div 
            v-for="(log, index) in outputLogs" 
            :key="index"
            class="output-log"
            :class="log.type"
          >
            <span class="log-timestamp">{{ formatTimestamp(log.timestamp) }}</span>
            <span class="log-message">{{ log.message }}</span>
            <div v-if="log.data" class="log-data">
              <pre>{{ JSON.stringify(log.data, null, 2) }}</pre>
            </div>
          </div>
        </div>
      </div>
      
      <!-- 空状态 -->
      <div v-else class="ide-empty">
        <div class="ide-empty-icon">📋</div>
        <div class="ide-empty-text">暂无结果数据</div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { ElMessage } from 'element-plus'
import type { QueryResult } from '../../stores/queryStore'

// Props
interface Props {
  results?: QueryResult | null
  isLoading?: boolean
  outputLogs?: any[]
}

const props = withDefaults(defineProps<Props>(), {
  results: null,
  isLoading: false,
  outputLogs: () => []
})

// Emits
const emit = defineEmits<{
  'refresh': []
  'export': [data: any]
}>()

// 响应式数据
const activeTab = ref('output')
const sortColumn = ref<string | null>(null)
const sortDirection = ref<'asc' | 'desc'>('asc')
const selectedRows = ref(new Set<number>())
const currentPage = ref(1)
const pageSize = ref(50)

// 结果标签页
const resultTabs = ref([
  { id: 'output', name: '输出', icon: 'ide-icon-settings', count: 0 }
])

// 计算属性
const currentResult = computed(() => props.results)

const sortedData = computed(() => {
  if (!currentResult.value || !sortColumn.value) {
    return currentResult.value?.series[0]?.values || []
  }
  
  const columnIndex = currentResult.value.series[0].columns.indexOf(sortColumn.value)
  if (columnIndex === -1) return currentResult.value.series[0].values
  
  return [...currentResult.value.series[0].values].sort((a, b) => {
    const aVal = a[columnIndex]
    const bVal = b[columnIndex]
    
    if (aVal === bVal) return 0
    if (aVal === null || aVal === undefined) return 1
    if (bVal === null || bVal === undefined) return -1
    
    const comparison = aVal < bVal ? -1 : 1
    return sortDirection.value === 'asc' ? comparison : -comparison
  })
})

const totalPages = computed(() => {
  if (!sortedData.value) return 1
  return Math.ceil(sortedData.value.length / pageSize.value)
})

const paginatedData = computed(() => {
  if (!sortedData.value) return []
  const start = (currentPage.value - 1) * pageSize.value
  const end = start + pageSize.value
  return sortedData.value.slice(start, end)
})

// 方法
const setActiveTab = (tabId: string) => {
  activeTab.value = tabId
}

const createNewTab = () => {
  const newTabId = `result_${Date.now()}`
  resultTabs.value.push({
    id: newTabId,
    name: `结果 ${resultTabs.value.length}`,
    icon: 'ide-icon-table',
    count: 0
  })
  setActiveTab(newTabId)
}

const closeTab = (tabId: string) => {
  const index = resultTabs.value.findIndex(tab => tab.id === tabId)
  if (index > -1) {
    resultTabs.value.splice(index, 1)
    if (activeTab.value === tabId) {
      setActiveTab(resultTabs.value[0]?.id || 'output')
    }
  }
}

const refreshResults = () => {
  emit('refresh')
}

const exportResults = () => {
  if (!currentResult.value) return
  
  try {
    const data = {
      query: currentResult.value.query || '',
      timestamp: new Date().toISOString(),
      results: currentResult.value
    }
    
    const blob = new Blob([JSON.stringify(data, null, 2)], { type: 'application/json' })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `query_results_${Date.now()}.json`
    a.click()
    URL.revokeObjectURL(url)
    
    ElMessage.success('结果已导出')
  } catch (error) {
    ElMessage.error('导出失败')
  }
}

const pinResults = () => {
  ElMessage.info('固定功能开发中...')
}

const showChart = () => {
  setActiveTab('chart')
}

const sortByColumn = (column: string) => {
  if (sortColumn.value === column) {
    sortDirection.value = sortDirection.value === 'asc' ? 'desc' : 'asc'
  } else {
    sortColumn.value = column
    sortDirection.value = 'asc'
  }
}

const toggleRowSelection = (index: number) => {
  if (selectedRows.value.has(index)) {
    selectedRows.value.delete(index)
  } else {
    selectedRows.value.add(index)
  }
}

const previousPage = () => {
  if (currentPage.value > 1) {
    currentPage.value--
  }
}

const nextPage = () => {
  if (currentPage.value < totalPages.value) {
    currentPage.value++
  }
}

const getResultInfo = (result: QueryResult) => {
  const series = result.series[0]
  if (!series) return '无数据'
  
  const totalRows = series.values.length
  const totalCols = series.columns.length
  
  return `${totalRows} 行 × ${totalCols} 列`
}

const formatCellValue = (value: any) => {
  if (value === null || value === undefined) {
    return '<null>'
  }
  
  if (typeof value === 'object') {
    return JSON.stringify(value)
  }
  
  return String(value)
}

const formatTimestamp = (timestamp: Date) => {
  return timestamp.toLocaleTimeString()
}

// 监听结果变化
watch(() => props.results, (newResults) => {
  if (newResults && newResults.series.length > 0) {
    // 自动切换到表格视图
    if (activeTab.value === 'output') {
      setActiveTab('table')
    }
    
    // 更新标签页计数
    const tableTab = resultTabs.value.find(tab => tab.id === 'table')
    if (tableTab) {
      tableTab.count = newResults.series[0].values.length
    }
  }
}, { immediate: true })

// 监听输出日志变化
watch(() => props.outputLogs, (newLogs) => {
  const outputTab = resultTabs.value.find(tab => tab.id === 'output')
  if (outputTab) {
    outputTab.count = newLogs.length
  }
}, { immediate: true })
</script>

<style scoped>
.query-results {
  height: 100%;
  display: flex;
  flex-direction: column;
  background-color: var(--ide-bg-primary);
}

.results-tabs {
  height: 35px;
  background-color: var(--ide-bg-secondary);
  border-bottom: 1px solid var(--ide-border);
  display: flex;
  align-items: center;
}

.results-toolbar {
  height: 30px;
  background-color: var(--ide-bg-secondary);
  border-bottom: 1px solid var(--ide-border);
  display: flex;
  align-items: center;
  padding: 0 10px;
  gap: 10px;
}

.results-info {
  font-size: 12px;
  color: var(--ide-text-secondary);
}

.results-actions {
  display: flex;
  gap: 5px;
  margin-left: auto;
}

.results-content {
  flex: 1;
  overflow: hidden;
  position: relative;
}

.table-view {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.table-container {
  flex: 1;
  overflow: auto;
}

.chart-view {
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
}

.chart-container {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
}

.chart-placeholder {
  text-align: center;
  color: var(--ide-text-secondary);
}

.output-view {
  height: 100%;
  overflow-y: auto;
}

.output-container {
  padding: 10px;
}

.output-log {
  margin-bottom: 8px;
  padding: 8px;
  border-radius: var(--ide-border-radius);
  background-color: var(--ide-bg-secondary);
  font-family: var(--ide-font-mono);
  font-size: 12px;
}

.output-log.success {
  border-left: 3px solid var(--ide-success);
}

.output-log.error {
  border-left: 3px solid var(--ide-error);
}

.output-log.info {
  border-left: 3px solid var(--ide-info);
}

.output-log.warning {
  border-left: 3px solid var(--ide-warning);
}

.log-timestamp {
  color: var(--ide-text-secondary);
  margin-right: 8px;
}

.log-message {
  color: var(--ide-text-primary);
}

.log-data {
  margin-top: 8px;
  padding: 8px;
  background-color: var(--ide-bg-primary);
  border-radius: var(--ide-border-radius);
  overflow-x: auto;
}

.log-data pre {
  margin: 0;
  font-size: 11px;
  color: var(--ide-text-secondary);
}

.pagination {
  height: 40px;
  background-color: var(--ide-bg-secondary);
  border-top: 1px solid var(--ide-border);
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 10px;
  padding: 0 10px;
}

.page-info {
  font-size: 12px;
  color: var(--ide-text-secondary);
  min-width: 60px;
  text-align: center;
}

/* 表格增强样式 */
.ide-table th.sortable {
  cursor: pointer;
  user-select: none;
  position: relative;
}

.ide-table th.sortable:hover {
  background-color: var(--ide-bg-hover);
}

.ide-table th.sorted {
  background-color: var(--ide-accent-primary);
  color: white;
}

.sort-indicator {
  margin-left: 4px;
  font-size: 10px;
}

.ide-table tr.selected {
  background-color: var(--ide-accent-primary);
  color: white;
}

.ide-table tr.selected:hover {
  background-color: var(--ide-accent-secondary);
}

/* 响应式设计 */
@media (max-width: 768px) {
  .results-toolbar {
    height: 35px;
    padding: 0 5px;
  }
  
  .results-info {
    font-size: 11px;
  }
  
  .pagination {
    height: 35px;
  }
  
  .page-info {
    font-size: 11px;
  }
}
</style> 