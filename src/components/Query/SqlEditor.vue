<template>
  <div class="sql-editor-container">
    <!-- 编辑器标签页 -->
    <div class="editor-tabs">
      <button 
        v-for="tab in editorTabs" 
        :key="tab.id"
        class="editor-tab" 
        :class="{ active: activeTab === tab.id }"
        @click="setActiveTab(tab.id)"
      >
        <span class="icon icon-database"></span>
        {{ tab.name }}
        <button 
          v-if="tab.id !== 'console'"
          class="tab-close" 
          @click.stop="closeTab(tab.id)"
        >
          ×
        </button>
      </button>
      <button class="editor-tab new-tab" @click="createNewTab">
        <span>+</span>
      </button>
    </div>
    
    <!-- 编辑器工具栏 -->
    <div class="editor-toolbar">
      <div class="editor-controls">
        <button 
          class="editor-btn primary"
          @click="executeQuery"
          :disabled="!canExecute || isExecuting"
        >
          <span class="icon icon-play"></span>
          {{ isExecuting ? '执行中...' : '运行' }}
        </button>
        <button class="editor-btn" @click="formatQuery">
          <span class="icon icon-settings"></span>
          格式化
        </button>
        <button class="editor-btn" @click="clearQuery">
          <span>🗑</span>
          清空
        </button>
      </div>
      
      <div class="editor-status">
        <span>数据库: {{ selectedDatabase || '未选择' }}</span>
        <span v-if="selectedMeasurement">表: {{ selectedMeasurement }}</span>
        <span>行: {{ currentLine }}</span>
        <span>列: {{ currentColumn }}</span>
      </div>
    </div>
    
    <!-- Monaco编辑器 -->
    <div class="monaco-editor-container" ref="editorContainer">
      <div v-if="!monacoLoaded" class="editor-loading">
        <div class="loading-spinner"></div>
        <span>正在加载编辑器...</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch, nextTick } from 'vue'
import { ElMessage } from 'element-plus'
import { useQueryStore } from '../../stores/queryStore'
import { useConnectionStore } from '../../stores/connectionStore'
import { loadMonaco } from '../../utils/monacoLoader'
import { QueryValidator } from '../../utils/queryValidator'
import { InfluxDBVersion } from '../../types/influxdb'

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
  'query-executed': [result: any]
}>()

// Stores
const queryStore = useQueryStore()
const connectionStore = useConnectionStore()

// 响应式数据
const editorContainer = ref<HTMLElement>()
const monacoLoaded = ref(false)
const isExecuting = ref(false)
const activeTab = ref('console')
const currentLine = ref(1)
const currentColumn = ref(1)

// 编辑器标签页
const editorTabs = ref([
  { id: 'console', name: 'console', content: '' }
])

// 计算属性
const canExecute = computed(() => {
  const activeTabData = editorTabs.value.find(tab => tab.id === activeTab.value)
  return activeTabData && activeTabData.content.trim() && connectionStore.isConnected
})

const currentQuery = computed(() => {
  const activeTabData = editorTabs.value.find(tab => tab.id === activeTab.value)
  return activeTabData?.content || ''
})

// 方法
const setActiveTab = (tabId: string) => {
  activeTab.value = tabId
}

const createNewTab = () => {
  const newTabId = `query_${Date.now()}`
  editorTabs.value.push({
    id: newTabId,
    name: `query_${editorTabs.value.length}`,
    content: ''
  })
  setActiveTab(newTabId)
}

const closeTab = (tabId: string) => {
  const index = editorTabs.value.findIndex(tab => tab.id === tabId)
  if (index > -1) {
    editorTabs.value.splice(index, 1)
    if (activeTab.value === tabId) {
      setActiveTab(editorTabs.value[0]?.id || 'console')
    }
  }
}

const executeQuery = async () => {
  if (!canExecute.value) return
  
  const query = currentQuery.value.trim()
  if (!query) {
    ElMessage.warning('请输入查询语句')
    return
  }
  
  if (!props.selectedDatabase) {
    ElMessage.warning('请先选择数据库')
    return
  }
  
  try {
    isExecuting.value = true
    
    // 验证查询语法
    const activeConnection = connectionStore.activeConnectionConfig
    const validation = QueryValidator.validateQuery(
      query, 
      activeConnection?.version || InfluxDBVersion.V1
    )
    
    if (!validation.isValid) {
      ElMessage.error(`查询语法错误: ${validation.error}`)
      return
    }
    
    // 执行查询
    const result = await queryStore.executeQuery(
      query,
      props.selectedDatabase,
      connectionStore.activeConnectionId || ''
    )
    
    if (result) {
      emit('query-executed', result)
      ElMessage.success('查询执行成功')
    }
  } catch (error: any) {
    ElMessage.error(`查询执行失败: ${error.message || error}`)
  } finally {
    isExecuting.value = false
  }
}

const formatQuery = () => {
  // 简单的SQL格式化
  const query = currentQuery.value
  const formatted = query
    .replace(/\s+/g, ' ')
    .replace(/\s*([,()])\s*/g, '$1 ')
    .replace(/\s*(SELECT|FROM|WHERE|GROUP BY|ORDER BY|LIMIT|OFFSET)\s+/gi, '\n$1 ')
    .trim()
  
  updateCurrentTabContent(formatted)
}

