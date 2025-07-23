<template>
  <div class="data-preview-tab">
    <div v-if="loading" class="loading-container">
      <el-skeleton :rows="10" animated />
    </div>
    
    <div v-else-if="!measurement" class="empty-container">
      <el-empty description="请选择一个measurement查看数据预览" />
    </div>
    
    <div v-else class="data-preview-content">
      <!-- 工具栏 -->
      <div class="toolbar">
        <div class="toolbar-left">
          <el-button 
            size="small" 
            @click="refreshData" 
            :loading="refreshing"
            :icon="Refresh"
          >
            刷新
          </el-button>
          <el-button 
            size="small" 
            @click="exportData" 
            :disabled="!data.length"
            :icon="Download"
          >
            导出
          </el-button>
          <el-tag size="small" type="info">
            共 {{ totalCount }} 条记录
          </el-tag>
        </div>
        
        <div class="toolbar-right">
          <el-input
            v-model="searchQuery"
            placeholder="搜索数据..."
            size="small"
            style="width: 200px;"
            clearable
            :prefix-icon="Search"
          />
          <el-select 
            v-model="pageSize" 
            size="small" 
            style="width: 100px;"
            @change="handlePageSizeChange"
          >
            <el-option label="10条" :value="10" />
            <el-option label="20条" :value="20" />
            <el-option label="50条" :value="50" />
            <el-option label="100条" :value="100" />
          </el-select>
        </div>
      </div>

      <!-- 数据表格 -->
      <div class="table-container">
        <el-table 
          :data="filteredData" 
          style="width: 100%"
          size="small"
          border
          stripe
          :max-height="400"
          @sort-change="handleSortChange"
        >
          <!-- 时间列 -->
          <el-table-column 
            prop="time" 
            label="时间" 
            width="180"
            sortable="custom"
            fixed="left"
          >
            <template #default="{ row }">
              <span class="timestamp">{{ formatTimestamp(row.time) }}</span>
            </template>
          </el-table-column>

          <!-- 动态字段列 -->
          <el-table-column 
            v-for="field in visibleFields" 
            :key="field"
            :prop="field"
            :label="field"
            min-width="120"
            sortable="custom"
          >
            <template #default="{ row }">
              <span 
                :class="getValueClass(row[field])"
                :title="getValueTitle(row[field])"
              >
                {{ formatValue(row[field]) }}
              </span>
            </template>
          </el-table-column>

          <!-- 标签列 -->
          <el-table-column 
            v-if="visibleTags.length"
            label="标签" 
            min-width="200"
            fixed="right"
          >
            <template #default="{ row }">
              <div class="tags-display">
                <el-tag 
                  v-for="tag in getVisibleTags(row)" 
                  :key="tag.key"
                  size="small"
                  class="data-tag"
                >
                  {{ tag.key }}: {{ tag.value }}
                </el-tag>
                <span v-if="getHiddenTagsCount(row) > 0" class="more-tags">
                  +{{ getHiddenTagsCount(row) }}
                </span>
              </div>
            </template>
          </el-table-column>
        </el-table>
      </div>

      <!-- 分页 -->
      <div class="pagination-container">
        <el-pagination
          v-model:current-page="currentPage"
          v-model:page-size="pageSize"
          :page-sizes="[10, 20, 50, 100]"
          :total="totalCount"
          layout="total, sizes, prev, pager, next, jumper"
          @size-change="handlePageSizeChange"
          @current-change="handleCurrentChange"
        />
      </div>

      <!-- 字段选择器 -->
      <el-drawer 
        v-model="showFieldSelector" 
        title="字段选择" 
        direction="rtl"
        size="300px"
      >
        <div class="field-selector">
          <h4>显示字段</h4>
          <el-checkbox-group v-model="selectedFields">
            <el-checkbox 
              v-for="field in allFields" 
              :key="field"
              :label="field"
            >
              {{ field }}
            </el-checkbox>
          </el-checkbox-group>
          
          <h4>显示标签</h4>
          <el-checkbox-group v-model="selectedTags">
            <el-checkbox 
              v-for="tag in allTags" 
              :key="tag"
              :label="tag"
            >
              {{ tag }}
            </el-checkbox>
          </el-checkbox-group>
        </div>
      </el-drawer>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { ElMessage } from 'element-plus'
