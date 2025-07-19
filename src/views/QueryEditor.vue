<template>
  <div class="query-editor-view">
    <div class="editor-pane">
      <div class="editor-toolbar">
        <el-select
          v-model="selectedDatabase"
          :placeholder="getDatabasePlaceholder()"
          size="small"
          style="width: 180px; margin-right: 10px;"
          @change="handleDatabaseChange"
        >
          <el-option v-for="db in databases" :key="db" :label="db" :value="db" />
        </el-select>
        <el-button @click="executeQuery" :loading="isExecuting" type="primary" size="small">
          <el-icon><VideoPlay /></el-icon>
          执行查询
        </el-button>
        <el-button @click="showQueryHistory" size="small">历史</el-button>
        <el-button @click="showQueryExamples" size="small">示例</el-button>
        <el-button @click="resetTableNameReplaced" size="small" :disabled="!isTableNameReplaced">
          重置状态
        </el-button>
      </div>
      <div class="editor-container">
        <MonacoEditor
          v-model="currentQuery"
          :language="getQueryLanguage()"
          :options="editorOptions"
        />
      </div>
    </div>
    <div class="results-pane">
      <div class="results-container">
        <!-- Output面板 -->
        <OutputPanel 
          :logs="outputLogs" 
          @clear="clearOutputLogs"
          class="output-panel"
        />
        
        <!-- 结果面板 -->
        <div class="results-panel">
          <el-tabs v-model="activeResultTab" type="border-card" class="results-tabs">
            <el-tab-pane label="结果" name="table">
              <div v-if="!queryResults" class="no-results">
                <el-empty description="暂无查询结果" :image-size="80" />
              </div>
              <div v-else-if="queryResults.error" class="error-results">
                <el-alert :title="queryResults.error" type="error" show-icon :closable="false" />
              </div>
              <div v-else class="results-content">
                <div class="results-info">
                  <el-tag type="info" size="small">执行时间: {{ queryResults.execution_time }}ms</el-tag>
                  <el-tag type="success" size="small">结果行数: {{ getTotalResults() }}</el-tag>
                </div>
                <div class="table-container">
                  <el-table :data="tableData" style="width: 100%" max-height="300" border>
                    <el-table-column
                      v-for="column in tableColumns"
                      :key="column"
                      :prop="column"
                      :label="column"
                      show-overflow-tooltip
                    />
                  </el-table>
                </div>
              </div>
            </el-tab-pane>
            <el-tab-pane label="JSON" name="json">
               <div class="json-container">
                  <pre>{{ JSON.stringify(queryResults, null, 2) }}</pre>
                </div>
            </el-tab-pane>
          </el-tabs>
        </div>
      </div>
    </div>

    <!-- 查询历史对话框 -->
    <el-dialog v-model="showHistory" title="查询历史" width="800px">
      <el-table :data="recentQueries" style="width: 100%">
        <el-table-column prop="query" label="查询语句" show-overflow-tooltip />
        <el-table-column prop="database" label="数据库" width="120" />
        <el-table-column prop="timestamp" label="时间" width="150">
          <template #default="{ row }">{{ formatTime(row.timestamp) }}</template>
        </el-table-column>
        <el-table-column label="操作" width="100">
          <template #default="{ row }">
            <el-button size="small" @click="loadQuery(row)">加载</el-button>
          </template>
        </el-table-column>
      </el-table>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted } from 'vue';
import { useRoute } from 'vue-router';
import { ElMessage } from 'element-plus';
import { VideoPlay } from '@element-plus/icons-vue';
import { invoke } from '@tauri-apps/api/core';
import { useConnectionStore } from '../stores/connectionStore';
import { useQueryStore } from '../stores/queryStore';
import type { QueryResult } from '../stores/queryStore';
import { InfluxDBVersion } from '../types/influxdb';
import { QueryValidator } from '../utils/queryValidator';
import { SqlReplacer } from '../utils/sqlReplacer';
import { eventBus, Events } from '../utils/eventBus';
import MonacoEditor from '../components/Common/MonacoEditor.vue';
import OutputPanel, { type OutputLog } from '../components/Query/OutputPanel.vue';

const props = defineProps<{
  initialDb?: string;
  initialMeasurement?: string;
}>();