const clearQuery = () => {
  updateCurrentTabContent('')
}

const updateCurrentTabContent = (content: string) => {
  const activeTabData = editorTabs.value.find(tab => tab.id === activeTab.value)
  if (activeTabData) {
    activeTabData.content = content
  }
}

// Monaco编辑器相关
let monaco: any = null
let editor: any = null

const initializeMonaco = async () => {
  try {
    monaco = await loadMonaco()
    monacoLoaded.value = true
    
    await nextTick()
    
    if (editorContainer.value) {
      editor = monaco.editor.create(editorContainer.value, {
        value: currentQuery.value,
        language: 'sql',
        theme: 'vs-dark',
        automaticLayout: true,
        minimap: { enabled: false },
        scrollBeyondLastLine: false,
        fontSize: 14,
        fontFamily: 'JetBrains Mono, Fira Code, Consolas, monospace',
        lineNumbers: 'on',
        roundedSelection: false,
        scrollbar: {
          vertical: 'visible',
          horizontal: 'visible'
        }
      })
      
      // 监听内容变化
      editor.onDidChangeModelContent(() => {
        const value = editor.getValue()
        updateCurrentTabContent(value)
      })
      
      // 监听光标位置变化
      editor.onDidChangeCursorPosition((e: any) => {
        currentLine.value = e.position.lineNumber
        currentColumn.value = e.position.column
      })
    }
  } catch (error) {
    console.error('Monaco编辑器初始化失败:', error)
    ElMessage.error('编辑器加载失败')
  }
}

// 监听标签页切换
watch(activeTab, (newTabId) => {
  if (editor) {
    const activeTabData = editorTabs.value.find(tab => tab.id === newTabId)
    if (activeTabData) {
      editor.setValue(activeTabData.content)
    }
  }
})

// 监听选中数据库变化，自动生成查询模板
watch(() => props.selectedMeasurement, (newMeasurement) => {
  if (newMeasurement && !currentQuery.value.trim()) {
    const template = `SELECT * FROM "${newMeasurement}" LIMIT 100`
    updateCurrentTabContent(template)
    if (editor) {
      editor.setValue(template)
    }
  }
})

// 生命周期
onMounted(() => {
  initializeMonaco()
})

onUnmounted(() => {
  if (editor) {
    editor.dispose()
  }
})
</script>

<style scoped>
.sql-editor-container {
  display: flex;
  flex-direction: column;
  height: 100%;
  background-color: var(--ide-bg-primary);
}

.editor-tabs {
  height: 35px;
  background-color: var(--ide-bg-secondary);
  border-bottom: 1px solid var(--ide-border);
  display: flex;
  align-items: center;
}

.editor-tab {
  padding: 8px 15px;
  background-color: var(--ide-bg-tertiary);
  border: none;
  color: var(--ide-text-primary);
  cursor: pointer;
  border-right: 1px solid var(--ide-border);
  display: flex;
  align-items: center;
  gap: 5px;
  font-size: 13px;
  position: relative;
}

.editor-tab.active {
  background-color: var(--ide-bg-primary);
  border-bottom: 2px solid var(--ide-accent-primary);
}

.editor-tab.new-tab {
  background-color: transparent;
  border: none;
  font-size: 16px;
  padding: 8px 10px;
}

.tab-close {
  background: none;
  border: none;
  color: var(--ide-text-secondary);
  cursor: pointer;
  font-size: 12px;
  padding: 0;
  margin-left: 5px;
}

.tab-close:hover {
  color: var(--ide-text-primary);
}

.editor-toolbar {
  height: 35px;
  background-color: var(--ide-bg-secondary);
  border-bottom: 1px solid var(--ide-border);
  display: flex;
  align-items: center;
  padding: 0 10px;
  gap: 10px;
}

.editor-controls {
  display: flex;
  gap: 5px;
}

.editor-btn {
  padding: 5px 10px;
  background-color: transparent;
  border: none;
  color: var(--ide-text-primary);
  cursor: pointer;
  border-radius: var(--ide-border-radius);
  display: flex;
  align-items: center;
  gap: 5px;
  font-size: 12px;
}

.editor-btn:hover:not(:disabled) {
  background-color: var(--ide-bg-tertiary);
}

.editor-btn.primary {
  background-color: var(--ide-accent-primary);
  color: white;
}

.editor-btn.primary:hover:not(:disabled) {
  background-color: var(--ide-accent-secondary);
}

.editor-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.editor-status {
  margin-left: auto;
  display: flex;
  gap: 10px;
  font-size: 12px;
  color: var(--ide-text-secondary);
}

.monaco-editor-container {
  flex: 1;
  position: relative;
}

.editor-loading {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 10px;
  color: var(--ide-text-secondary);
}

.loading-spinner {
  width: 20px;
  height: 20px;
  border: 2px solid var(--ide-border);
  border-top: 2px solid var(--ide-accent-primary);
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.icon {
  width: 16px;
  height: 16px;
  display: inline-block;
  background-size: contain;
  background-repeat: no-repeat;
  background-position: center;
}

.icon-database { background-color: var(--ide-accent-primary); border-radius: 2px; }
.icon-play { background-color: var(--ide-accent-primary); border-radius: 2px; }
.icon-settings { background-color: var(--ide-accent-purple); border-radius: 2px; }
</style> 