import { Refresh, Download, Search } from '@element-plus/icons-vue'
// import { useMeasurementStore } from '../../../stores/measurementStore'

interface Props {
  measurement: any
  loading?: boolean
}

const props = defineProps<Props>()

// const measurementStore = useMeasurementStore()

// 响应式数据
const data = ref<any[]>([])
const loading = ref(false)
const refreshing = ref(false)
const searchQuery = ref('')
const currentPage = ref(1)
const pageSize = ref(20)
const totalCount = ref(0)
const sortBy = ref('')
const sortOrder = ref<'ascending' | 'descending' | null>(null)
const showFieldSelector = ref(false)
const selectedFields = ref<string[]>([])
const selectedTags = ref<string[]>([])

// 计算属性
const allFields = computed(() => {
  if (!data.value.length) return []
  const fields = new Set<string>()
  data.value.forEach(row => {
    Object.keys(row).forEach(key => {
      if (key !== 'time' && key !== 'tags') {
        fields.add(key)
      }
    })
  })
  return Array.from(fields)
})

const allTags = computed(() => {
  if (!data.value.length) return []
  const tags = new Set<string>()
  data.value.forEach(row => {
    if (row.tags) {
      Object.keys(row.tags).forEach(key => {
        tags.add(key)
      })
    }
  })
  return Array.from(tags)
})

const visibleFields = computed(() => {
  return selectedFields.value.length > 0 ? selectedFields.value : allFields.value
})

const visibleTags = computed(() => {
  return selectedTags.value
})

const filteredData = computed(() => {
  let filtered = data.value
  
  // 搜索过滤
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    filtered = filtered.filter(row => {
      return Object.entries(row).some(([key, value]) => {
        if (key === 'time' || key === 'tags') return false
        return String(value).toLowerCase().includes(query)
      })
    })
  }
  
  // 排序
  if (sortBy.value && sortOrder.value) {
    filtered = [...filtered].sort((a, b) => {
      const aVal = a[sortBy.value]
      const bVal = b[sortBy.value]
      
      if (sortOrder.value === 'ascending') {
        return aVal > bVal ? 1 : -1
      } else {
        return aVal < bVal ? 1 : -1
      }
    })
  }
  
  return filtered
})

// 方法
const loadData = async () => {
  if (!props.measurement) return
  
  loading.value = true
  try {
    // 暂时使用模拟数据，因为getMeasurementData方法不存在
    const mockData = [
      { time: new Date().toISOString(), value: 100, tag1: 'value1' },
      { time: new Date(Date.now() - 60000).toISOString(), value: 200, tag1: 'value2' },
      { time: new Date(Date.now() - 120000).toISOString(), value: 150, tag1: 'value1' }
    ]
    
    const result = {
      data: mockData,
      total: mockData.length
    }
    
    data.value = result.data || []
    totalCount.value = result.total || 0
    
    // 初始化字段选择
    if (selectedFields.value.length === 0) {
      selectedFields.value = allFields.value
    }
    if (selectedTags.value.length === 0) {
      selectedTags.value = allTags.value
    }
  } catch (error: any) {
    ElMessage.error(error.message || '加载数据失败')
  } finally {
    loading.value = false
  }
}

const refreshData = async () => {
  refreshing.value = true
  await loadData()
  refreshing.value = false
}

const exportData = () => {
  const csvContent = generateCSV()
  const blob = new Blob([csvContent], { type: 'text/csv;charset=utf-8;' })
  const link = document.createElement('a')
  link.href = URL.createObjectURL(blob)
  link.download = `${props.measurement.name}_data.csv`
  link.click()
}

