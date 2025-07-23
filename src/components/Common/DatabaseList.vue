<template>
  <div class="database-list">
    <div class="database-list__header">
      <h3 class="database-list__title">{{ title }}</h3>
      <div class="database-list__actions">
        <el-button 
          v-if="canCreateDatabase"
          type="primary" 
          size="small" 
          @click="$emit('create-database')"
          :disabled="loading"
        >
          <el-icon><Plus /></el-icon>
          新建
        </el-button>
        <el-button 
          type="default" 
          size="small" 
          @click="refreshDatabases"
          :loading="loading"
        >
          <el-icon><Refresh /></el-icon>
          刷新
        </el-button>
      </div>
    </div>
    
    <tree-view
      :nodes="databaseTreeData"
      :loading="loading"
      :empty-text="emptyText"
      node-key="id"
      :default-expanded-keys="defaultExpandedKeys"
      @node-click="handleNodeClick"
      @node-dblclick="handleNodeDoubleClick"
      @context-menu-action="handleContextMenuAction"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue';
import { ElMessage } from 'element-plus';
import { Plus, Refresh } from '@element-plus/icons-vue';
import { useDatabaseStore } from '../../stores/databaseStore';
import { useMeasurementStore } from '../../stores/measurementStore';
import TreeView from './TreeView.vue';

// Props
const props = defineProps({
  title: {
    type: String,
    default: '数据库'
  },
  emptyText: {
    type: String,
    default: '暂无数据库'
  },
  canCreateDatabase: {
    type: Boolean,
    default: true
  },
  connectionId: {
    type: String,
    default: ''
  }
});

// Emits
const emit = defineEmits([
  'create-database',
  'node-click',
  'node-dblclick',
  'refresh-complete',
  'delete-database',
  'delete-measurement',
  'query-data',
  'create-measurement',
  'context-menu-action'
]);

// Stores
const databaseStore = useDatabaseStore();
const measurementStore = useMeasurementStore();

// State
const defaultExpandedKeys = ref<string[]>([]);
const databaseTreeData = ref<any[]>([]);

// Computed
const loading = computed(() => databaseStore.isLoading || measurementStore.isLoading);

// Methods
const refreshDatabases = async () => {
  try {
    await databaseStore.fetchDatabases();
    
    // Transform database data for tree view
    databaseTreeData.value = databaseStore.getDatabases.map(db => ({
      id: db.name,
      name: db.name,
      type: 'database',
      children: []
    }));
    
    emit('refresh-complete', databaseTreeData.value);
  } catch (error) {
    console.error('Error refreshing databases:', error);
    ElMessage.error('获取数据库列表失败');
  }
};

const loadMeasurements = async (database: string) => {
  try {
    const measurements = await measurementStore.fetchMeasurementsForDatabase(database);
    
    // Find the database node and update its children
    const dbNode = databaseTreeData.value.find(db => db.name === database);
    if (dbNode) {
      if (measurements && measurements.length > 0) {
        dbNode.children = measurements.map(measurement => ({
          id: `${database}.${measurement.name}`,
          name: measurement.name,
          type: 'measurement',
          database,
          count: 0 // Could be updated with actual count if available
        }));
      } else {
        dbNode.children = []; // Empty array if no measurements found
      }
    }
  } catch (error) {
    console.error(`Error loading measurements for ${database}:`, error);
    ElMessage.error(`获取测量值列表失败: ${error instanceof Error ? error.message : '未知错误'}`);
  }
};

const handleNodeClick = async (node: any) => {
  emit('node-click', node);
  
  if (node.type === 'database') {
    // Load measurements when a database is clicked
    await loadMeasurements(node.name);
  }
};

const handleNodeDoubleClick = (node: any) => {
  emit('node-dblclick', node);
};

const handleContextMenuAction = async (event: any) => {
  const { action, node } = event;
  
  switch (action) {
    case 'refresh-all':
      await refreshDatabases();
      break;
      
    case 'refresh':
      if (node && node.type === 'database') {
        await loadMeasurements(node.name);
      } else {
        await refreshDatabases();
      }
      break;
      
    case 'create-database':
      emit('create-database');
      break;
      
    case 'delete-database':
      if (node && node.type === 'database') {
        emit('node-click', node); // Select the node first
        emit('delete-database', node);
      }
      break;
      
    case 'create-measurement':
      if (node && node.type === 'database') {
        emit('create-measurement', node);
      }
      break;
      
    case 'delete-measurement':
      if (node && node.type === 'measurement') {
        emit('node-click', node); // Select the node first
        emit('delete-measurement', node);
      }
      break;
      
    case 'query-data':
      if (node) {
        emit('query-data', node);
      }
      break;
      
    default:
      console.log('未处理的上下文菜单操作', action);
      emit('context-menu-action', event);
  }
};

// Watch for connection changes
watch(() => props.connectionId, (newConnectionId) => {
  if (newConnectionId) {
    refreshDatabases();
  } else {
    databaseTreeData.value = [];
  }
});

// Lifecycle hooks
onMounted(() => {
  if (props.connectionId) {
    refreshDatabases();
  }
});
</script>

<style>
.database-list {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.database-list__header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--ide-spacing-md);
}

.database-list__title {
  margin: 0;
  font-size: var(--ide-font-size-md);
  font-weight: 500;
}

.database-list__actions {
  display: flex;
  gap: var(--ide-spacing-xs);
}
</style>