const route = useRoute();
const connectionStore = useConnectionStore();
const queryStore = useQueryStore();


const currentQuery = ref('SELECT * FROM measurement LIMIT 10');
const selectedDatabase = ref('');
const isExecuting = ref(false);
const queryResults = ref<QueryResult | null>(null);
const activeResultTab = ref('table');
const showHistory = ref(false);
const databases = ref<string[]>([]);
const outputLogs = ref<OutputLog[]>([]);
const isTableNameReplaced = ref(false); // 标记是否已经通过双击替换了表名

const editorOptions = {
  theme: 'vs-dark',
  fontSize: 14,
  minimap: { enabled: false },
  scrollBeyondLastLine: false,
  automaticLayout: true,
  wordWrap: 'on' as const,
};

const activeConnection = computed(() => connectionStore.activeConnectionConfig);
const recentQueries = computed(() => queryStore.recentQueries);

const tableData = computed(() => {
  if (!queryResults.value?.series?.length) return [];
  return queryResults.value.series.flatMap(series =>
    series.values.map(row =>
      series.columns.reduce((obj, col, index) => {
        obj[col] = row[index];
        return obj;
      }, {} as any)
    )
  );
});

const tableColumns = computed(() => {
  return queryResults.value?.series?.[0]?.columns || [];
});

const getQueryLanguage = () => activeConnection.value?.version === InfluxDBVersion.V2 ? 'flux' : 'sql';
const getDatabasePlaceholder = () => activeConnection.value?.version === InfluxDBVersion.V2 ? '选择存储桶' : '选择数据库';

const handleDatabaseChange = (db: string) => selectedDatabase.value = db;

const executeQuery = async () => {
  // 清空之前的输出日志
  clearOutputLogs()
  
  // 记录查询开始
  addOutputLog({
    type: 'info',
    message: '开始执行查询',
    details: `数据库: ${selectedDatabase.value}\n查询语句: ${currentQuery.value}`
  })

  if (!activeConnection.value || !selectedDatabase.value) {
    addOutputLog({
      type: 'error',
      message: '查询执行失败：缺少必要参数',
      details: '请确保已选择数据库和连接'
    })
    ElMessage.warning('请先选择一个数据库');
    return;
  }
  
  const backendConnectionId = connectionStore.connectionStatus[activeConnection.value.id]?.backendConnectionId;
  if (!backendConnectionId) {
    addOutputLog({
      type: 'error',
      message: '查询执行失败：连接未建立',
      details: '请先连接到数据库'
    })
    ElMessage.error('连接未建立');
    return;
  }

  // 验证查询语法
  addOutputLog({
    type: 'info',
    message: '正在验证查询语法...'
  })

  const validation = QueryValidator.validateQuery(currentQuery.value, activeConnection.value.version);
  if (!validation.isValid) {
    addOutputLog({
      type: 'error',
      message: '查询语法验证失败',
      details: `错误: ${validation.error}\n建议: ${validation.suggestion || '无'}\n修正后的查询: ${validation.correctedQuery || '无'}`
    })
    ElMessage.error(`查询语法错误: ${validation.error}`);
    if (validation.suggestion) {
      ElMessage.info(`建议: ${validation.suggestion}`);
    }
    if (validation.correctedQuery) {
      ElMessage.info(`修正后的查询: ${validation.correctedQuery}`);
    }
    return;
  }

  addOutputLog({
    type: 'success',
    message: '查询语法验证通过'
  })

  isExecuting.value = true;
  const startTime = Date.now()
  
  try {
    addOutputLog({
      type: 'info',
      message: '正在执行查询...',
      details: `发送查询到后端服务`
    })

    const result = await queryStore.executeQuery(currentQuery.value, selectedDatabase.value, backendConnectionId);
    
    const executionTime = Date.now() - startTime
    
    if (result) {
      queryResults.value = result;
      
      if (result.error) {
        addOutputLog({
          type: 'error',
          message: '查询执行失败',
          details: `执行时间: ${executionTime}ms\n错误信息: ${result.error}`,
          data: result
        })
        ElMessage.error('查询执行失败');
      } else {
        addOutputLog({
          type: 'success',
          message: '查询执行成功',
          details: `执行时间: ${executionTime}ms\n结果行数: ${getTotalResults()}\n数据系列数: ${result.series?.length || 0}`,
          data: result
        })
        ElMessage.success('查询执行成功');
      }
    }
  } catch (error) {
    const executionTime = Date.now() - startTime
    const errorMessage = error instanceof Error ? error.message : String(error)
    
    addOutputLog({
      type: 'error',
      message: '查询执行异常',
      details: `执行时间: ${executionTime}ms\n异常信息: ${errorMessage}`,
      data: { error: errorMessage, stack: error instanceof Error ? error.stack : undefined }
    })
    ElMessage.error(`查询执行失败: ${errorMessage}`);
  } finally {
    isExecuting.value = false;
  }
};

