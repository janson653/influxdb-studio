<template>
  <div class="ddl-tab">
    <div v-if="loading" class="loading-container">
      <el-skeleton :rows="6" animated />
    </div>
    
    <div v-else-if="!object" class="empty-container">
      <el-empty description="请选择一个对象查看DDL" />
    </div>
    
    <div v-else class="ddl-content">
      <!-- 工具栏 -->
      <div class="toolbar">
        <div class="toolbar-left">
          <el-button 
            size="small" 
            @click="copyDDL" 
            :icon="Document"
            :disabled="!ddlScript"
          >
            复制DDL
          </el-button>
          <el-button 
            size="small" 
            @click="copyQuery" 
            :icon="Document"
            :disabled="!queryScript"
          >
            复制查询
          </el-button>
          <el-button 
            size="small" 
            @click="openInQueryEditor" 
            :icon="Edit"
            :disabled="!queryScript"
          >
            在查询编辑器中打开
          </el-button>
        </div>
        
        <div class="toolbar-right">
          <el-select 
            v-model="selectedScriptType" 
            size="small" 
            style="width: 150px;"
          >
            <el-option label="DDL脚本" value="ddl" />
            <el-option label="查询脚本" value="query" />
            <el-option label="插入脚本" value="insert" />
          </el-select>
        </div>
      </div>

      <!-- 脚本内容 -->
      <div class="script-container">
        <div class="script-header">
          <h3 class="script-title">{{ getScriptTitle() }}</h3>
          <div class="script-actions">
            <el-button 
              size="small" 
              @click="formatScript" 
            >
              格式化
            </el-button>
            <el-button 
              size="small" 
              @click="toggleSyntaxHighlighting" 
              :icon="View"
            >
              {{ syntaxHighlighting ? '关闭语法高亮' : '开启语法高亮' }}
            </el-button>
          </div>
        </div>
        
        <div class="script-content">
          <MonacoEditor
            v-if="syntaxHighlighting"
            v-model="currentScript"
            :language="getLanguage()"
            :options="editorOptions"
            :height="400"
            @change="handleScriptChange"
          />
          <pre v-else class="plain-script">{{ currentScript }}</pre>
        </div>
      </div>

      <!-- 脚本说明 -->
      <div class="script-description">
        <h4>脚本说明</h4>
        <div class="description-content">
          <p v-if="selectedScriptType === 'ddl'">
            此DDL脚本用于创建当前对象。包含所有必要的字段、标签和配置信息。
          </p>
          <p v-else-if="selectedScriptType === 'query'">
            此查询脚本用于从当前对象中检索数据。包含基本的SELECT语句和可选的WHERE条件。
          </p>
          <p v-else-if="selectedScriptType === 'insert'">
            此插入脚本展示了如何向当前对象中插入数据。包含示例字段和标签值。
          </p>
        </div>
      </div>

      <!-- 相关脚本 -->
      <div class="related-scripts">
        <h4>相关脚本</h4>
        <div class="script-tabs">
          <el-tabs v-model="selectedScriptType" @tab-change="handleScriptTypeChange">
            <el-tab-pane label="DDL脚本" name="ddl">
              <div class="script-preview">
                <pre class="preview-script">{{ ddlScript }}</pre>
              </div>
            </el-tab-pane>
            <el-tab-pane label="查询脚本" name="query">
              <div class="script-preview">
                <pre class="preview-script">{{ queryScript }}</pre>
              </div>
            </el-tab-pane>
            <el-tab-pane label="插入脚本" name="insert">
              <div class="script-preview">
                <pre class="preview-script">{{ insertScript }}</pre>
              </div>
            </el-tab-pane>
          </el-tabs>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { ElMessage } from 'element-plus'
import { Document, Edit, View } from '@element-plus/icons-vue'
import MonacoEditor from '../MonacoEditor.vue'

interface Props {
  object: any
  loading?: boolean
}

const props = defineProps<Props>()

// 响应式数据
const selectedScriptType = ref('ddl')
const syntaxHighlighting = ref(true)
const currentScript = ref('')

// 编辑器配置
const editorOptions = {
  readOnly: true,
  minimap: { enabled: false },
  scrollBeyondLastLine: false,
  fontSize: 12,
  fontFamily: 'Monaco, Menlo, Ubuntu Mono, monospace',
  theme: 'vs-dark'
}

// 计算属性
const ddlScript = computed(() => {
  if (!props.object) return ''
  
  if (props.object.type === 'database') {
    return generateDatabaseDDL(props.object)
  } else if (props.object.type === 'measurement') {
    return generateMeasurementDDL(props.object)
  }
  
  return ''
})

const queryScript = computed(() => {
  if (!props.object) return ''
  
  if (props.object.type === 'database') {
    return generateDatabaseQuery(props.object)
  } else if (props.object.type === 'measurement') {
    return generateMeasurementQuery(props.object)
  }
  
  return ''
})

const insertScript = computed(() => {
  if (!props.object) return ''
  
  if (props.object.type === 'measurement') {
    return generateInsertScript(props.object)
  }
  
  return '此对象类型不支持插入脚本'
})

