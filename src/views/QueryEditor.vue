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
import { ref, computed, watch } from 'vue';
import { useRoute } from 'vue-router';
import { ElMessage } from 'element-plus';
import { VideoPlay } from '@element-plus/icons-vue';
import { invoke } from '@tauri-apps/api/core';
import { useConnectionStore } from '../stores/connectionStore';
import { useQueryStore } from '../stores/queryStore';
import type { QueryResult } from '../stores/queryStore';
import { InfluxDBVersion } from '../types/influxdb';
import { QueryValidator } from '../utils/queryValidator';
import MonacoEditor from '../components/Common/MonacoEditor.vue';

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
  if (!activeConnection.value || !selectedDatabase.value) {
    ElMessage.warning('请先选择一个数据库');
    return;
  }
  const backendConnectionId = connectionStore.connectionStatus[activeConnection.value.id]?.backendConnectionId;
  if (!backendConnectionId) {
    ElMessage.error('连接未建立');
    return;
  }

  // 验证查询语法
  const validation = QueryValidator.validateQuery(currentQuery.value, activeConnection.value.version);
  if (!validation.isValid) {
    ElMessage.error(`查询语法错误: ${validation.error}`);
    if (validation.suggestion) {
      ElMessage.info(`建议: ${validation.suggestion}`);
    }
    if (validation.correctedQuery) {
      ElMessage.info(`修正后的查询: ${validation.correctedQuery}`);
    }
    return;
  }

  isExecuting.value = true;
  try {
    const result = await queryStore.executeQuery(currentQuery.value, selectedDatabase.value, backendConnectionId);
    if (result) {
      queryResults.value = result;
      if (!result.error) {
        ElMessage.success('查询执行成功');
      }
    }
  } catch (error) {
    ElMessage.error(`查询执行失败: ${error}`);
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
    currentQuery.value = conn.version === InfluxDBVersion.V2
      ? 'from(bucket: "my-bucket")\n  |> range(start: -1h)\n  |> limit(n: 10)'
      : 'SELECT * FROM measurement LIMIT 10';
  } else {
    databases.value = [];
    selectedDatabase.value = '';
  }
}, { immediate: true });
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