const showQueryHistory = () => showHistory.value = true;

const loadQuery = (query: any) => {
  currentQuery.value = query.query;
  selectedDatabase.value = query.database;
  showHistory.value = false;
};

const showQueryExamples = () => {
  if (!activeConnection.value) {
    ElMessage.warning('请先选择一个连接');
    return;
  }
  
  const examples = QueryValidator.getQueryExamples(activeConnection.value.version);
  const exampleText = examples.join('\n\n');
  
  ElMessage.info(`查询示例:\n${exampleText}`);
};

const getTotalResults = () => {
  return queryResults.value?.series?.reduce((total, series) => total + series.values.length, 0) || 0;
};

const formatTime = (timestamp: Date) => new Date(timestamp).toLocaleString();

// 日志记录方法
const addOutputLog = (log: Omit<OutputLog, 'timestamp'>) => {
  outputLogs.value.push({
    ...log,
    timestamp: new Date()
  })
}

const clearOutputLogs = () => {
  outputLogs.value = []
}

// 重置表名替换标记
const resetTableNameReplaced = () => {
  isTableNameReplaced.value = false
  addOutputLog({
    type: 'info',
    message: '已重置表名替换状态',
    details: '现在可以正常切换连接和重置SQL'
  })
}

const loadDatabases = async () => {
  const backendConnectionId = activeConnection.value ? connectionStore.connectionStatus[activeConnection.value.id]?.backendConnectionId : null;
  if (!backendConnectionId) return;

  try {
    const response = await invoke('get_databases', { connectionId: backendConnectionId }) as any;
    if (response.success && response.data) {
      databases.value = response.data;
      if (!selectedDatabase.value && databases.value.length > 0) {
        selectedDatabase.value = databases.value[0];
      }
    }
  } catch (error) {
    console.error('获取数据库列表失败:', error);
  }
};

watch(() => [props.initialDb, props.initialMeasurement], ([db, measurement]) => {
  if (db) selectedDatabase.value = db;
  if (measurement) currentQuery.value = `SELECT * FROM "${measurement}" LIMIT 10`;
}, { immediate: true });

watch(() => route.query.query, (newQuery) => {
  if (newQuery && typeof newQuery === 'string') {
    currentQuery.value = newQuery;
  }
}, { immediate: true });

watch(activeConnection, (conn) => {
  if (conn) {
    loadDatabases();
    // 只有在没有通过双击替换表名的情况下才重置SQL
    if (!isTableNameReplaced.value) {
      currentQuery.value = conn.version === InfluxDBVersion.V2
        ? 'from(bucket: "my-bucket")\n  |> range(start: -1h)\n  |> limit(n: 10)'
        : 'SELECT * FROM measurement LIMIT 10';
    }
  } else {
    databases.value = [];
    selectedDatabase.value = '';
    isTableNameReplaced.value = false; // 重置标记
  }
}, { immediate: true });

