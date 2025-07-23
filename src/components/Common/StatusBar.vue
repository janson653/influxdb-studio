<template>
  <div class="status-bar">
    <div class="status-left">
      <!-- 连接状态 -->
      <div class="status-item">
        <el-icon :class="getConnectionIconClass()">
          <component :is="getConnectionIcon()" />
        </el-icon>
        <span class="status-text">{{ getConnectionStatusText() }}</span>
      </div>

      <!-- 连接信息 -->
      <div v-if="connection" class="status-item">
        <span class="status-label">连接:</span>
        <span class="status-value">{{ connection.name }}</span>
        <el-tag :type="getVersionTagType(connection.version)" size="small">
          {{ connection.version }}
        </el-tag>
      </div>

      <!-- 当前数据库 -->
      <div v-if="currentDatabase" class="status-item">
        <span class="status-label">数据库:</span>
        <span class="status-value">{{ currentDatabase }}</span>
      </div>

      <!-- 当前对象 -->
      <div v-if="currentObject" class="status-item">
        <span class="status-label">对象:</span>
        <span class="status-value">{{ currentObject.name }}</span>
        <el-tag :type="getObjectTypeTagType(currentObject.type)" size="small">
          {{ getObjectTypeLabel(currentObject.type) }}
        </el-tag>
      </div>
    </div>

    <div class="status-right">
      <!-- 统计信息 -->
      <div v-if="stats" class="status-item">
        <span class="status-label">数据库:</span>
        <span class="status-value">{{ stats.databaseCount || 0 }}</span>
        <span class="status-separator">|</span>
        <span class="status-label">Measurement:</span>
        <span class="status-value">{{ stats.measurementCount || 0 }}</span>
        <span class="status-separator">|</span>
        <span class="status-label">系列:</span>
        <span class="status-value">{{ formatNumber(stats.seriesCount || 0) }}</span>
        <span class="status-separator">|</span>
        <span class="status-label">数据点:</span>
        <span class="status-value">{{ formatNumber(stats.pointCount || 0) }}</span>
      </div>

      <!-- 加载状态 -->
      <div v-if="loading" class="status-item">
        <el-icon class="is-loading">
          <Loading />
        </el-icon>
        <span class="status-text">加载中...</span>
      </div>

      <!-- 错误状态 -->
      <div v-if="error" class="status-item error">
        <el-icon>
          <Warning />
        </el-icon>
        <span class="status-text">{{ error }}</span>
      </div>

      <!-- 时间戳 -->
      <div class="status-item">
        <span class="status-text">{{ currentTime }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { 
  Connection, 
  Loading, 
  Warning, 
  CircleCheck, 
  CircleClose 
} from '@element-plus/icons-vue'
import { InfluxDBVersion } from '../../types/influxdb'

interface Props {
  connection?: any
  currentDatabase?: string
  currentObject?: any
  stats?: {
    databaseCount?: number
    measurementCount?: number
    seriesCount?: number
    pointCount?: number
  }
  loading?: boolean
  error?: string
}

const props = defineProps<Props>()

// 响应式数据
const currentTime = ref('')

// 计算属性
const getConnectionIcon = () => {
  if (props.loading) return Loading
  if (props.error) return CircleClose
  if (props.connection) return CircleCheck
  return Connection
}

const getConnectionIconClass = () => {
  if (props.loading) return 'status-icon loading'
  if (props.error) return 'status-icon error'
  if (props.connection) return 'status-icon success'
  return 'status-icon'
}

const getConnectionStatusText = () => {
  if (props.loading) return '连接中...'
  if (props.error) return '连接错误'
  if (props.connection) return '已连接'
  return '未连接'
}

const getVersionTagType = (version: string) => {
  switch (version) {
    case InfluxDBVersion.V1: return 'info'
    case InfluxDBVersion.V2: return 'warning'
    case InfluxDBVersion.V3: return 'success'
    default: return 'info'
  }
}

const getObjectTypeTagType = (type: string) => {
  const typeMap: Record<string, string> = {
    'database': 'success',
    'measurement': 'warning',
    'series': 'info',
    'field': 'primary'
  }
  return typeMap[type] || 'info'
}

const getObjectTypeLabel = (type: string) => {
  const labelMap: Record<string, string> = {
    'database': '数据库',
    'measurement': 'Measurement',
    'series': '系列',
    'field': '字段'
  }
  return labelMap[type] || type
}

const formatNumber = (num: number) => {
  if (num >= 1000000) {
    return (num / 1000000).toFixed(1) + 'M'
  } else if (num >= 1000) {
    return (num / 1000).toFixed(1) + 'K'
  }
  return num.toString()
}

// 更新时间
const updateTime = () => {
  const now = new Date()
  currentTime.value = now.toLocaleString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit'
  })
}

let timeInterval: number

onMounted(() => {
  updateTime()
  timeInterval = setInterval(updateTime, 1000)
})

onUnmounted(() => {
  if (timeInterval) {
    clearInterval(timeInterval)
  }
})
</script>

<style scoped>
.status-bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 16px;
  background-color: #f5f7fa;
  border-top: 1px solid #e4e7ed;
  font-size: 12px;
  color: #606266;
  height: 32px;
  box-sizing: border-box;
}

.status-left,
.status-right {
  display: flex;
  align-items: center;
  gap: 16px;
}

.status-item {
  display: flex;
  align-items: center;
  gap: 4px;
}

.status-icon {
  font-size: 14px;
}

.status-icon.success {
  color: #67c23a;
}

.status-icon.error {
  color: #f56c6c;
}

.status-icon.loading {
  color: #409eff;
  animation: rotating 2s linear infinite;
}

.status-text {
  font-weight: 500;
}

.status-label {
  color: #909399;
}

.status-value {
  font-weight: 500;
  color: #303133;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
}

.status-separator {
  color: #c0c4cc;
  margin: 0 4px;
}

.error {
  color: #f56c6c;
}

@keyframes rotating {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

:deep(.el-tag) {
  margin-left: 4px;
}
</style> 