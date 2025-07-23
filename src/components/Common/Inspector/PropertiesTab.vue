<template>
  <div class="properties-tab">
    <div v-if="loading" class="loading-container">
      <el-skeleton :rows="8" animated />
    </div>
    
    <div v-else-if="!object" class="empty-container">
      <el-empty description="请选择一个对象查看属性" />
    </div>
    
    <div v-else class="properties-content">
      <!-- 基本信息 -->
      <div class="info-section">
        <h3 class="section-title">基本信息</h3>
        <el-descriptions :column="2" border size="small">
          <el-descriptions-item label="名称">
            <code>{{ object.name }}</code>
          </el-descriptions-item>
          <el-descriptions-item label="类型">
            <el-tag :type="getTypeTagType(object.type)" size="small">
              {{ getTypeLabel(object.type) }}
            </el-tag>
          </el-descriptions-item>
          <el-descriptions-item v-if="object.database" label="数据库">
            <code>{{ object.database }}</code>
          </el-descriptions-item>
          <el-descriptions-item v-if="object.retentionPolicy" label="保留策略">
            <code>{{ object.retentionPolicy }}</code>
          </el-descriptions-item>
        </el-descriptions>
      </div>

      <!-- 统计信息 -->
      <div class="info-section">
        <h3 class="section-title">统计信息</h3>
        <el-row :gutter="16">
          <el-col :span="6" v-if="object.measurementCount !== undefined">
            <div class="stat-item">
              <div class="stat-value">{{ object.measurementCount }}</div>
              <div class="stat-label">measurement数量</div>
            </div>
          </el-col>
          <el-col :span="6" v-if="object.seriesCount !== undefined">
            <div class="stat-item">
              <div class="stat-value">{{ formatNumber(object.seriesCount) }}</div>
              <div class="stat-label">系列数量</div>
            </div>
          </el-col>
          <el-col :span="6" v-if="object.pointCount !== undefined">
            <div class="stat-item">
              <div class="stat-value">{{ formatNumber(object.pointCount) }}</div>
              <div class="stat-label">数据点数量</div>
            </div>
          </el-col>
          <el-col :span="6" v-if="object.fieldKeys">
            <div class="stat-item">
              <div class="stat-value">{{ object.fieldKeys.length }}</div>
              <div class="stat-label">字段数量</div>
            </div>
          </el-col>
          <el-col :span="6" v-if="object.tagKeys">
            <div class="stat-item">
              <div class="stat-value">{{ object.tagKeys.length }}</div>
              <div class="stat-label">标签数量</div>
            </div>
          </el-col>
        </el-row>
      </div>

      <!-- 时间信息 -->
      <div class="info-section" v-if="object.createdAt || object.updatedAt">
        <h3 class="section-title">时间信息</h3>
        <el-descriptions :column="2" border size="small">
          <el-descriptions-item v-if="object.createdAt" label="创建时间">
            <span class="timestamp">{{ formatTimestamp(object.createdAt) }}</span>
          </el-descriptions-item>
          <el-descriptions-item v-if="object.updatedAt" label="更新时间">
            <span class="timestamp">{{ formatTimestamp(object.updatedAt) }}</span>
          </el-descriptions-item>
          <el-descriptions-item v-if="object.lastWriteTime" label="最后写入">
            <span class="timestamp">{{ formatTimestamp(object.lastWriteTime) }}</span>
          </el-descriptions-item>
        </el-descriptions>
      </div>

      <!-- 配置信息 -->
      <div class="info-section" v-if="object.config">
        <h3 class="section-title">配置信息</h3>
        <el-descriptions :column="1" border size="small">
                      <el-descriptions-item 
              v-for="(value, key) in object.config" 
              :key="key" 
              :label="formatConfigKey(String(key))"
            >
            <code class="config-value">{{ formatConfigValue(value) }}</code>
          </el-descriptions-item>
        </el-descriptions>
      </div>

      <!-- 权限信息 -->
      <div class="info-section" v-if="object.permissions">
        <h3 class="section-title">权限信息</h3>
        <el-table :data="object.permissions" style="width: 100%" size="small" border>
          <el-table-column prop="user" label="用户" width="150" />
          <el-table-column prop="role" label="角色" width="120" />
          <el-table-column prop="permissions" label="权限">
            <template #default="{ row }">
              <el-tag 
                v-for="perm in row.permissions" 
                :key="perm"
                size="small"
                class="permission-tag"
              >
                {{ perm }}
              </el-tag>
            </template>
          </el-table-column>
        </el-table>
      </div>

      <!-- 标签信息 -->
      <div class="info-section" v-if="object.tags && Object.keys(object.tags).length">
        <h3 class="section-title">标签</h3>
        <div class="tags-container">
          <el-tag 
            v-for="(value, key) in object.tags" 
            :key="key"
            size="small"
            class="object-tag"
          >
            {{ key }}: {{ value }}
          </el-tag>
        </div>
      </div>

      <!-- 原始数据 -->
      <div class="info-section">
        <h3 class="section-title">
          原始数据
          <el-button 
            size="small" 
            type="primary" 
            @click="copyRawData"
            :icon="Document"
          >
            复制JSON
          </el-button>
        </h3>
        <div class="raw-data-container">
          <pre class="raw-data">{{ formatRawData(object) }}</pre>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">

