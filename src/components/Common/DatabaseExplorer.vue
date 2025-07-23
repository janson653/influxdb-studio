<template>
  <div class="database-explorer">
    <!-- 浏览器头部 -->
    <div class="explorer-header">
      <div class="header-title">
        <span class="ide-icon ide-icon-database"></span>
        <span>数据库浏览器</span>
        <span class="ide-badge" v-if="databaseCount > 0">{{ databaseCount }}</span>
      </div>
      <div class="header-actions">
        <button 
          class="ide-btn ide-btn-small" 
          @click="refreshDatabases"
          :disabled="isLoading"
          title="刷新数据库列表"
        >
          <span class="ide-icon ide-icon-refresh" :class="{ 'ide-loading-spinner': isLoading }"></span>
        </button>
        <button 
          class="ide-btn ide-btn-small" 
          @click="showCreateDialog = true"
          title="创建新数据库"
        >
          <span class="ide-icon ide-icon-plus"></span>
        </button>
        <button 
          class="ide-btn ide-btn-small" 
          @click="showSettings = true"
          title="设置"
        >
          <span class="ide-icon ide-icon-settings"></span>
        </button>
      </div>
    </div>

    <!-- 搜索框 -->
    <div class="explorer-search">
      <input 
        type="text" 
        class="ide-input search-input"
        v-model="searchQuery"
        placeholder="搜索数据库或表..."
        @input="filterDatabases"
      />
    </div>

    <!-- 数据库树 -->
    <div class="explorer-tree">
      <div v-if="isLoading" class="ide-loading">
        <div class="ide-loading-spinner"></div>
        <span>加载中...</span>
      </div>
      
      <div v-else-if="filteredDatabases.length === 0" class="ide-empty">
        <div class="ide-empty-icon">📁</div>
        <div class="ide-empty-text">
          {{ searchQuery ? '未找到匹配的数据库' : '暂无数据库' }}
        </div>
        <button 
          v-if="!searchQuery" 
          class="ide-btn ide-btn-primary"
          @click="showCreateDialog = true"
        >
          创建第一个数据库
        </button>
      </div>
      
      <div v-else class="tree-container">
        <div 
          v-for="database in filteredDatabases" 
          :key="database.name"
          class="ide-tree-item"
          :class="{ 
            'selected': selectedDatabase === database.name,
            'expanded': expandedDatabases.has(database.name)
          }"
        >
          <div class="tree-item-content" @click="toggleDatabase(database.name)">
            <span 
              class="ide-tree-expand"
              :class="{ 'expanded': expandedDatabases.has(database.name) }"
              @click.stop="toggleDatabase(database.name)"
            >
              ▶
            </span>
            <span class="ide-tree-icon ide-icon-database"></span>
            <span class="tree-item-name">{{ database.name }}</span>
            <span class="tree-item-count" v-if="database.measurements">
              ({{ database.measurements.length }})
            </span>
          </div>
          
          <!-- 数据库操作菜单 -->
          <div class="tree-item-actions">
            <button 
              class="ide-btn ide-btn-small"
              @click.stop="selectDatabase(database.name)"
              title="选择数据库"
            >
              <span class="ide-icon ide-icon-play"></span>
            </button>
            <button 
              class="ide-btn ide-btn-small"
              @click.stop="showDatabaseMenu(database)"
              title="更多操作"
            >
              ⋯
            </button>
          </div>
        </div>
        
        <!-- 测量值列表 -->
        <div 
          v-for="database in filteredDatabases" 
          :key="`${database.name}-measurements`"
          class="ide-tree-children"
          v-show="expandedDatabases.has(database.name)"
        >
          <div 
            v-for="measurement in database.measurements" 
            :key="measurement.name"
            class="ide-tree-item measurement-item"
            :class="{ 'selected': selectedMeasurement === measurement.name }"
            @click="selectMeasurement(database.name, measurement.name)"
          >
            <div class="tree-item-content">
              <span class="ide-tree-icon ide-icon-table"></span>
              <span class="tree-item-name">{{ measurement.name }}</span>
              <span class="tree-item-count" v-if="measurement.series_count">
                ({{ measurement.series_count }})
              </span>
            </div>
            
            <div class="tree-item-actions">
              <button 
                class="ide-btn ide-btn-small"
                @click.stop="queryMeasurement(database.name, measurement.name)"
                title="查询数据"
              >
                <span class="ide-icon ide-icon-play"></span>
              </button>
              <button 
                class="ide-btn ide-btn-small"
                @click.stop="showMeasurementMenu(measurement)"
                title="更多操作"
              >
                ⋯
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 上下文菜单 -->
    <div 
      v-if="showContextMenu" 
      class="context-menu"
      :style="contextMenuStyle"
      @click.stop
    >
      <div class="context-menu-item" @click="handleContextMenuAction('refresh')">
        <span class="ide-icon ide-icon-refresh"></span>
        刷新
      </div>
      <div class="context-menu-item" @click="handleContextMenuAction('query')">
        <span class="ide-icon ide-icon-play"></span>
        查询
      </div>
      <div class="ide-divider"></div>
      <div class="context-menu-item" @click="handleContextMenuAction('edit')">
        <span class="ide-icon ide-icon-settings"></span>
        编辑
      </div>
      <div class="context-menu-item danger" @click="handleContextMenuAction('delete')">
        <span class="ide-icon ide-icon-close"></span>
        删除
      </div>
    </div>

    <!-- 创建数据库对话框 -->
    <CreateDatabaseDialog 
      v-model="showCreateDialog"
      @created="handleDatabaseCreated"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { useDatabaseStore } from '../../stores/databaseStore'
