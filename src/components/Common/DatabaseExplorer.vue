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
      <!-- 调试信息 -->
      <div class="debug-info" style="padding: 8px; font-size: 10px; color: var(--ide-text-tertiary); border-bottom: 1px solid var(--ide-border);">
        <div>数据库数量: {{ databaseStore.databases.length }}</div>
        <div>测量值数量: {{ measurementStore.measurements.length }}</div>
        <div>展开的数据库: {{ Array.from(expandedDatabases).join(', ') || '无' }}</div>
        <div>活跃连接: {{ connectionStore.activeConnectionId || '无' }}</div>
        <div>连接状态: {{ connectionStore.connectionStatus[connectionStore.activeConnectionId || '']?.status || '无' }}</div>
        <div>后端连接ID: {{ connectionStore.connectionStatus[connectionStore.activeConnectionId || '']?.backendConnectionId || '无' }}</div>
        <div>isConnected: {{ connectionStore.isConnected }}</div>
        <div>数据库加载状态: {{ databaseStore.loading ? '加载中' : '已完成' }}</div>
        <div>数据库错误: {{ databaseStore.error || '无' }}</div>
      </div>
      
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
          class="database-explorer-item database-item"
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
            <span class="tree-item-count" v-if="getDatabaseMeasurements(database.name).length > 0">
              ({{ getDatabaseMeasurements(database.name).length }})
            </span>
          </div>
          
          <!-- 数据库操作 -->
          <div class="tree-item-actions">
            <button 
              class="ide-btn ide-btn-small"
              @click.stop="selectDatabase(database.name)"
              title="选择数据库"
            >
              <span class="ide-icon ide-icon-play"></span>
            </button>
          </div>
          
          <!-- 测量值列表 -->
          <div v-if="expandedDatabases.has(database.name)" class="database-explorer-children">
            <div 
              v-for="measurement in getDatabaseMeasurements(database.name)" 
              :key="measurement.name"
              class="database-explorer-item measurement-item"
              :class="{ 'selected': selectedMeasurement === measurement.name }"
              @click="selectMeasurement(database.name, measurement.name)"
            >
              <div class="tree-item-content">
                <span class="ide-tree-icon ide-icon-table"></span>
                <span class="tree-item-name">{{ measurement.name }}</span>
              </div>
              
              <!-- 测量值操作 -->
              <div class="tree-item-actions">
                <button 
                  class="ide-btn ide-btn-small"
                  @click.stop="requestQuery(database.name, measurement.name)"
                  title="查询数据"
                >
                  <span class="ide-icon ide-icon-play"></span>
                </button>
              </div>
            </div>
          </div>
        </div>
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
import { ElMessage } from 'element-plus'
import { useDatabaseStore } from '../../stores/databaseStore'
import { useMeasurementStore } from '../../stores/measurementStore'
import { useConnectionStore } from '../../stores/connectionStore'
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
const connectionStore = useConnectionStore()

// 响应式数据
const searchQuery = ref('')
const isLoading = ref(false)
const expandedDatabases = ref(new Set<string>())
const showCreateDialog = ref(false)

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
    const dbMeasurements = measurementStore.measurements.filter(m => m.database === db.name)
    return dbMeasurements.some(m => m.name.toLowerCase().includes(query))
  })
})

// 获取数据库的测量值
const getDatabaseMeasurements = (databaseName: string) => {
  return measurementStore.measurements.filter(m => m.database === databaseName)
}

// 方法
const refreshDatabases = async () => {
  if (!connectionStore.isConnected) {
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
          getDatabaseMeasurements(db.name).some(m => m.name.toLowerCase().includes(query))) {
        expandedDatabases.value.add(db.name)
      }
    })
  }
}

