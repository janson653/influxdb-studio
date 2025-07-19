<template>
  <div class="main-layout">
    <!-- 顶部工具栏 -->
    <div class="top-toolbar">
      <el-button v-if="!isQueryView" @click="goBackToEditor" size="small" text>
        <el-icon><ArrowLeft /></el-icon>
        返回
      </el-button>
      <div class="project-selector">
        <span class="project-icon">DB</span>
        <span>{{ activeConnection ? activeConnection.name : '未连接' }}</span>
      </div>
      <div class="toolbar-actions">
        <el-button @click="gotoSettings" size="small">
          <el-icon><Setting /></el-icon>
          设置
        </el-button>
      </div>
    </div>

    <!-- 主容器 -->
    <div class="main-container">
      <!-- 左侧边栏 -->
      <div class="sidebar">
        <!-- 连接管理 -->
        <div class="connection-section">
          <div class="sidebar-header">
            <span>连接管理</span>
            <el-button @click="showAddDialog" size="small" circle>
              <el-icon><Plus /></el-icon>
            </el-button>
          </div>
          <div class="connection-list">
            <div
              v-for="conn in connections"
              :key="conn.id"
              class="connection-item"
              :class="{ active: activeConnection && activeConnection.id === conn.id }"
              @click="handleConnectionSelect(conn)"
            >
              <span class="conn-name">{{ conn.name }}</span>
              <el-dropdown trigger="click" @command="handleCommand($event, conn)">
                <span class="conn-actions">...</span>
                <template #dropdown>
                  <el-dropdown-menu>
                    <el-dropdown-item command="connect">
                      <el-icon><Link /></el-icon>连接
                    </el-dropdown-item>
                    <el-dropdown-item command="edit">
                      <el-icon><Edit /></el-icon>编辑
                    </el-dropdown-item>
                    <el-dropdown-item command="delete" divided>
                      <el-icon><Delete /></el-icon>删除
                    </el-dropdown-item>
                  </el-dropdown-menu>
                </template>
              </el-dropdown>
            </div>
          </div>
        </div>

        <!-- 数据库浏览器 -->
        <div class="database-explorer">
          <div class="sidebar-header">
            <span>数据库浏览器</span>
            <el-button @click="refreshData" :loading="isLoading" size="small" circle>
              <el-icon><Refresh /></el-icon>
            </el-button>
          </div>
          <div class="explorer-tree" @contextmenu.prevent="openContextMenu($event, null)">
            <div v-if="!activeConnection || !isConnected" class="empty-state">
              <el-empty description="未连接" :image-size="60" />
            </div>
            <el-tree
              v-else
              :data="databaseTreeData"
              :props="{ children: 'children', label: 'name' }"
              @node-click="handleDatabaseClick"
            >
              <template #default="{ node, data }">
                <span class="tree-node" @contextmenu.prevent.stop="openContextMenu($event, data)">
                  <el-icon v-if="data.type === 'database'"><Folder /></el-icon>
                  <el-icon v-else><Document /></el-icon>
                  <span>{{ node.label }}</span>
                </span>
              </template>
            </el-tree>
          </div>
        </div>
      </div>

      <!-- 右侧内容区域 -->
      <div class="content-area">
        <QueryEditor v-if="isQueryView" :key="editorKey" :initial-db="selectedDb" :initial-measurement="selectedMeasurement" />
        <router-view v-else />
      </div>
    </div>

    <!-- 底部状态栏 -->
    <div class="status-bar">
       <div class="status-item" v-if="activeConnection">
        <el-tag :type="getConnectionStatusType()" size="small" effect="dark">
          {{ getConnectionStatusText() }}
        </el-tag>
        <span>{{ activeConnection.config.host }}:{{ activeConnection.config.port }}</span>
      </div>
    </div>

    <ConnectionDialog
      v-model="connectionDialogVisible"
      :connection="editingConnection"
      @save="handleSaveConnection"
    />

    <!-- 右键上下文菜单 -->
    <div v-if="contextMenu.visible" class="context-menu" :style="{ top: contextMenu.top, left: contextMenu.left }">
      <div v-if="contextMenu.node?.type === 'database'">
        <div class="context-menu-item" @click="handleNewTable">新建表</div>
        <div class="context-menu-item" @click="handleDeleteDatabase">删除库</div>
      </div>
      <div v-else-if="contextMenu.node?.type === 'measurement'">
        <div class="context-menu-item" @click="handleDeleteTable">删除表</div>
      </div>
      <div v-else>
        <div class="context-menu-item" @click="handleNewDatabase">新建库</div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch, reactive } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { ElMessage, ElMessageBox } from 'element-plus';
import {
  Refresh, Folder, Document, Setting, Plus, Link, Edit, Delete, ArrowLeft
} from '@element-plus/icons-vue';
import { invoke } from '@tauri-apps/api/core';
import { useConnectionStore } from '../../stores/connectionStore';
import type { ConnectionProfile } from '../../types/influxdb';
import ConnectionDialog from '../Connection/ConnectionDialog.vue';
import QueryEditor from '../../views/QueryEditor.vue';