import { ElMessage } from 'element-plus'
import { Document } from '@element-plus/icons-vue'

interface Props {
  object: any
  loading?: boolean
}

const props = defineProps<Props>()

const getTypeTagType = (type: string) => {
  const typeMap: Record<string, string> = {
    'database': 'success',
    'measurement': 'warning',
    'series': 'info',
    'field': 'primary'
  }
  return typeMap[type] || 'info'
}

const getTypeLabel = (type: string) => {
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

const formatTimestamp = (timestamp: string | number) => {
  if (!timestamp) return '未知'
  
  const date = new Date(timestamp)
  return date.toLocaleString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit'
  })
}

const formatConfigKey = (key: string) => {
  return key.replace(/([A-Z])/g, ' $1')
    .replace(/^./, str => str.toUpperCase())
}

const formatConfigValue = (value: any) => {
  if (typeof value === 'object') {
    return JSON.stringify(value, null, 2)
  }
  return value?.toString() || 'null'
}

const formatRawData = (obj: any) => {
  return JSON.stringify(obj, null, 2)
}

const copyRawData = async () => {
  try {
    await navigator.clipboard.writeText(formatRawData(props.object))
    ElMessage.success('JSON数据已复制到剪贴板')
  } catch (error) {
    ElMessage.error('复制失败')
  }
}
</script>

<style scoped>
.properties-tab {
  padding: 16px;
  height: 100%;
  overflow-y: auto;
}

.loading-container,
.empty-container {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 300px;
}

.properties-content {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.info-section {
  border: 1px solid #e4e7ed;
  border-radius: 6px;
  padding: 16px;
  background-color: #fafafa;
}

.section-title {
  margin: 0 0 16px 0;
  font-size: 16px;
  font-weight: 600;
  color: #303133;
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.stat-item {
  text-align: center;
  padding: 16px;
  background-color: white;
  border: 1px solid #e4e7ed;
  border-radius: 6px;
}

.stat-value {
  font-size: 24px;
  font-weight: bold;
  color: #409eff;
  margin-bottom: 8px;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
}

.stat-label {
  font-size: 12px;
  color: #606266;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.timestamp {
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: 12px;
  color: #606266;
}

.config-value {
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: 12px;
  background-color: #f0f0f0;
  padding: 2px 6px;
  border-radius: 3px;
  white-space: pre-wrap;
  word-break: break-all;
}

.permission-tag {
  margin-right: 4px;
  margin-bottom: 4px;
}

.tags-container {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.object-tag {
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: 11px;
}

.raw-data-container {
  background-color: #1e1e1e;
  border-radius: 6px;
  padding: 16px;
  overflow-x: auto;
}

.raw-data {
  color: #d4d4d4;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: 12px;
  line-height: 1.5;
  margin: 0;
  white-space: pre-wrap;
  word-break: break-all;
}

:deep(.el-descriptions__label) {
  font-weight: 600;
  color: #606266;
}

:deep(.el-descriptions__content) {
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
}

:deep(.el-table) {
  font-size: 12px;
}

:deep(.el-table th) {
  background-color: #f5f7fa;
  font-weight: 600;
}
</style> 