import { useMeasurementStore } from '../../stores/measurementStore'
import CreateDatabaseDialog from './Dialog/CreateDatabaseDialog.vue'

// Props
interface Props {
  selectedDatabase?: string | null
  selectedMeasurement?: string | null
}

const props = withDefaults(defineProps<Props>(), {
  selectedDatabase: null,
  selectedMeasurement: null
})

// Emits
const emit = defineEmits<{
  'database-selected': [databaseName: string]
  'measurement-selected': [databaseName: string, measurementName: string]
  'query-requested': [databaseName: string, measurementName: string]
}>()

// Stores
const databaseStore = useDatabaseStore()
const measurementStore = useMeasurementStore()

// 响应式数据
const searchQuery = ref('')
const isLoading = ref(false)
const expandedDatabases = ref(new Set<string>())
const showCreateDialog = ref(false)
const showSettings = ref(false)
const showContextMenu = ref(false)
const contextMenuStyle = ref({ top: '0px', left: '0px' })
const contextMenuTarget = ref<any>(null)

// 计算属性
const databaseCount = computed(() => databaseStore.databases.length)

const filteredDatabases = computed(() => {
  if (!searchQuery.value) {
    return databaseStore.databases
  }
  
  const query = searchQuery.value.toLowerCase()
  return databaseStore.databases.filter(db => {
    // 搜索数据库名称
    if (db.name.toLowerCase().includes(query)) {
      return true
    }
    
    // 搜索测量值名称
    if (db.measurements) {
      return db.measurements.some(m => m.name.toLowerCase().includes(query))
    }
    
    return false
  })
})

// 方法
const refreshDatabases = async () => {
  if (!databaseStore.isConnected) {
    ElMessage.warning('请先连接数据库')
    return
  }
  
  isLoading.value = true
  try {
    await databaseStore.fetchDatabases()
    ElMessage.success('数据库列表已刷新')
  } catch (error: any) {
    ElMessage.error(`刷新失败: ${error.message || error}`)
  } finally {
    isLoading.value = false
  }
}

const filterDatabases = () => {
  // 搜索时自动展开包含匹配项的数据库
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    filteredDatabases.value.forEach(db => {
      if (db.name.toLowerCase().includes(query) || 
          db.measurements?.some(m => m.name.toLowerCase().includes(query))) {
        expandedDatabases.value.add(db.name)
      }
    })
  }
}

const toggleDatabase = async (databaseName: string) => {
  if (expandedDatabases.value.has(databaseName)) {
    expandedDatabases.value.delete(databaseName)
  } else {
    expandedDatabases.value.add(databaseName)
    // 加载测量值
    try {
      await measurementStore.fetchMeasurementsForDatabase(databaseName)
    } catch (error) {
      console.error('加载测量值失败:', error)
    }
  }
}

const selectDatabase = (databaseName: string) => {
  emit('database-selected', databaseName)
}

const selectMeasurement = (databaseName: string, measurementName: string) => {
  emit('measurement-selected', databaseName, measurementName)
}

const queryMeasurement = (databaseName: string, measurementName: string) => {
  emit('query-requested', databaseName, measurementName)
}

const showDatabaseMenu = (database: any) => {
  contextMenuTarget.value = { type: 'database', data: database }
  showContextMenu.value = true
  // 设置菜单位置
  const rect = event?.target?.getBoundingClientRect()
  if (rect) {
    contextMenuStyle.value = {
      top: `${rect.bottom + 5}px`,
      left: `${rect.left}px`
    }
  }
}

const showMeasurementMenu = (measurement: any) => {
  contextMenuTarget.value = { type: 'measurement', data: measurement }
  showContextMenu.value = true
  // 设置菜单位置
  const rect = event?.target?.getBoundingClientRect()
  if (rect) {
    contextMenuStyle.value = {
      top: `${rect.bottom + 5}px`,
      left: `${rect.left}px`
    }
  }
}