// 监听双击表名事件
const handleDoubleClickTable = (data: { tableName: string; database: string }) => {
  console.log('[FE] 查询编辑器收到双击表名事件:', data)
  console.log('[FE] 当前SQL:', currentQuery.value)
  
  // 记录到Output面板
  addOutputLog({
    type: 'info',
    message: `双击表名: ${data.tableName}`,
    details: `数据库: ${data.database}\n当前SQL: ${currentQuery.value}`
  })
  
  // 如果当前选择的数据库与双击的表所在数据库不同，则切换数据库
  if (selectedDatabase.value !== data.database) {
    selectedDatabase.value = data.database
    addOutputLog({
      type: 'info',
      message: `已切换到数据库: ${data.database}`
    })
    ElMessage.info(`已切换到数据库: ${data.database}`)
  }
  
  // 检查当前SQL是否包含表名
  const hasTableName = SqlReplacer.hasAnyTableName(currentQuery.value)
  const currentTableName = SqlReplacer.getFirstTableName(currentQuery.value)
  
  console.log('[FE] 检查结果:', { hasTableName, currentTableName })
  
  if (hasTableName && currentTableName) {
    // 如果当前SQL包含表名，则替换
    const newQuery = SqlReplacer.replaceTableName(currentQuery.value, data.tableName)
    console.log('[FE] 替换结果:', { oldQuery: currentQuery.value, newQuery })
    
    if (newQuery !== currentQuery.value) {
      currentQuery.value = newQuery
      isTableNameReplaced.value = true // 设置标记
      addOutputLog({
        type: 'success',
        message: `已替换表名: ${currentTableName} → ${data.tableName}`,
        details: `原表名: ${currentTableName}\n新表名: ${data.tableName}\n新SQL: ${newQuery}`
      })
      ElMessage.success(`已替换表名: ${currentTableName} → ${data.tableName}`)
    } else {
      // 替换失败，生成新查询
      currentQuery.value = `SELECT * FROM "${data.tableName}" LIMIT 10`
      isTableNameReplaced.value = true // 设置标记
      addOutputLog({
        type: 'warning',
        message: `替换失败，已生成新查询`,
        details: `无法替换表名，生成默认查询: SELECT * FROM "${data.tableName}" LIMIT 10`
      })
      ElMessage.warning(`替换失败，已生成新查询`)
    }
  } else {
    // 如果当前SQL不包含表名，则生成一个新的查询
    currentQuery.value = `SELECT * FROM "${data.tableName}" LIMIT 10`
    isTableNameReplaced.value = true // 设置标记
    addOutputLog({
      type: 'success',
      message: `已生成新查询`,
      details: `当前SQL不包含表名，生成默认查询: SELECT * FROM "${data.tableName}" LIMIT 10`
    })
    ElMessage.success(`已生成查询: SELECT * FROM "${data.tableName}" LIMIT 10`)
  }
  
  console.log('[FE] 最终SQL:', currentQuery.value)
}

// 注册事件监听器
onMounted(() => {
  eventBus.on(Events.DOUBLE_CLICK_TABLE, handleDoubleClickTable)
})

// 清理事件监听器
onUnmounted(() => {
  eventBus.off(Events.DOUBLE_CLICK_TABLE, handleDoubleClickTable)
})
</script>

<style scoped>
.query-editor-view {
  display: flex;
  flex-direction: column;
  height: 100%;
  background-color: #2b2b2b;
}

.editor-pane {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 200px; /* 确保编辑器有最小高度 */
}

.editor-toolbar {
  padding: 8px;
  background-color: #3c3f41;
  border-bottom: 1px solid #555;
  display: flex;
  align-items: center;
}

.editor-container {
  flex: 1;
  border: 1px solid #555;
  border-radius: 4px;
  overflow: hidden;
}

.results-pane {
  flex: 1;
  display: flex;
  flex-direction: column;
  border-top: 1px solid #555;
  min-height: 250px; /* 确保结果区域有最小高度 */
}

.results-container {
  display: flex;
  flex: 1;
  height: 100%;
}

.output-panel {
  width: 300px;
  flex-shrink: 0;
}

.results-panel {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-width: 0; /* 防止flex子项溢出 */
}

.results-tabs {
  flex: 1;
  display: flex;
  flex-direction: column;
}

.results-tabs .el-tabs__content {
  flex: 1;
  overflow-y: auto;
}

.no-results, .error-results {
  padding: 20px;
}

.results-content {
  padding: 10px;
}

.results-info {
  margin-bottom: 10px;
}

.table-container {
  max-height: calc(100% - 40px);
  overflow: auto;
}

.json-container {
  height: 100%;
  overflow: auto;
  background-color: #2b2b2b;
  padding: 15px;
  border-radius: 4px;
  color: #a9b7c6;
}

.json-container pre {
  margin: 0;
  font-family: 'JetBrains Mono', 'Fira Code', monospace;
  font-size: 13px;
}
</style>