const toggleDatabase = async (databaseName: string) => {
  console.log('切换数据库展开状态:', databaseName)
  
  if (expandedDatabases.value.has(databaseName)) {
    expandedDatabases.value.delete(databaseName)
    console.log('折叠数据库:', databaseName)
  } else {
    expandedDatabases.value.add(databaseName)
    console.log('展开数据库:', databaseName)
    
    // 加载测量值
    try {
      console.log('开始加载测量值:', databaseName)
      const measurements = await measurementStore.fetchMeasurementsForDatabase(databaseName)
      console.log('测量值加载完成:', databaseName, measurements)
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

const requestQuery = (databaseName: string, measurementName: string) => {
  emit('query-requested', databaseName, measurementName)
}

const handleDatabaseCreated = (databaseName: string) => {
  showCreateDialog.value = false
  refreshDatabases()
  ElMessage.success(`数据库 ${databaseName} 创建成功`)
}

// 监听选中状态变化
watch(() => props.selectedDatabase, (newDatabase) => {
  if (newDatabase && !expandedDatabases.value.has(newDatabase)) {
    expandedDatabases.value.add(newDatabase)
  }
})

// 监听连接状态变化
watch(() => connectionStore.isConnected, (connected) => {
  if (connected) {
    console.log('连接状态变为已连接，自动刷新数据库列表')
    refreshDatabases()
  } else {
    console.log('连接状态变为未连接，清空数据库列表')
    databaseStore.databases = []
    measurementStore.measurements = []
    expandedDatabases.value.clear()
  }
})

// 生命周期
onMounted(() => {
  if (connectionStore.isConnected) {
    refreshDatabases()
  }
})
</script>

<style scoped>
.database-explorer {
  height: 100%;
  display: flex;
  flex-direction: column;
  background-color: var(--ide-bg-primary);
}

.explorer-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--ide-spacing-sm) var(--ide-spacing-md);
  border-bottom: 1px solid var(--ide-border);
  background-color: var(--ide-bg-secondary);
}

.header-title {
  display: flex;
  align-items: center;
  gap: var(--ide-spacing-xs);
  font-weight: 600;
  color: var(--ide-text-primary);
}

.header-actions {
  display: flex;
  gap: var(--ide-spacing-xs);
}

.explorer-search {
  padding: var(--ide-spacing-sm) var(--ide-spacing-md);
  border-bottom: 1px solid var(--ide-border);
  background-color: var(--ide-bg-secondary);
}

.search-input {
  width: 100%;
  font-size: var(--ide-font-size-sm);
  background-color: var(--ide-bg-primary);
  border: 1px solid var(--ide-border);
  color: var(--ide-text-primary);
  font-weight: 500;
}

.search-input::placeholder {
  color: var(--ide-text-secondary);
  opacity: 1;
  font-weight: 400;
}

.search-input:focus {
  background-color: var(--ide-bg-secondary);
  border-color: var(--ide-accent-primary);
  color: var(--ide-text-primary);
  font-weight: 500;
}

.explorer-tree {
  flex: 1;
  overflow-y: auto;
  padding: var(--ide-spacing-sm);
  background-color: var(--ide-bg-primary);
}

.tree-container {
  display: flex;
  flex-direction: column;
  gap: 2px;
  padding: var(--ide-spacing-xs);
  margin-left: 0;
}

.tree-item-content {
  display: flex;
  align-items: center;
  gap: var(--ide-spacing-xs);
  flex: 1;
  cursor: pointer;
  padding: var(--ide-spacing-xs) var(--ide-spacing-sm);
  border-radius: var(--ide-border-radius);
  transition: all var(--ide-transition-fast);
  background-color: var(--ide-bg-primary);
  border: 1px solid transparent;
  margin: 2px 0;
}

.tree-item-content:hover {
  background-color: var(--ide-bg-hover);
  border-color: var(--ide-border-light);
  transform: translateX(2px);
}

.tree-item-name {
  flex: 1;
  font-size: var(--ide-font-size-sm);
  color: var(--ide-text-primary);
  font-weight: 600;
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.3);
}

.tree-item-count {
  font-size: var(--ide-font-size-xs);
  color: var(--ide-text-primary);
  background-color: var(--ide-accent-primary);
  padding: 2px 8px;
  border-radius: 12px;
  font-weight: 600;
  text-shadow: none;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
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

.ide-tree-item.selected .tree-item-content {
  background-color: var(--ide-accent-primary);
  border-color: var(--ide-accent-secondary);
  color: var(--ide-text-inverse);
  box-shadow: 0 2px 8px rgba(124, 58, 237, 0.3);
}

.ide-tree-item.selected .tree-item-name {
  color: var(--ide-text-inverse);
  font-weight: 700;
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.3);
}

.ide-tree-item.selected .tree-item-count {
  color: var(--ide-text-inverse);
  background-color: rgba(255, 255, 255, 0.3);
  font-weight: 700;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
}

.ide-tree-item.expanded > .tree-item-content {
  background-color: var(--ide-bg-highlight);
  border-color: var(--ide-accent-primary);
  box-shadow: 0 2px 8px rgba(124, 58, 237, 0.15);
  position: relative;
}

.ide-tree-item.expanded > .tree-item-content::after {
  content: '';
  position: absolute;
  bottom: -1px;
  left: 0;
  right: 0;
  height: 2px;
  background: linear-gradient(to right, var(--ide-accent-primary), transparent);
  border-radius: 1px;
}

.ide-tree-expand {
  color: var(--ide-text-primary);
  font-size: 12px;
  transition: all var(--ide-transition-fast);
  cursor: pointer;
  padding: 4px;
  border-radius: 3px;
  font-weight: bold;
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.3);
}

.ide-tree-expand:hover {
  color: var(--ide-accent-primary);
  background-color: var(--ide-bg-tertiary);
  transform: scale(1.1);
}

.ide-tree-expand.expanded {
  transform: rotate(90deg) scale(1.1);
  color: var(--ide-accent-primary);
}