const handleContextMenuAction = async (action: string) => {
  const target = contextMenuTarget.value
  if (!target) return
  
  showContextMenu.value = false
  
  switch (action) {
    case 'refresh':
      await refreshDatabases()
      break
      
    case 'query':
      if (target.type === 'database') {
        selectDatabase(target.data.name)
      } else if (target.type === 'measurement') {
        queryMeasurement(target.data.database, target.data.name)
      }
      break
      
    case 'edit':
      ElMessage.info('编辑功能开发中...')
      break
      
    case 'delete':
      await handleDelete(target)
      break
  }
}

const handleDelete = async (target: any) => {
  try {
    const message = target.type === 'database' 
      ? `确定要删除数据库 "${target.data.name}" 吗？此操作不可恢复。`
      : `确定要删除测量值 "${target.data.name}" 吗？此操作不可恢复。`
    
    await ElMessageBox.confirm(message, '确认删除', {
      confirmButtonText: '删除',
      cancelButtonText: '取消',
      type: 'warning'
    })
    
    if (target.type === 'database') {
      await databaseStore.deleteDatabase(target.data.name)
      ElMessage.success('数据库已删除')
    } else {
      await measurementStore.deleteMeasurement(target.data.name)
      ElMessage.success('测量值已删除')
    }
    
    await refreshDatabases()
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('删除失败')
    }
  }
}

const handleDatabaseCreated = (database: any) => {
  ElMessage.success(`数据库 "${database.name}" 创建成功`)
  refreshDatabases()
}

// 监听选中状态变化
watch(() => props.selectedDatabase, (newDatabase) => {
  if (newDatabase && !expandedDatabases.value.has(newDatabase)) {
    expandedDatabases.value.add(newDatabase)
  }
})

// 点击外部关闭上下文菜单
const handleClickOutside = () => {
  showContextMenu.value = false
}

// 生命周期
onMounted(() => {
  document.addEventListener('click', handleClickOutside)
  refreshDatabases()
})

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside)
})
</script>

<style scoped>
.database-explorer {
  height: 100%;
  display: flex;
  flex-direction: column;
  background-color: var(--ide-bg-secondary);
}

.explorer-header {
  padding: 10px;
  border-bottom: 1px solid var(--ide-border);
  display: flex;
  justify-content: space-between;
  align-items: center;
  background-color: var(--ide-bg-tertiary);
}

.header-title {
  display: flex;
  align-items: center;
  gap: 8px;
  font-weight: 500;
  color: var(--ide-text-primary);
}

.header-actions {
  display: flex;
  gap: 4px;
}

.explorer-search {
  padding: 8px 10px;
  border-bottom: 1px solid var(--ide-border);
}

.search-input {
  width: 100%;
  font-size: 12px;
}

.explorer-tree {
  flex: 1;
  overflow-y: auto;
  position: relative;
}

.tree-container {
  padding: 5px;
}

.tree-item-content {
  display: flex;
  align-items: center;
  gap: 5px;
  flex: 1;
  cursor: pointer;
  padding: 2px 0;
}

.tree-item-name {
  flex: 1;
  font-size: 13px;
  color: var(--ide-text-primary);
}

.tree-item-count {
  font-size: 11px;
  color: var(--ide-text-secondary);
}

.tree-item-actions {
  display: flex;
  gap: 2px;
  opacity: 0;
  transition: opacity var(--ide-transition-fast);
}

.ide-tree-item:hover .tree-item-actions {
  opacity: 1;
}

.measurement-item {
  margin-left: 20px;
}

.context-menu {
  position: fixed;
  background-color: var(--ide-bg-secondary);
  border: 1px solid var(--ide-border);
  border-radius: var(--ide-border-radius);
  box-shadow: var(--ide-shadow-lg);
  z-index: 1000;
  min-width: 150px;
  overflow: hidden;
}

.context-menu-item {
  padding: 8px 12px;
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  font-size: 13px;
  color: var(--ide-text-primary);
  transition: background-color var(--ide-transition-fast);
}

.context-menu-item:hover {
  background-color: var(--ide-bg-tertiary);
}

.context-menu-item.danger {
  color: var(--ide-error);
}

.context-menu-item.danger:hover {
  background-color: var(--ide-error);
  color: white;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .explorer-header {
    padding: 8px;
  }
  
  .header-title {
    font-size: 14px;
  }
  
  .tree-item-name {
    font-size: 12px;
  }
  
  .context-menu {
    min-width: 120px;
  }
  
  .context-menu-item {
    padding: 6px 10px;
    font-size: 12px;
  }
}
</style> 