const generateCSV = () => {
  const headers = ['time', ...visibleFields.value, ...visibleTags.value.map(tag => `tag_${tag}`)]
  const rows = filteredData.value.map(row => {
    const csvRow = [formatTimestamp(row.time)]
    
    // 添加字段值
    visibleFields.value.forEach(field => {
      csvRow.push(formatValue(row[field]))
    })
    
    // 添加标签值
    visibleTags.value.forEach(tag => {
      csvRow.push(row.tags?.[tag] || '')
    })
    
    return csvRow.join(',')
  })
  
  return [headers.join(','), ...rows].join('\n')
}

const formatTimestamp = (timestamp: string | number) => {
  if (!timestamp) return ''
  const date = new Date(timestamp)
  return date.toLocaleString('zh-CN')
}

const formatValue = (value: any) => {
  if (value === null || value === undefined) {
    return 'null'
  }
  
  if (typeof value === 'number') {
    return value.toString()
  }
  
  if (typeof value === 'boolean') {
    return value ? 'true' : 'false'
  }
  
  return String(value)
}

const getValueClass = (value: any) => {
  if (value === null || value === undefined) {
    return 'null-value'
  }
  
  if (typeof value === 'number') {
    return 'number-value'
  }
  
  if (typeof value === 'boolean') {
    return 'boolean-value'
  }
  
  return 'string-value'
}

const getValueTitle = (value: any) => {
  return formatValue(value)
}

const getVisibleTags = (row: any) => {
  if (!row.tags || !visibleTags.value.length) return []
  
  return visibleTags.value.slice(0, 3).map(tag => ({
    key: tag,
    value: row.tags[tag]
  }))
}

const getHiddenTagsCount = (row: any) => {
  if (!row.tags) return 0
  return Math.max(0, visibleTags.value.length - 3)
}

const handleSortChange = ({ prop, order }: any) => {
  sortBy.value = prop
  sortOrder.value = order
  loadData()
}

const handlePageSizeChange = (size: number) => {
  pageSize.value = size
  currentPage.value = 1
  loadData()
}

const handleCurrentChange = (page: number) => {
  currentPage.value = page
  loadData()
}

// 监听measurement变化
watch(() => props.measurement, () => {
  if (props.measurement) {
    currentPage.value = 1
    selectedFields.value = []
    selectedTags.value = []
    loadData()
  }
}, { immediate: true })
</script>

<style scoped>
.data-preview-tab {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.loading-container,
.empty-container {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 300px;
}

.data-preview-content {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.toolbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  border-bottom: 1px solid #e4e7ed;
  background-color: #fafafa;
}

.toolbar-left,
.toolbar-right {
  display: flex;
  align-items: center;
  gap: 12px;
}

.table-container {
  flex: 1;
  overflow: hidden;
}

.pagination-container {
  padding: 12px 16px;
  border-top: 1px solid #e4e7ed;
  background-color: #fafafa;
  display: flex;
  justify-content: center;
}

.timestamp {
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: 11px;
  color: #606266;
}

.null-value {
  color: #c0c4cc;
  font-style: italic;
}

.number-value {
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  color: #409eff;
}

.boolean-value {
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  color: #67c23a;
}

.string-value {
  color: #303133;
}

.tags-display {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
  align-items: center;
}

.data-tag {
  font-size: 10px;
  max-width: 120px;
  overflow: hidden;
  text-overflow: ellipsis;
}

.more-tags {
  font-size: 10px;
  color: #909399;
  font-style: italic;
}

.field-selector {
  padding: 16px;
}

.field-selector h4 {
  margin: 16px 0 8px 0;
  color: #303133;
  font-size: 14px;
}

.field-selector .el-checkbox-group {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

:deep(.el-table) {
  font-size: 12px;
}

:deep(.el-table th) {
  background-color: #f5f7fa;
  font-weight: 600;
}

:deep(.el-table td) {
  padding: 4px 8px;
}
</style> 