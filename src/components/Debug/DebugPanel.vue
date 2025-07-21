<template>
  <div class="debug-panel" v-if="isVisible">
    <div class="debug-header">
      <h3>调试面板</h3>
      <div class="debug-actions">
        <el-button size="small" @click="clearLogs">清空日志</el-button>
        <el-button size="small" @click="exportLogs">导出日志</el-button>
        <el-button size="small" @click="closePanel">关闭</el-button>
      </div>
    </div>
    
    <el-tabs v-model="activeTab" class="debug-tabs">
      <el-tab-pane label="日志" name="logs">
        <div class="logs-container">
          <div 
            v-for="log in logs" 
            :key="log.timestamp" 
            :class="['log-entry', `log-${log.level}`]"
          >
            <div class="log-header">
              <span class="log-time">{{ formatTime(log.timestamp) }}</span>
              <span :class="['log-level', `level-${log.level}`]">{{ log.level.toUpperCase() }}</span>
            </div>
            <div class="log-message">{{ log.message }}</div>
            <div v-if="log.data" class="log-data">
              <pre>{{ JSON.stringify(log.data, null, 2) }}</pre>
            </div>
          </div>
        </div>
      </el-tab-pane>
      
      <el-tab-pane label="状态" name="status">
        <div class="status-container">
          <el-descriptions title="连接状态" :column="1" border>
            <el-descriptions-item label="活跃连接">
              {{ connectionStatus.hasActiveConnection ? '是' : '否' }}
            </el-descriptions-item>
            <el-descriptions-item label="连接数量">
              {{ connectionStatus.connections?.length || 0 }}
            </el-descriptions-item>
            <el-descriptions-item label="连接状态">
              <el-tag :type="getConnectionStatusType()">
                {{ getConnectionStatusText() }}
              </el-tag>
            </el-descriptions-item>
          </el-descriptions>
          
          <el-descriptions title="数据库树状态" :column="1" border style="margin-top: 20px;">
            <el-descriptions-item label="数据库数量">
              {{ databaseTreeStatus.treeDataLength }}
            </el-descriptions-item>
            <el-descriptions-item label="数据库列表">
              <div v-for="db in databaseTreeStatus.treeData" :key="db.name">
                {{ db.name }} ({{ db.childrenCount }} 个子项)
              </div>
            </el-descriptions-item>
          </el-descriptions>
        </div>
      </el-tab-pane>
    </el-tabs>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { ElMessage } from 'element-plus'
import { debugHelper, type DebugInfo } from '../../utils/debugHelper'
import { useConnectionStore } from '../../stores/connectionStore'

const props = defineProps<{
  isVisible: boolean
}>()

const emit = defineEmits<{
  close: []
}>()

const connectionStore = useConnectionStore()
const activeTab = ref('logs')

const logs = computed(() => debugHelper.getLogs())

const connectionStatus = computed(() => {
  return {
    hasActiveConnection: !!connectionStore.activeConnectionConfig,
    connections: connectionStore.connections,
    connectionStatus: connectionStore.connectionStatus
  }
})

const databaseTreeStatus = computed(() => {
  // 这里需要从父组件传入数据库树数据
  return {
    treeDataLength: 0,
    treeData: []
  }
})

const getConnectionStatusType = () => {
  const status = connectionStore.connectionStatus[connectionStore.activeConnectionConfig?.id || '']
  if (!status) return 'info'
  return { connected: 'success', connecting: 'warning', error: 'danger', disconnected: 'info' }[status.status] || 'info'
}

const getConnectionStatusText = () => {
  const status = connectionStore.connectionStatus[connectionStore.activeConnectionConfig?.id || '']
  if (!status) return '未连接'
  return { connected: '已连接', connecting: '连接中', error: '错误', disconnected: '未连接' }[status.status] || '未连接'
}

const formatTime = (timestamp: string) => {
  return new Date(timestamp).toLocaleTimeString()
}

const clearLogs = () => {
  debugHelper.clear()
  ElMessage.success('日志已清空')
}

const exportLogs = () => {
  const debugInfo = debugHelper.exportDebugInfo()
  const blob = new Blob([JSON.stringify(debugInfo, null, 2)], { type: 'application/json' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = `debug-logs-${new Date().toISOString().slice(0, 19)}.json`
  a.click()
  URL.revokeObjectURL(url)
  ElMessage.success('日志已导出')
}

const closePanel = () => {
  emit('close')
}
</script>

<style scoped>
.debug-panel {
  position: fixed;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  width: 80vw;
  height: 80vh;
  background: #2b2b2b;
  border: 1px solid #555;
  border-radius: 8px;
  z-index: 9999;
  display: flex;
  flex-direction: column;
}

.debug-header {
  padding: 15px;
  border-bottom: 1px solid #555;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.debug-header h3 {
  margin: 0;
  color: #fff;
}

.debug-actions {
  display: flex;
  gap: 10px;
}

.debug-tabs {
  flex: 1;
  display: flex;
  flex-direction: column;
}

.logs-container {
  height: 60vh;
  overflow-y: auto;
  padding: 10px;
}

.log-entry {
  margin-bottom: 10px;
  padding: 10px;
  border-radius: 4px;
  border-left: 4px solid #666;
}

.log-entry.log-info {
  background: #1e3a5f;
  border-left-color: #409eff;
}

.log-entry.log-warn {
  background: #5c3a1e;
  border-left-color: #e6a23c;
}

.log-entry.log-error {
  background: #5c1e1e;
  border-left-color: #f56c6c;
}

.log-header {
  display: flex;
  justify-content: space-between;
  margin-bottom: 5px;
  font-size: 12px;
}

.log-time {
  color: #999;
}

.log-level {
  font-weight: bold;
}

.level-info { color: #409eff; }
.level-warn { color: #e6a23c; }
.level-error { color: #f56c6c; }

.log-message {
  color: #fff;
  margin-bottom: 5px;
}

.log-data {
  background: #1a1a1a;
  padding: 10px;
  border-radius: 4px;
  font-size: 12px;
  color: #ccc;
}

.log-data pre {
  margin: 0;
  white-space: pre-wrap;
  word-break: break-all;
}

.status-container {
  padding: 20px;
}
</style> 