const router = useRouter();
const route = useRoute();
const connectionStore = useConnectionStore();

const isLoading = ref(false);
const databaseTreeData = ref<any[]>([]);
const connectionDialogVisible = ref(false);
const editingConnection = ref<ConnectionProfile | null>(null);
const selectedDb = ref('');
const selectedMeasurement = ref('');
const editorKey = ref(0);

const contextMenu = reactive({
  visible: false,
  top: '0px',
  left: '0px',
  node: null as any,
});

const connections = computed(() => connectionStore.connections);
const activeConnection = computed(() => connectionStore.activeConnectionConfig);
const connectionStatus = computed(() => activeConnection.value ? connectionStore.connectionStatus[activeConnection.value.id] : null);
const isConnected = computed(() => connectionStatus.value?.status === 'connected');
const isQueryView = computed(() => route.path === '/');

const gotoSettings = () => router.push('/settings');
const goBackToEditor = () => router.push('/');

const showAddDialog = () => {
  editingConnection.value = null;
  connectionDialogVisible.value = true;
};

const handleSaveConnection = (conn: ConnectionProfile) => {
  connectionStore.addConnection(conn);
  ElMessage.success('连接已保存');
  connectionDialogVisible.value = false;
};

const handleConnectionSelect = (conn: ConnectionProfile) => {
  connectionStore.setActiveConnection(conn.id);
  if (!isConnected.value) {
    connectTo(conn.id);
  }
};

const handleCommand = (command: string, conn: ConnectionProfile) => {
  switch (command) {
    case 'connect':
      connectTo(conn.id);
      break;
    case 'edit':
      editingConnection.value = { ...conn };
      connectionDialogVisible.value = true;
      break;
    case 'delete':
      deleteConnection(conn.id);
      break;
  }
};

const connectTo = async (id: string) => {
  try {
    const success = await connectionStore.connectTo(id);
    if (success) ElMessage.success('连接成功');
    else ElMessage.error('连接失败');
  } catch (error) {
    ElMessage.error(`连接失败: ${error}`);
  }
};

const deleteConnection = async (id: string) => {
  await ElMessageBox.confirm('确定要删除这个连接吗？', '确认删除', { type: 'warning' });
  connectionStore.removeConnection(id);
  ElMessage.success('连接已删除');
};

const refreshData = async () => {
  if (!isConnected.value) return;
  isLoading.value = true;
  try {
    await loadDatabases();
    ElMessage.success('数据已刷新');
  } catch (error) {
    ElMessage.error('刷新失败');
  } finally {
    isLoading.value = false;
  }
};

const loadDatabases = async () => {
  const backendId = connectionStatus.value?.backendConnectionId;
  if (!backendId) return;
  const response = await invoke('get_databases', { connectionId: backendId }) as any;
  databaseTreeData.value = response.success ? response.data.map((db: string) => ({ id: db, name: db, type: 'database', children: [] })) : [];
};

const handleDatabaseClick = async (data: any) => {
  selectedDb.value = data.type === 'database' ? data.name : data.database;
  selectedMeasurement.value = data.type === 'measurement' ? data.name : '';
  editorKey.value++; // Force re-render of QueryEditor
  if (data.type === 'database') await loadMeasurements(data.name);
  router.push('/');
};

const loadMeasurements = async (db: string) => {
  const backendId = connectionStatus.value?.backendConnectionId;
  if (!backendId) return;
  const response = await invoke('get_measurements', { connectionId: backendId, database: db }) as any;
  if (response.success) {
    const node = databaseTreeData.value.find(d => d.name === db);
    if (node) node.children = response.data.map((m: string) => ({ id: `${db}.${m}`, name: m, type: 'measurement', database: db }));
  }
};

const getConnectionStatusType = () => {
  if (!connectionStatus.value) return 'info';
  return { connected: 'success', connecting: 'warning', error: 'danger' }[connectionStatus.value.status] || 'info';
};

const getConnectionStatusText = () => {
  if (!connectionStatus.value) return '未连接';
  return { connected: '已连接', connecting: '连接中', error: '错误' }[connectionStatus.value.status] || '未连接';
};

watch(isConnected, (connected) => {
  if (connected) loadDatabases();
  else databaseTreeData.value = [];
});

onMounted(() => {
  if (isConnected.value) loadDatabases();
  document.addEventListener('click', closeContextMenu);
});

onUnmounted(() => {
  document.removeEventListener('click', closeContextMenu);
});

// --- 右键菜单方法 ---
const openContextMenu = (event: MouseEvent, node: any) => {
  contextMenu.visible = true;
  contextMenu.top = `${event.clientY}px`;
  contextMenu.left = `${event.clientX}px`;
  contextMenu.node = node;
};

const closeContextMenu = () => {
  contextMenu.visible = false;
};