// 方法
const getScriptTitle = () => {
  const titles: Record<string, string> = {
    ddl: 'DDL脚本',
    query: '查询脚本',
    insert: '插入脚本'
  }
  return titles[selectedScriptType.value] || '脚本'
}

const getLanguage = () => {
  return 'sql'
}

const generateDatabaseDDL = (database: any) => {
  return `-- 创建数据库 DDL
CREATE DATABASE "${database.name}"

-- 保留策略配置
${database.retentionPolicy ? `WITH DURATION ${database.retentionPolicy}` : 'WITH DURATION INF'}

-- 副本配置
${database.replicationFactor ? `REPLICATION ${database.replicationFactor}` : 'REPLICATION 1'}

-- 分片持续时间
${database.shardDuration ? `SHARD DURATION ${database.shardDuration}` : '-- 使用默认分片持续时间'};`
}

const generateMeasurementDDL = (measurement: any) => {
  let ddl = `-- 创建 measurement DDL
-- 注意: InfluxDB 的 measurement 是自动创建的，此脚本仅供参考

-- 数据库: ${measurement.database}
-- Measurement: ${measurement.name}

-- 字段定义:`

  if (measurement.fieldKeys) {
    measurement.fieldKeys.forEach((field: any) => {
      ddl += `\n-- ${field.key}: ${field.type}`
    })
  }

  ddl += '\n\n-- 标签定义:'
  
  if (measurement.tagKeys) {
    measurement.tagKeys.forEach((tag: any) => {
      ddl += `\n-- ${tag.key}: string (标签)`
    })
  }

  ddl += '\n\n-- 示例插入语句:'
  ddl += `\nINSERT ${measurement.name}`
  
  if (measurement.tagKeys?.length) {
    const tags = measurement.tagKeys.map((tag: any) => `${tag.key}=value`).join(',')
    ddl += `,${tags}`
  }
  
  if (measurement.fieldKeys?.length) {
    const fields = measurement.fieldKeys.map((field: any) => `${field.key}=0`).join(',')
    ddl += ` ${fields}`
  }
  
  ddl += '\n-- 时间戳会自动生成'

  return ddl
}

const generateDatabaseQuery = (database: any) => {
  return `-- 数据库查询脚本
-- 数据库: ${database.name}

-- 查看所有 measurement
SHOW MEASUREMENTS

-- 查看数据库统计信息
SHOW STATS

-- 查看诊断信息
SHOW DIAGNOSTICS

-- 查看用户权限
SHOW USERS

-- 查看保留策略
SHOW RETENTION POLICIES

-- 查看连续查询
SHOW CONTINUOUS QUERIES`
}

const generateMeasurementQuery = (measurement: any) => {
  let query = `-- Measurement 查询脚本
-- 数据库: ${measurement.database}
-- Measurement: ${measurement.name}

-- 基本查询
SELECT * FROM "${measurement.name}"
LIMIT 100

-- 指定字段查询`

  if (measurement.fieldKeys) {
    const fields = measurement.fieldKeys.map((field: any) => field.key).join(', ')
    query += `\nSELECT ${fields} FROM "${measurement.name}"\nLIMIT 100`
  }

  query += '\n\n-- 时间范围查询'
  query += `\nSELECT * FROM "${measurement.name}"\nWHERE time >= now() - 1h\nLIMIT 100`

  if (measurement.tagKeys?.length) {
    query += '\n\n-- 标签过滤查询'
    const tag = measurement.tagKeys[0]
    query += `\nSELECT * FROM "${measurement.name}"\nWHERE "${tag.key}" = 'value'\nLIMIT 100`
  }

  query += '\n\n-- 聚合查询'
  if (measurement.fieldKeys) {
    const numericFields = measurement.fieldKeys.filter((field: any) => 
      field.type === 'float' || field.type === 'integer'
    )
    if (numericFields.length) {
      const field = numericFields[0]
      query += `\nSELECT mean("${field.key}") FROM "${measurement.name}"\nWHERE time >= now() - 1h\nGROUP BY time(5m)`
    }
  }

  return query
}

const generateInsertScript = (measurement: any) => {
  let insert = `-- 插入脚本示例
-- 数据库: ${measurement.database}
-- Measurement: ${measurement.name}

-- 基本插入`

  if (measurement.tagKeys?.length && measurement.fieldKeys?.length) {
    const tags = measurement.tagKeys.map((tag: any) => `${tag.key}=value`).join(',')
    const fields = measurement.fieldKeys.map((field: any) => {
      switch (field.type) {
        case 'float': return `${field.key}=0.0`
        case 'integer': return `${field.key}=0`
        case 'string': return `${field.key}="value"`
        case 'boolean': return `${field.key}=true`
        default: return `${field.key}=0`
      }
    }).join(',')
    
    insert += `\nINSERT ${measurement.name},${tags} ${fields}`
  } else if (measurement.fieldKeys?.length) {
    const fields = measurement.fieldKeys.map((field: any) => {
      switch (field.type) {
        case 'float': return `${field.key}=0.0`
        case 'integer': return `${field.key}=0`
        case 'string': return `${field.key}="value"`
        case 'boolean': return `${field.key}=true`
        default: return `${field.key}=0`
      }
    }).join(',')
    
    insert += `\nINSERT ${measurement.name} ${fields}`
  }

  insert += '\n\n-- 批量插入示例'
  insert += `\nINSERT ${measurement.name}`
  
  if (measurement.tagKeys?.length) {
    const tags = measurement.tagKeys.map((tag: any) => `${tag.key}=value`).join(',')
    insert += `,${tags}`
  }
  
  if (measurement.fieldKeys?.length) {
    const fields = measurement.fieldKeys.map((field: any) => {
      switch (field.type) {
        case 'float': return `${field.key}=0.0`
        case 'integer': return `${field.key}=0`
        case 'string': return `${field.key}="value"`
        case 'boolean': return `${field.key}=true`
        default: return `${field.key}=0`
      }
    }).join(',')
    insert += ` ${fields}`
  }
  
  insert += '\n-- 可以添加多行数据，每行用换行符分隔'

  return insert
}