.ide-tree-icon {
  width: 18px;
  height: 18px;
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 11px;
  color: white;
  font-weight: bold;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.3);
}

.database-explorer-children {
  margin-left: 20px;
  border-left: 2px solid var(--ide-border);
  padding-left: var(--ide-spacing-sm);
  margin-top: 6px;
  margin-bottom: 6px;
  position: relative;
  background-color: var(--ide-bg-primary);
  border-radius: var(--ide-border-radius);
  padding-top: 6px;
  padding-bottom: 6px;
}

.database-explorer-children::before {
  content: '';
  position: absolute;
  left: -2px;
  top: 0;
  bottom: 0;
  width: 2px;
  background: linear-gradient(to bottom, var(--ide-accent-primary), var(--ide-border));
  border-radius: 1px;
}

/* 测量值项样式 */
.database-explorer-item.measurement-item {
  margin-bottom: 3px;
  border-radius: var(--ide-border-radius);
  transition: all var(--ide-transition-fast);
  margin-left: 0;
  padding-left: 0;
}

.database-explorer-item.measurement-item .tree-item-content {
  background-color: var(--ide-bg-secondary);
  border: 1px solid var(--ide-border-light);
  margin: 1px 0;
  padding: var(--ide-spacing-xs) var(--ide-spacing-sm);
  border-radius: var(--ide-border-radius);
  transition: all var(--ide-transition-fast);
  margin-left: 0;
  padding-left: var(--ide-spacing-sm);
  position: relative;
}

.database-explorer-item.measurement-item .tree-item-content:hover {
  background-color: var(--ide-bg-hover);
  border-color: var(--ide-accent-primary);
  transform: translateX(2px);
  box-shadow: 0 2px 6px rgba(124, 58, 237, 0.15);
}

.database-explorer-item.measurement-item .tree-item-name {
  font-size: var(--ide-font-size-sm);
  color: var(--ide-text-primary);
  font-weight: 500;
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.2);
}

.database-explorer-item.measurement-item .ide-tree-icon {
  width: 14px;
  height: 14px;
  font-size: 9px;
  background-color: var(--ide-accent-orange);
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
}

.database-explorer-item.measurement-item.selected .tree-item-content {
  background-color: var(--ide-accent-primary);
  border-color: var(--ide-accent-secondary);
  box-shadow: 0 2px 6px rgba(124, 58, 237, 0.25);
}

.database-explorer-item.measurement-item.selected .tree-item-name {
  color: var(--ide-text-inverse);
  font-weight: 600;
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.3);
}

/* 数据库项样式 */
.database-explorer-item.database-item {
  margin-bottom: 2px;
  border-radius: var(--ide-border-radius);
  transition: all var(--ide-transition-fast);
}

.database-explorer-item.database-item .tree-item-content {
  background-color: var(--ide-bg-primary);
  border: 1px solid transparent;
  margin: 1px 0;
  padding: var(--ide-spacing-xs) var(--ide-spacing-sm);
  border-radius: var(--ide-border-radius);
  transition: all var(--ide-transition-fast);
  position: relative;
}

.database-explorer-item.database-item .tree-item-content:hover {
  background-color: var(--ide-bg-hover);
  border-color: var(--ide-border-light);
  transform: translateX(1px);
}

.database-explorer-item.database-item.expanded .tree-item-content {
  background-color: var(--ide-bg-highlight);
  border-color: var(--ide-accent-primary);
  box-shadow: 0 1px 4px rgba(124, 58, 237, 0.1);
}

.database-explorer-item.database-item.selected .tree-item-content {
  background-color: var(--ide-accent-primary);
  border-color: var(--ide-accent-secondary);
  box-shadow: 0 2px 6px rgba(124, 58, 237, 0.25);
}

.database-explorer-item.database-item.selected .tree-item-name {
  color: var(--ide-text-inverse);
  font-weight: 700;
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.3);
}

.database-explorer-item.database-item .ide-tree-icon {
  width: 16px;
  height: 16px;
  font-size: 10px;
  background-color: var(--ide-accent-primary);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

/* 空状态样式 */
.ide-empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: var(--ide-spacing-xl);
  color: var(--ide-text-secondary);
  text-align: center;
}

.ide-empty-icon {
  font-size: 48px;
  margin-bottom: var(--ide-spacing-md);
  opacity: 0.6;
}

.ide-empty-text {
  font-size: var(--ide-font-size-sm);
  margin-bottom: var(--ide-spacing-md);
}

/* 加载状态样式 */
.ide-loading {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: var(--ide-spacing-xl);
  color: var(--ide-text-secondary);
}

.ide-loading-spinner {
  width: 20px;
  height: 20px;
  border: 2px solid var(--ide-border);
  border-top: 2px solid var(--ide-accent-primary);
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin-bottom: var(--ide-spacing-sm);
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}
</style> 