const handleNewDatabase = async () => {
  closeContextMenu();
  try {
    const { value } = await ElMessageBox.prompt('请输入新数据库的名称:', '新建库', {
      confirmButtonText: '创建',
      cancelButtonText: '取消',
    });
    const backendId = connectionStatus.value?.backendConnectionId;
    if (!backendId) return;
    const response = await invoke('create_database', { connectionId: backendId, database: value }) as any;
    if (response.success) {
      ElMessage.success('数据库创建成功');
      await loadDatabases();
    } else {
      ElMessage.error(response.error || '创建失败');
    }
  } catch { /* 用户取消 */ }
};

const handleDeleteDatabase = async () => {
  const dbName = contextMenu.node.name;
  closeContextMenu();
  try {
    await ElMessageBox.confirm(`确定要删除数据库 "${dbName}" 吗？此操作不可逆！`, '确认删除', { type: 'warning' });
    const backendId = connectionStatus.value?.backendConnectionId;
    if (!backendId) return;
    const response = await invoke('drop_database', { connectionId: backendId, database: dbName }) as any;
    if (response.success) {
      ElMessage.success('数据库删除成功');
      await loadDatabases();
    } else {
      ElMessage.error(response.error || '删除失败');
    }
  } catch { /* 用户取消 */ }
};

const handleNewTable = async () => {
  const dbName = contextMenu.node.name;
  closeContextMenu();
  try {
    const { value } = await ElMessageBox.prompt('请输入新表的名称:', '新建表', {
      confirmButtonText: '创建',
      cancelButtonText: '取消',
    });
    const newQuery = `INSERT ${value},tag_key=tag_value field_key="field_value"`;
    router.push({ path: '/', query: { db: dbName, measurement: value, query: newQuery } });
    ElMessage.info('请在查询编辑器中执行 INSERT 语句以创建表和写入数据。');
  } catch { /* 用户取消 */ }
};

const handleDeleteTable = async () => {
  const { database, name } = contextMenu.node;
  closeContextMenu();
  try {
    await ElMessageBox.confirm(`确定要删除表 "${name}" 吗？`, '确认删除', { type: 'warning' });
    const backendId = connectionStatus.value?.backendConnectionId;
    if (!backendId) return;
    const query = `DROP MEASUREMENT "${name}"`;
    const response = await invoke('execute_query', { connectionId: backendId, database, query }) as any;
    if (response.success) {
      ElMessage.success('表删除成功');
      await loadMeasurements(database);
    } else {
      ElMessage.error(response.error || '删除失败');
    }
  } catch { /* 用户取消 */ }
};
</script>

<style scoped>
.main-layout { display: flex; flex-direction: column; height: 100vh; background: #2b2b2b; }
.top-toolbar { height: 40px; background: #3c3f41; border-bottom: 1px solid #555; display: flex; align-items: center; padding: 0 10px; flex-shrink: 0; }
.project-selector { display: flex; align-items: center; gap: 8px; padding: 5px 10px; background: #4c5052; border-radius: 4px; margin-left: 10px; }
.project-icon { width: 20px; height: 20px; background: #6a8759; border-radius: 3px; display: flex; align-items: center; justify-content: center; color: white; font-size: 12px; }
.toolbar-actions { margin-left: auto; }
.main-container { display: flex; flex: 1; overflow: hidden; }
.sidebar { width: 240px; background: #3c3f41; border-right: 1px solid #555; display: flex; flex-direction: column; }
.sidebar-header { padding: 10px; border-bottom: 1px solid #555; display: flex; justify-content: space-between; align-items: center; }
.connection-section { border-bottom: 1px solid #555; }
.connection-list { padding: 5px; max-height: 200px; overflow-y: auto; }
.connection-item { display: flex; justify-content: space-between; padding: 8px 10px; cursor: pointer; border-radius: 3px; }
.connection-item:hover { background: #4c5052; }
.connection-item.active { background: #6a8759; color: white; }
.conn-actions { cursor: pointer; padding: 0 5px; }
.database-explorer { flex: 1; display: flex; flex-direction: column; }
.explorer-tree { flex: 1; overflow-y: auto; padding: 5px; }
.tree-node { display: flex; align-items: center; gap: 5px; }
.empty-state { padding: 20px; text-align: center; }
.content-area { flex: 1; display: flex; flex-direction: column; overflow: hidden; }
.status-bar { height: 25px; background: #3c3f41; border-top: 1px solid #555; display: flex; align-items: center; padding: 0 10px; font-size: 12px; color: #808080; }
.status-item { margin-right: 20px; display: flex; align-items: center; gap: 8px; }

.context-menu {
  position: fixed;
  z-index: 1000;
  background-color: #4c5052;
  border: 1px solid #555;
  border-radius: 4px;
  box-shadow: 0 2px 12px 0 rgba(0,0,0,.1);
  padding: 5px 0;
}

.context-menu-item {
  padding: 8px 15px;
  cursor: pointer;
  font-size: 14px;
  color: #a9b7c6;
}

.context-menu-item:hover {
  background-color: #6a8759;
  color: white;
}
</style>