const copyDDL = async () => {
  try {
    await navigator.clipboard.writeText(ddlScript.value)
    ElMessage.success('DDL脚本已复制到剪贴板')
  } catch (error) {
    ElMessage.error('复制失败')
  }
}

const copyQuery = async () => {
  try {
    await navigator.clipboard.writeText(queryScript.value)
    ElMessage.success('查询脚本已复制到剪贴板')
  } catch (error) {
    ElMessage.error('复制失败')
  }
}

const openInQueryEditor = () => {
  // 这里可以通过事件总线或路由跳转到查询编辑器
  ElMessage.info('功能开发中：将在查询编辑器中打开脚本')
}

const formatScript = () => {
  // 简单的格式化逻辑
  currentScript.value = currentScript.value
    .replace(/\s+/g, ' ')
    .replace(/\s*,\s*/g, ', ')
    .replace(/\s*=\s*/g, ' = ')
    .trim()
}

const toggleSyntaxHighlighting = () => {
  syntaxHighlighting.value = !syntaxHighlighting.value
}

const handleScriptChange = (value: string) => {
  currentScript.value = value
}

const handleScriptTypeChange = (type: string) => {
  selectedScriptType.value = type
}

// 监听脚本类型变化
watch(selectedScriptType, (newType) => {
  switch (newType) {
    case 'ddl':
      currentScript.value = ddlScript.value
      break
    case 'query':
      currentScript.value = queryScript.value
      break
    case 'insert':
      currentScript.value = insertScript.value
      break
  }
}, { immediate: true })

// 监听对象变化
watch(() => props.object, () => {
  if (props.object) {
    handleScriptTypeChange(selectedScriptType.value)
  }
}, { immediate: true })
</script>

<style scoped>
.ddl-tab {
  height: 100%;
  display: flex;
  flex-direction: column;
  padding: 16px;
}

.loading-container,
.empty-container {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 300px;
}

.ddl-content {
  height: 100%;
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.toolbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  border: 1px solid #e4e7ed;
  border-radius: 6px;
  background-color: #fafafa;
}

.toolbar-left,
.toolbar-right {
  display: flex;
  align-items: center;
  gap: 12px;
}

.script-container {
  border: 1px solid #e4e7ed;
  border-radius: 6px;
  overflow: hidden;
}

.script-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  background-color: #f5f7fa;
  border-bottom: 1px solid #e4e7ed;
}

.script-title {
  margin: 0;
  font-size: 14px;
  font-weight: 600;
  color: #303133;
}

.script-actions {
  display: flex;
  gap: 8px;
}

.script-content {
  background-color: #1e1e1e;
}

.plain-script {
  margin: 0;
  padding: 16px;
  color: #d4d4d4;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: 12px;
  line-height: 1.5;
  white-space: pre-wrap;
  word-break: break-all;
  max-height: 400px;
  overflow-y: auto;
}

.script-description {
  border: 1px solid #e4e7ed;
  border-radius: 6px;
  padding: 16px;
  background-color: #fafafa;
}

.script-description h4 {
  margin: 0 0 12px 0;
  font-size: 14px;
  font-weight: 600;
  color: #303133;
}

.description-content p {
  margin: 0;
  color: #606266;
  line-height: 1.6;
}

.related-scripts {
  border: 1px solid #e4e7ed;
  border-radius: 6px;
  overflow: hidden;
}

.related-scripts h4 {
  margin: 0;
  padding: 12px 16px;
  font-size: 14px;
  font-weight: 600;
  color: #303133;
  background-color: #f5f7fa;
  border-bottom: 1px solid #e4e7ed;
}

.script-tabs {
  background-color: white;
}

.script-preview {
  padding: 16px;
  background-color: #fafafa;
}

.preview-script {
  margin: 0;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: 11px;
  line-height: 1.4;
  color: #606266;
  white-space: pre-wrap;
  word-break: break-all;
}

:deep(.el-tabs__header) {
  margin: 0;
  background-color: #f5f7fa;
}

:deep(.el-tabs__nav-wrap) {
  padding: 0 16px;
}

:deep(.el-tabs__item) {
  font-size: 12px;
}
</style> 