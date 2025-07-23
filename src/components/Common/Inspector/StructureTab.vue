<template>
  <div class="structure-tab">
    <div v-if="loading" class="loading-container">
      <el-skeleton :rows="6" animated />
    </div>
    
    <div v-else-if="!measurement" class="empty-container">
      <el-empty description="请选择一个measurement查看结构" />
    </div>
    
    <div v-else class="structure-content">
      <!-- 基本信息 -->
      <div class="info-section">
        <h3 class="section-title">基本信息</h3>
        <el-descriptions :column="2" border size="small">
          <el-descriptions-item label="名称">
            <code>{{ measurement.name }}</code>
          </el-descriptions-item>
          <el-descriptions-item label="数据库">
            <code>{{ measurement.database }}</code>
          </el-descriptions-item>
          <el-descriptions-item label="字段数量">
            {{ measurement.fieldKeys?.length || 0 }}
          </el-descriptions-item>
          <el-descriptions-item label="标签数量">
            {{ measurement.tagKeys?.length || 0 }}
          </el-descriptions-item>
        </el-descriptions>
      </div>

      <!-- 字段结构 -->
      <div class="info-section">
        <h3 class="section-title">
          字段结构
          <el-tag size="small" type="info">{{ measurement.fieldKeys?.length || 0 }} 个字段</el-tag>
        </h3>
        
        <div v-if="!measurement.fieldKeys?.length" class="no-data">
          <el-empty description="暂无字段数据" :image-size="60" />
        </div>
        
        <el-table 
          v-else 
          :data="measurement.fieldKeys" 
          style="width: 100%"
          size="small"
          :show-header="true"
          border
        >
          <el-table-column prop="key" label="字段名" min-width="150">
            <template #default="{ row }">
              <code class="field-name">{{ row.key }}</code>
            </template>
          </el-table-column>
          
          <el-table-column prop="type" label="数据类型" width="120">
            <template #default="{ row }">
              <el-tag 
                :type="getTypeTagType(row.type)" 
                size="small"
                class="type-tag"
              >
                {{ row.type }}
              </el-tag>
            </template>
          </el-table-column>
          
          <el-table-column label="示例值" min-width="200">
            <template #default="{ row }">
              <div class="sample-values">
                <el-tag 
                  v-for="(value, index) in row.sampleValues?.slice(0, 3)" 
                  :key="index"
                  size="small"
                  class="sample-tag"
                >
                  {{ formatSampleValue(value, row.type) }}
                </el-tag>
                <span v-if="row.sampleValues?.length > 3" class="more-indicator">
                  +{{ row.sampleValues.length - 3 }} 更多
                </span>
              </div>
            </template>
          </el-table-column>
        </el-table>
      </div>

      <!-- 标签结构 -->
      <div class="info-section">
        <h3 class="section-title">
          标签结构
          <el-tag size="small" type="info">{{ measurement.tagKeys?.length || 0 }} 个标签</el-tag>
        </h3>
        
        <div v-if="!measurement.tagKeys?.length" class="no-data">
          <el-empty description="暂无标签数据" :image-size="60" />
        </div>
        
        <el-table 
          v-else 
          :data="measurement.tagKeys" 
          style="width: 100%"
          size="small"
          :show-header="true"
          border
        >
          <el-table-column prop="key" label="标签键" min-width="150">
            <template #default="{ row }">
              <code class="tag-key">{{ row.key }}</code>
            </template>
          </el-table-column>
          
          <el-table-column label="标签值" min-width="300">
            <template #default="{ row }">
              <div class="tag-values">
                <el-tag 
                  v-for="(value, index) in row.values?.slice(0, 5)" 
                  :key="index"
                  size="small"
                  class="tag-value"
                >
                  {{ value }}
                </el-tag>
                <span v-if="row.values?.length > 5" class="more-indicator">
                  +{{ row.values.length - 5 }} 更多
                </span>
              </div>
            </template>
          </el-table-column>
          
          <el-table-column label="数量" width="80" align="center">
            <template #default="{ row }">
              <el-tag size="small" type="info">
                {{ row.values?.length || 0 }}
              </el-tag>
            </template>
          </el-table-column>
        </el-table>
      </div>

      <!-- 统计信息 -->
      <div class="info-section">
        <h3 class="section-title">统计信息</h3>
        <el-row :gutter="20">
          <el-col :span="8">
            <el-card shadow="never" class="stat-card">
              <div class="stat-value">{{ measurement.seriesCount || 0 }}</div>
              <div class="stat-label">系列数量</div>
            </el-card>
          </el-col>
          <el-col :span="8">
            <el-card shadow="never" class="stat-card">
              <div class="stat-value">{{ formatNumber(measurement.pointCount || 0) }}</div>
              <div class="stat-label">数据点数量</div>
            </el-card>
          </el-col>
          <el-col :span="8">
            <el-card shadow="never" class="stat-card">
              <div class="stat-value">{{ measurement.retentionPolicy || 'default' }}</div>
              <div class="stat-label">保留策略</div>
            </el-card>
          </el-col>
        </el-row>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
interface Props {
  measurement: any
  loading?: boolean
}

defineProps<Props>()

const getTypeTagType = (type: string) => {
  const typeMap: Record<string, string> = {
    'float': 'success',
    'integer': 'warning',
    'string': 'info',
    'boolean': 'danger'
  }
  return typeMap[type] || 'info'
}

const formatSampleValue = (value: any, type: string) => {
  if (value === null || value === undefined) {
    return 'null'
  }
  
  switch (type) {
    case 'string':
      return `"${value}"`
    case 'boolean':
      return value ? 'true' : 'false'
    case 'float':
    case 'integer':
      return value.toString()
    default:
      return value.toString()
  }
}

const formatNumber = (num: number) => {
  if (num >= 1000000) {
    return (num / 1000000).toFixed(1) + 'M'
  } else if (num >= 1000) {
    return (num / 1000).toFixed(1) + 'K'
  }
  return num.toString()
}
</script>

<style scoped>
.structure-tab {
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

.structure-content {
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
  gap: 8px;
}

.no-data {
  display: flex;
  justify-content: center;
  padding: 40px 0;
}

.field-name,
.tag-key {
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  background-color: #f0f0f0;
  padding: 2px 6px;
  border-radius: 3px;
  font-size: 12px;
}

.type-tag {
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: 11px;
}

.sample-values,
.tag-values {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
  align-items: center;
}

.sample-tag,
.tag-value {
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: 11px;
  max-width: 120px;
  overflow: hidden;
  text-overflow: ellipsis;
}

.more-indicator {
  font-size: 11px;
  color: #909399;
  font-style: italic;
}

.stat-card {
  text-align: center;
  border: 1px solid #e4e7ed;
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

:deep(.el-table) {
  font-size: 12px;
}

:deep(.el-table th) {
  background-color: #f5f7fa;
  font-weight: 600;
}

:deep(.el-descriptions__label) {
  font-weight: 600;
  color: #606266;
}

:deep(.el-descriptions__content) {
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
}
</style> 