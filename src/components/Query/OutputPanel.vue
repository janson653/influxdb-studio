<template>
  <div class="output-panel">
    <div class="output-header">
      <span class="output-title">Output</span>
      <div class="output-actions">
        <el-button 
          size="small" 
          @click="clearOutput"
          :disabled="outputLogs.length === 0"
        >
          清空
        </el-button>
        <el-button 
          size="small" 
          @click="copyOutput"
          :disabled="outputLogs.length === 0"
        >
          复制
        </el-button>
      </div>
    </div>
    
    <div class="output-content" ref="outputContentRef">
      <div v-if="outputLogs.length === 0" class="empty-output">
        <el-empty description="暂无输出信息" :image-size="60" />
      </div>
      
      <div v-else class="output-logs">
        <div 
          v-for="(log, index) in outputLogs" 
          :key="index"
          class="output-log-item"
          :class="getLogItemClass(log.type)"
        >
          <div class="log-header">
            <span class="log-timestamp">{{ formatTimestamp(log.timestamp) }}</span>
            <el-tag 
              :type="getLogTagType(log.type)" 
              size="small"
              class="log-type-tag"
            >
              {{ getLogTypeText(log.type) }}
            </el-tag>
          </div>
          
          <div class="log-message" :class="getLogMessageClass(log.type)">
            {{ log.message }}
          </div>
          
          <div v-if="log.details" class="log-details">
            <pre>{{ log.details }}</pre>
          </div>
          
          <div v-if="log.data" class="log-data">
            <el-collapse>
              <el-collapse-item title="详细信息" name="details">
                <pre>{{ JSON.stringify(log.data, null, 2) }}</pre>
              </el-collapse-item>
            </el-collapse>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, nextTick } from 'vue'
import { ElMessage } from 'element-plus'

// 日志类型定义
export interface OutputLog {
  type: 'info' | 'success' | 'warning' | 'error' | 'debug'
  message: string
  timestamp: Date
  details?: string
  data?: any
}

// Props
interface Props {
  logs: OutputLog[]
  autoScroll?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  logs: () => [],
  autoScroll: true
})

// 响应式数据
const outputContentRef = ref<HTMLElement>()

// 计算属性
const outputLogs = computed(() => props.logs)

// 方法
const getLogItemClass = (type: string) => {
  return {
    'log-item-info': type === 'info',
    'log-item-success': type === 'success',
    'log-item-warning': type === 'warning',
    'log-item-error': type === 'error',
    'log-item-debug': type === 'debug'
  }
}

const getLogTagType = (type: string) => {
  const typeMap: Record<string, string> = {
    info: 'info',
    success: 'success',
    warning: 'warning',
    error: 'danger',
    debug: ''
  }
  return typeMap[type] || 'info'
}

const getLogTypeText = (type: string) => {
  const typeMap: Record<string, string> = {
    info: '信息',
    success: '成功',
    warning: '警告',
    error: '错误',
    debug: '调试'
  }
  return typeMap[type] || '信息'
}

const getLogMessageClass = (type: string) => {
  return {
    'message-info': type === 'info',
    'message-success': type === 'success',
    'message-warning': type === 'warning',
    'message-error': type === 'error',
    'message-debug': type === 'debug'
  }
}

const formatTimestamp = (timestamp: Date) => {
  const timeString = timestamp.toLocaleTimeString('zh-CN', {
    hour12: false,
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit'
  })
  
  // 手动添加毫秒部分
  const milliseconds = timestamp.getMilliseconds().toString().padStart(3, '0')
  return `${timeString}.${milliseconds}`
}

const clearOutput = () => {
  // 触发清空事件
  emit('clear')
  ElMessage.success('输出已清空')
}

const copyOutput = async () => {
  try {
    const outputText = outputLogs.value
      .map(log => `[${formatTimestamp(log.timestamp)}] [${getLogTypeText(log.type)}] ${log.message}`)
      .join('\n')
    
    await navigator.clipboard.writeText(outputText)
    ElMessage.success('输出已复制到剪贴板')
  } catch (error) {
    ElMessage.error('复制失败')
  }
}

const scrollToBottom = async () => {
  if (props.autoScroll && outputContentRef.value) {
    await nextTick()
    outputContentRef.value.scrollTop = outputContentRef.value.scrollHeight
  }
}

// 监听日志变化，自动滚动到底部
watch(() => props.logs.length, () => {
  scrollToBottom()
})

// 事件
const emit = defineEmits<{
  clear: []
}>()
</script>

<style scoped>
.output-panel {
  display: flex;
  flex-direction: column;
  height: 100%;
  background-color: #1e1e1e;
  border-right: 1px solid #555;
}

.output-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 12px;
  background-color: #2d2d2d;
  border-bottom: 1px solid #555;
  flex-shrink: 0;
}

.output-title {
  font-weight: 600;
  color: #a9b7c6;
  font-size: 14px;
}

.output-actions {
  display: flex;
  gap: 8px;
}

.output-content {
  flex: 1;
  overflow-y: auto;
  padding: 8px;
}

.empty-output {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: #808080;
}

.output-logs {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.output-log-item {
  padding: 8px;
  border-radius: 4px;
  border-left: 3px solid transparent;
  background-color: #2b2b2b;
}

.log-item-info {
  border-left-color: #409eff;
}

.log-item-success {
  border-left-color: #67c23a;
}

.log-item-warning {
  border-left-color: #e6a23c;
}

.log-item-error {
  border-left-color: #f56c6c;
}

.log-item-debug {
  border-left-color: #909399;
}

.log-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 4px;
}

.log-timestamp {
  font-size: 12px;
  color: #808080;
  font-family: 'JetBrains Mono', monospace;
}

.log-type-tag {
  font-size: 10px;
}

.log-message {
  font-size: 13px;
  line-height: 1.4;
  word-break: break-word;
}

.message-info {
  color: #a9b7c6;
}

.message-success {
  color: #6a8759;
}

.message-warning {
  color: #ffc66d;
}

.message-error {
  color: #f56c6c;
}

.message-debug {
  color: #9876aa;
}

.log-details {
  margin-top: 8px;
  padding: 8px;
  background-color: #1e1e1e;
  border-radius: 3px;
  border: 1px solid #555;
}

.log-details pre {
  margin: 0;
  font-family: 'JetBrains Mono', monospace;
  font-size: 12px;
  color: #a9b7c6;
  white-space: pre-wrap;
  word-break: break-word;
}

.log-data {
  margin-top: 8px;
}

.log-data pre {
  margin: 0;
  font-family: 'JetBrains Mono', monospace;
  font-size: 11px;
  color: #a9b7c6;
  background-color: #1e1e1e;
  padding: 8px;
  border-radius: 3px;
  border: 1px solid #555;
  overflow-x: auto;
}

/* 滚动条样式 */
.output-content::-webkit-scrollbar {
  width: 8px;
}

.output-content::-webkit-scrollbar-track {
  background: #2b2b2b;
}

.output-content::-webkit-scrollbar-thumb {
  background: #555;
  border-radius: 4px;
}

.output-content::-webkit-scrollbar-thumb:hover {
  background: #666;
}
</style> 