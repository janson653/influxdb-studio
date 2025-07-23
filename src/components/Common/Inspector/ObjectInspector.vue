<template>
  <div class="object-inspector">
    <div v-if="!object" class="empty-state">
      <el-empty description="请选择一个对象查看详细信息" />
    </div>
    
    <div v-else class="inspector-content">
      <!-- 对象标题 -->
      <div class="object-header">
        <div class="object-info">
          <el-icon class="object-icon" :class="getObjectIconClass()">
            <component :is="getObjectIcon()" />
          </el-icon>
          <div class="object-details">
            <h3 class="object-name">{{ object.name }}</h3>
            <div class="object-meta">
              <el-tag :type="getTypeTagType()" size="small">
                {{ getTypeLabel() }}
              </el-tag>
              <span v-if="object.database" class="database-name">
                在 {{ object.database }}
              </span>
            </div>
          </div>
        </div>
        
        <div class="object-actions">
          <el-button 
            size="small" 
            @click="refreshObject" 
            :loading="refreshing"
            :icon="Refresh"
          >
            刷新
          </el-button>
          <el-button 
            size="small" 
            @click="openInQueryEditor" 
            :icon="Edit"
          >
            查询
          </el-button>
        </div>
      </div>

      <!-- 标签页 -->
      <div class="inspector-tabs">
        <el-tabs v-model="activeTab" class="inspector-tabs-content">
          <el-tab-pane label="结构" name="structure">
            <StructureTab 
              :measurement="object" 
              :loading="loading"
            />
          </el-tab-pane>
          
          <el-tab-pane label="属性" name="properties">
            <PropertiesTab 
              :object="object" 
              :loading="loading"
            />
          </el-tab-pane>
          
          <el-tab-pane 
            v-if="object.type === 'measurement'" 
            label="数据预览" 
            name="data"
          >
            <DataPreviewTab 
              :measurement="object" 
              :loading="loading"
            />
          </el-tab-pane>
          
          <el-tab-pane label="DDL" name="ddl">
            <DDLTab 
              :object="object" 
              :loading="loading"
            />
          </el-tab-pane>
        </el-tabs>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { ElMessage } from 'element-plus'
import { 
  Refresh, 
  Edit, 
  Folder, 
  Document, 
  Collection, 
  DataAnalysis 
} from '@element-plus/icons-vue'
import StructureTab from './StructureTab.vue'
import PropertiesTab from './PropertiesTab.vue'
import DataPreviewTab from './DataPreviewTab.vue'
import DDLTab from './DDLTab.vue'

interface Props {
  object: any
  loading?: boolean
}

interface Emits {
  (e: 'refresh'): void
  (e: 'query', object: any): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

// 响应式数据
const activeTab = ref('structure')
const refreshing = ref(false)

// 计算属性
const loading = computed(() => props.loading || refreshing.value)

// 方法
const getObjectIcon = () => {
  switch (props.object?.type) {
    case 'database':
      return Folder
    case 'measurement':
      return Document
    case 'series':
      return Collection
    case 'field':
      return DataAnalysis
    default:
      return Document
  }
}

const getObjectIconClass = () => {
  return `object-icon-${props.object?.type || 'default'}`
}

const getTypeTagType = () => {
  const typeMap: Record<string, string> = {
    'database': 'success',
    'measurement': 'warning',
    'series': 'info',
    'field': 'primary'
  }
  return typeMap[props.object?.type] || 'info'
}

const getTypeLabel = () => {
  const labelMap: Record<string, string> = {
    'database': '数据库',
    'measurement': 'Measurement',
    'series': '系列',
    'field': '字段'
  }
  return labelMap[props.object?.type] || props.object?.type || '对象'
}

const refreshObject = async () => {
  refreshing.value = true
  try {
    emit('refresh')
    ElMessage.success('对象信息已刷新')
  } catch (error: any) {
    ElMessage.error(error.message || '刷新失败')
  } finally {
    refreshing.value = false
  }
}

const openInQueryEditor = () => {
  emit('query', props.object)
}

// 监听对象变化，重置标签页
watch(() => props.object, () => {
  if (props.object) {
    // 根据对象类型设置默认标签页
    if (props.object.type === 'measurement') {
      activeTab.value = 'structure'
    } else {
      activeTab.value = 'properties'
    }
  }
}, { immediate: true })
</script>

<style scoped>
.object-inspector {
  height: 100%;
  display: flex;
  flex-direction: column;
  background-color: white;
}

.empty-state {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100%;
  color: #909399;
}

.inspector-content {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.object-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px;
  border-bottom: 1px solid #e4e7ed;
  background-color: #fafafa;
}

.object-info {
  display: flex;
  align-items: center;
  gap: 12px;
}

.object-icon {
  font-size: 24px;
  color: #409eff;
}

.object-icon-database {
  color: #67c23a;
}

.object-icon-measurement {
  color: #e6a23c;
}

.object-icon-series {
  color: #909399;
}

.object-icon-field {
  color: #f56c6c;
}

.object-details {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.object-name {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  color: #303133;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
}

.object-meta {
  display: flex;
  align-items: center;
  gap: 8px;
}

.database-name {
  font-size: 12px;
  color: #606266;
}

.object-actions {
  display: flex;
  gap: 8px;
}

.inspector-tabs {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.inspector-tabs-content {
  height: 100%;
  display: flex;
  flex-direction: column;
}

:deep(.el-tabs__content) {
  flex: 1;
  overflow: hidden;
}

:deep(.el-tab-pane) {
  height: 100%;
}

:deep(.el-tabs__header) {
  margin: 0;
  background-color: #f5f7fa;
  border-bottom: 1px solid #e4e7ed;
}

:deep(.el-tabs__nav-wrap) {
  padding: 0 20px;
}

:deep(.el-tabs__item) {
  font-size: 13px;
  font-weight: 500;
}

:deep(.el-tabs__item.is-active) {
  color: #409eff;
  font-weight: 600;
}

:deep(.el-tabs__active-bar) {
  background-color: #409eff;
}
</style> 