<template>
  <div class="database-explorer">
    <el-container>
      <el-header class="header">
        <div class="header-left">
          <h2>{{ getExplorerTitle() }}</h2>
          <div v-if="activeConnection" class="connection-info">
            <el-tag type="success">
              {{ activeConnection.name }}
            </el-tag>
            <el-tag :type="getVersionTagType(activeConnection.version)" size="small">
              {{ activeConnection.version }}
            </el-tag>
            <el-tag :type="getConnectionStatusType()" size="small">
              {{ getConnectionStatusText() }}
            </el-tag>
          </div>
        </div>
        <div class="header-right">
          <el-tooltip content="连接到数据库" placement="bottom">
            <el-button @click="connectToDatabase" v-if="!isConnected" type="primary">
              <el-icon>
                <Connection />
              </el-icon>
              连接数据库
            </el-button>
          </el-tooltip>
          <el-tooltip content="刷新 (Ctrl+R)" placement="bottom">
            <el-button @click="refreshData" :loading="isLoading" v-else>
              <el-icon>
                <Refresh />
              </el-icon>
              刷新
            </el-button>
          </el-tooltip>
          <el-tooltip content="快捷键帮助 (F1)" placement="bottom">
            <el-button @click="showShortcutHelp" size="small">
              <el-icon>
                <QuestionFilled />
              </el-icon>
            </el-button>
          </el-tooltip>
        </div>
      </el-header>

      <el-main>
        <div v-if="!activeConnection" class="no-connection">
          <el-empty description="请先连接到数据库">
            <el-button type="primary" @click="$router.push('/connection')">
              去连接管理
            </el-button>
          </el-empty>
        </div>

        <div v-else-if="!isConnected" class="not-connected">
          <el-empty description="数据库未连接">
            <el-button type="primary" @click="connectToDatabase" :loading="isConnecting">
              连接数据库
            </el-button>
          </el-empty>
        </div>

        <div v-else-if="connectionError" class="connection-error">
          <el-alert :title="`连接错误: ${connectionError}`" type="error" :closable="false" show-icon>
            <template #default>
              <div style="margin-top: 10px;">
                <el-button @click="connectToDatabase" :loading="isConnecting">
                  重试连接
                </el-button>
                <el-button @click="disconnectFromDatabase">
                  断开连接
                </el-button>
              </div>
            </template>
          </el-alert>
        </div>

        <transition name="fade">
          <div v-else class="explorer-content">
            <el-row :gutter="20">
              <el-col :span="8">
                <!-- 数据库/存储桶列表 -->
                <el-card>
                  <template #header>
                    <span>{{ getDatabaseListTitle() }}</span>
                  </template>

                  <database-list :title="getDatabaseListTitle()" :empty-text="`暂无${getDatabaseNameLabel()}`"
                    :can-create-database="canCreateDatabase()" :connection-id="connectionStatus?.backendConnectionId"
                    @create-database="showCreateDatabaseDialog" @node-click="handleDatabaseClick"
                    @node-dblclick="handleDatabaseDoubleClick" @delete-database="showDeleteDatabaseDialog"
                    @delete-measurement="showDeleteMeasurementDialog" @query-data="openQueryEditor"
                    @create-measurement="handleCreateMeasurement" />
                </el-card>
              </el-col>

              <el-col :span="16">
                <!-- 对象检查器 -->
                <ObjectInspector 
                  :object="selectedItem"
                  :loading="isLoading"
                  @refresh="refreshData"
                  @query="openQueryEditor"
                />
              </el-col>
            </el-row>
          </div>
        </transition>
      </el-main>
      
      <!-- 状态栏 -->
      <StatusBar 
        :connection="activeConnection"
        :current-database="selectedItem?.type === 'database' ? selectedItem.name : selectedItem?.database"
        :current-object="selectedItem"
        :stats="getStats()"
        :loading="isLoading || isConnecting"
        :error="connectionError"
      />
    </el-container>

    <!-- 新建数据库对话框 -->
    <CreateDatabaseDialog 
      v-model="isCreateDatabaseDialogVisible"
      @created="handleDatabaseCreated"
    />

    <!-- 快捷键帮助对话框 -->
    <ShortcutHelpDialog 
      v-model="isShortcutHelpVisible"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage, ElMessageBox } from 'element-plus'
import {
  Refresh, Connection, QuestionFilled
} from '@element-plus/icons-vue'
import { invoke } from '@tauri-apps/api/core'
import { useConnectionStore } from '../stores/connectionStore'
import { InfluxDBVersion } from '../types/influxdb'
import { eventBus, Events } from '../utils/eventBus'
import { debugHelper } from '../utils/debugHelper'
import { useHotkeys } from '../composables/useHotkeys'
import CreateDatabaseDialog from '../components/Common/Dialog/CreateDatabaseDialog.vue'
import ObjectInspector from '../components/Common/Inspector/ObjectInspector.vue'
import StatusBar from '../components/Common/StatusBar.vue'
import DatabaseList from '../components/Common/DatabaseList.vue'
import ShortcutHelpDialog from '../components/Common/Dialog/ShortcutHelpDialog.vue'

// 路由
const router = useRouter()

// 状态管理
const connectionStore = useConnectionStore()

// 响应式数据
const isLoading = ref(false)
const isConnecting = ref(false)
const selectedItem = ref<any>(null)
const isCreateDatabaseDialogVisible = ref(false)
const isShortcutHelpVisible = ref(false)

// 数据库树数据
const databaseTreeData = ref<any[]>([])

// 计算属性
const activeConnection = computed(() => connectionStore.activeConnectionConfig)

// 热键
useHotkeys('r', () => refreshData(), { ctrl: true });
useHotkeys('r', () => refreshData(), { meta: true });
useHotkeys('n', () => showCreateDatabaseDialog(), { ctrl: true });
useHotkeys('n', () => showCreateDatabaseDialog(), { meta: true });
useHotkeys('delete', () => handleDeleteKey());
useHotkeys('backspace', () => handleDeleteKey());
useHotkeys('f1', () => showShortcutHelp());

const handleDeleteKey = () => {
  if (selectedItem.value) {
    if (selectedItem.value.type === 'database') {
      showDeleteDatabaseDialog();
    } else if (selectedItem.value.type === 'measurement') {
      showDeleteMeasurementDialog();
    }
  }
};

const connectionStatus = computed(() => {
  if (!activeConnection.value) return null
  return connectionStore.connectionStatus[activeConnection.value.id]
})

const isConnected = computed(() => {
  return connectionStatus.value?.status === 'connected' &&
    connectionStatus.value?.backendConnectionId
})

const connectionError = computed(() => {
  return connectionStatus.value?.error
})

// 方法
const getVersionTagType = (version: string) => {
  switch (version) {
    case InfluxDBVersion.V1: return 'info'
    case InfluxDBVersion.V2: return 'warning'
    case InfluxDBVersion.V3: return 'success'
    default: return 'info'
  }
}

const getConnectionStatusType = () => {
  if (!connectionStatus.value) return 'info'

  switch (connectionStatus.value.status) {
    case 'connected': return 'success'
    case 'connecting': return 'warning'
    case 'error': return 'danger'
    default: return 'info'
  }
}

const getConnectionStatusText = () => {
  if (!connectionStatus.value) return '未连接'

  switch (connectionStatus.value.status) {
    case 'connected': return '已连接'
    case 'connecting': return '连接中'
    case 'error': return '连接错误'
    default: return '未连接'
  }
}

const getExplorerTitle = () => {
  if (!activeConnection.value) return '数据库浏览器'

  // 目前只支持 v1.x
  return '数据库浏览器'
}

const getDatabaseListTitle = () => {
  if (!activeConnection.value) return '数据库'

  // 目前只支持 v1.x
  return '数据库'
}

const getDatabaseNameLabel = () => {
  if (!activeConnection.value) return '数据库'

  // 目前只支持 v1.x
  return '数据库'
}

const getMeasurementNameLabel = () => {
  if (!activeConnection.value) return '测量值'

  // 目前只支持 v1.x
  return '测量值'
}

const canCreateDatabase = () => {
  if (!activeConnection.value) return false

  // InfluxDB 2.x 通常不允许通过 API 创建 bucket
  return activeConnection.value.version !== InfluxDBVersion.V2
}

const connectToDatabase = async () => {
  if (!activeConnection.value) {
    console.log('[FE] connectToDatabase: 没有活跃连接')
    return
  }

  console.log('[FE] connectToDatabase: 开始连接', {
    connectionId: activeConnection.value.id,
    connectionName: activeConnection.value.name,
    connectionConfig: activeConnection.value
  })

  isConnecting.value = true
  try {
    const success = await connectionStore.connectTo(activeConnection.value.id)
    console.log('[FE] connectToDatabase: 连接结果', {
      success,
      connectionStatus: connectionStore.connectionStatus[activeConnection.value.id]
    })

    if (success) {
      ElMessage.success('数据库连接成功')
      await loadDatabases()
    } else {
      ElMessage.error('数据库连接失败')
    }
  } catch (error) {
    console.error('[FE] connectToDatabase: 连接异常', error)
    ElMessage.error(`连接失败: ${error}`)
  } finally {
    isConnecting.value = false
  }
}

const disconnectFromDatabase = async () => {
  if (!activeConnection.value) return

  try {
    await connectionStore.disconnectFrom(activeConnection.value.id)
    ElMessage.success('已断开连接')
    databaseTreeData.value = []
    selectedItem.value = null
  } catch (error) {
    ElMessage.error('断开连接失败')
  }
}

const refreshData = async () => {
  if (!activeConnection.value || !isConnected.value) return

  isLoading.value = true
  try {
    await loadDatabases()
    ElMessage.success('数据刷新成功')
  } catch (error) {
    ElMessage.error('数据刷新失败')
  } finally {
    isLoading.value = false
  }
}

const loadDatabases = async () => {
  debugHelper.log('info', '开始加载数据库列表', {
    hasActiveConnection: !!activeConnection.value,
    isConnected: isConnected.value,
    connectionStatus: connectionStatus.value
  })

  if (!activeConnection.value || !isConnected.value) {
    debugHelper.log('error', '连接检查失败', {
      hasActiveConnection: !!activeConnection.value,
      isConnected: isConnected.value,
      connectionStatus: connectionStatus.value
    })
    console.log('[FE] loadDatabases: 连接检查失败', {
      hasActiveConnection: !!activeConnection.value,
      isConnected: isConnected.value,
      connectionStatus: connectionStatus.value
    })
    return
  }

  const backendConnectionId = connectionStatus.value?.backendConnectionId

  if (!backendConnectionId) {
    debugHelper.log('error', 'backendConnectionId 不存在', {
      connectionStatus: connectionStatus.value
    })
    console.error('[FE] loadDatabases: backendConnectionId 不存在', {
      connectionStatus: connectionStatus.value
    })
    ElMessage.error('连接未建立，请先连接到数据库')
    return
  }

  console.log('[FE] loadDatabases: 开始获取数据库列表', {
    backendConnectionId,
    activeConnection: activeConnection.value
  })

  try {
    const response = await invoke('get_databases', {
      connectionId: backendConnectionId
    }) as any

    console.log('[FE] loadDatabases: 后端响应', response)

    if (response.success && response.data) {
      console.log('[FE] loadDatabases: 成功获取数据库列表', response.data)
      // 构建树形数据
      databaseTreeData.value = response.data.map((db: string) => ({
        id: db,
        name: db,
        type: 'database',
        children: []
      }))
      console.log('[FE] loadDatabases: 构建的树形数据', databaseTreeData.value)

      debugHelper.log('info', '成功加载数据库列表', {
        databaseCount: response.data.length,
        databases: response.data
      })
    } else {
      console.error('[FE] loadDatabases: 后端返回错误', response.error)
      debugHelper.log('error', '后端返回错误', response.error)
      ElMessage.error(response.error || '获取数据库列表失败')
    }
  } catch (error) {
    console.error('[FE] loadDatabases: 调用异常', error)
    debugHelper.log('error', '调用异常', error)
    ElMessage.error('获取数据库列表失败')
  }
}

const handleDatabaseClick = async (data: any) => {
  selectedItem.value = data

  if (data.type === 'database') {
    // 加载测量值
    await loadMeasurements(data.name)
  }
}

const handleDatabaseDoubleClick = (data: any) => {
  console.log('[FE] 数据库浏览器中检测到双击事件', {
    type: data.type,
    name: data.name,
    database: data.database,
    fullData: data
  });

  // 只有measurement类型才支持双击替换SQL
  if (data.type === 'measurement') {
    console.log(`[FE] 双击的是一个表: ${data.name}，位于数据库: ${data.database}`);
    // 触发事件，通知查询编辑器替换表名
    eventBus.emit(Events.DOUBLE_CLICK_TABLE, {
      tableName: data.name,
      database: data.database
    });
    ElMessage.success(`已选择表: ${data.name}`);
  } else if (data.type === 'database') {
    console.log(`[FE] 双击的是一个数据库: ${data.name}`);
    // 可以在此处添加针对双击数据库的逻辑，例如展开/折叠
    // 目前，我们只记录日志
  } else {
    console.log(`[FE] 双击了一个未知类型的节点: ${data.type}`);
  }
};

const loadMeasurements = async (database: string) => {
  console.log('[FE] loadMeasurements: 开始加载测量值', {
    database,
    hasActiveConnection: !!activeConnection.value,
    isConnected: isConnected.value,
    connectionStatus: connectionStatus.value
  })

  if (!activeConnection.value || !isConnected.value) {
    console.error('[FE] loadMeasurements: 连接检查失败', {
      hasActiveConnection: !!activeConnection.value,
      isConnected: isConnected.value
    })
    ElMessage.error('连接未建立，请先连接到数据库')
    return
  }

  const backendConnectionId = connectionStatus.value?.backendConnectionId

  if (!backendConnectionId) {
    console.error('[FE] loadMeasurements: backendConnectionId 不存在', {
      connectionStatus: connectionStatus.value
    })
    ElMessage.error('连接未建立，请先连接到数据库')
    return
  }

  try {
    console.log(`[FE] loadMeasurements: 正在获取数据库 ${database} 的测量值列表...`, {
      backendConnectionId,
      database
    })

    const response = await invoke('get_measurements', {
      connectionId: backendConnectionId,
      database
    }) as any

    console.log('[FE] loadMeasurements: 后端响应', response)

    if (response.success && response.data) {
      console.log('[FE] loadMeasurements: 成功获取测量值列表', response.data)
      // 更新树形数据
      const databaseNode = databaseTreeData.value.find(db => db.name === database)
      if (databaseNode) {
        databaseNode.children = response.data.map((measurement: string) => ({
          id: `${database}.${measurement}`,
          name: measurement,
          type: 'measurement',
          database,
          count: 0 // 简化实现
        }))
        console.log('[FE] loadMeasurements: 更新后的数据库节点', databaseNode)
        ElMessage.success(`成功加载 ${response.data.length} 个测量值`)
      } else {
        console.error('[FE] loadMeasurements: 未找到数据库节点', {
          database,
          availableDatabases: databaseTreeData.value.map(db => db.name)
        })
        ElMessage.error('未找到数据库节点')
      }
    } else {
      console.error('[FE] loadMeasurements: 后端返回错误', response.error)
      ElMessage.error(response.error || '获取测量值列表失败')
    }
  } catch (error) {
    console.error('[FE] loadMeasurements: 调用异常', error)
    ElMessage.error(`获取测量值列表失败: ${error instanceof Error ? error.message : '未知错误'}`)
  }
}

const showCreateDatabaseDialog = () => {
  isCreateDatabaseDialogVisible.value = true
}

const showShortcutHelp = () => {
  isShortcutHelpVisible.value = true
}

const handleDatabaseCreated = (database: any) => {
  // 刷新数据库列表
  loadDatabases()
  // 选中新创建的数据库
  selectedItem.value = database
}

const getStats = () => {
  if (!databaseTreeData.value.length) return undefined
  
  let databaseCount = 0
  let measurementCount = 0
  let seriesCount = 0
  let pointCount = 0
  
  databaseTreeData.value.forEach(db => {
    databaseCount++
    if (db.children) {
      measurementCount += db.children.length
      db.children.forEach((measurement: any) => {
        seriesCount += measurement.seriesCount || 0
        pointCount += measurement.pointCount || 0
      })
    }
  })
  
  return {
    databaseCount,
    measurementCount,
    seriesCount,
    pointCount
  }
}

const showDeleteDatabaseDialog = async () => {
  if (!selectedItem.value || selectedItem.value.type !== 'database') return

  try {
    await ElMessageBox.confirm(
      `确定要删除${getDatabaseNameLabel()} "${selectedItem.value.name}" 吗？`,
      '确认删除',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning',
      }
    )

    await deleteDatabase(selectedItem.value.name)
  } catch {
    // 用户取消删除
  }
}

const deleteDatabase = async (database: string) => {
  if (!activeConnection.value || !isConnected.value) return

  const backendConnectionId = connectionStatus.value?.backendConnectionId

  if (!backendConnectionId) {
    ElMessage.error('连接未建立，请先连接到数据库')
    return
  }

  try {
    const response = await invoke('drop_database', {
      connectionId: backendConnectionId,
      database
    }) as any

    if (response.success) {
      ElMessage.success(`${getDatabaseNameLabel()}删除成功`)
      selectedItem.value = null
      await loadDatabases()
    } else {
      ElMessage.error(response.error || `${getDatabaseNameLabel()}删除失败`)
    }
  } catch (error) {
    ElMessage.error(`${getDatabaseNameLabel()}删除失败`)
  }
}

const showDeleteMeasurementDialog = async () => {
  if (!selectedItem.value || selectedItem.value.type !== 'measurement') return

  try {
    await ElMessageBox.confirm(
      `确定要删除${getMeasurementNameLabel()} "${selectedItem.value.name}" 吗？`,
      '确认删除',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning',
      }
    )

    await deleteMeasurement(selectedItem.value.database, selectedItem.value.name)
  } catch {
    // 用户取消删除
  }
}

const deleteMeasurement = async (database: string, measurement: string) => {
  if (!activeConnection.value || !isConnected.value) return

  const backendConnectionId = connectionStatus.value?.backendConnectionId

  if (!backendConnectionId) {
    ElMessage.error('连接未建立，请先连接到数据库')
    return
  }

  try {
    // 简化实现：使用 DROP MEASUREMENT 查询
    const query = `DROP MEASUREMENT "${measurement}"`
    const response = await invoke('execute_query', {
      connectionId: backendConnectionId,
      database,
      query
    }) as any

    if (response.success) {
      ElMessage.success(`${getMeasurementNameLabel()}删除成功`)
      selectedItem.value = null
      await loadMeasurements(database)
    } else {
      ElMessage.error(response.error || `${getMeasurementNameLabel()}删除失败`)
    }
  } catch (error) {
    ElMessage.error(`${getMeasurementNameLabel()}删除失败`)
  }
}

const openQueryEditor = () => {
  if (!selectedItem.value) return

  const query: any = {
    database: selectedItem.value.type === 'database'
      ? selectedItem.value.name
      : selectedItem.value.database
  }

  if (selectedItem.value.type === 'measurement') {
    query.measurement = selectedItem.value.name
  }

  router.push({
    path: '/query',
    query
  })
}

const handleCreateMeasurement = (node: any) => {
  if (node && node.type === 'database') {
    // 这里可以添加创建测量值的逻辑
    ElMessage.info(`创建测量值功能尚未实现 (数据库: ${node.name})`)
  }
}

// const handleContextMenuAction = async (event: any) => {
//   console.log('[FE] 处理上下文菜单操作', event)

//   const { action, node } = event

//   switch (action) {
//     case 'refresh-all':
//       await refreshData()
//       break

//     case 'refresh':
//       if (node && node.type === 'database') {
//         await loadMeasurements(node.name)
//       } else {
//         await refreshData()
//       }
//       break

//     case 'create-database':
//       showCreateDatabaseDialog()
//       break

//     case 'delete-database':
//       if (node && node.type === 'database') {
//         selectedItem.value = node
//         await showDeleteDatabaseDialog()
//       }
//       break

//     case 'create-measurement':
//       if (node && node.type === 'database') {
//         handleCreateMeasurement(node)
//       }
//       break

//     case 'delete-measurement':
//       if (node && node.type === 'measurement') {
//         selectedItem.value = node
//         await showDeleteMeasurementDialog()
//       }
//       break

//     case 'query-data':
//       if (node) {
//         selectedItem.value = node
//         openQueryEditor()
//       }
//       break

//     default:
//       console.log('[FE] 未处理的上下文菜单操作', action)
//   }
// }

// 监听连接状态变化
watch(() => isConnected.value, (connected) => {
  console.log('[FE] 连接状态变化监听器', {
    connected,
    connectionStatus: connectionStatus.value,
    activeConnection: activeConnection.value
  })

  if (connected) {
    console.log('[FE] 连接成功，开始加载数据库列表')
    // 连接成功后自动加载数据
    loadDatabases()
  } else {
    console.log('[FE] 连接断开，清空数据')
    // 连接断开时清空数据
    databaseTreeData.value = []
    selectedItem.value = null
  }
})

// 生命周期
onMounted(() => {
  if (activeConnection.value && isConnected.value) {
    loadDatabases()
  }
})
</script>

<style scoped>
.database-explorer {
  height: 100vh;
  background-color: #f5f5f5;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  background-color: white;
  border-bottom: 1px solid #e4e7ed;
  padding: 0 20px;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 10px;
}

.header-left h2 {
  margin: 0;
  color: #303133;
}

.connection-info {
  display: flex;
  align-items: center;
  gap: 5px;
}

.header-right {
  display: flex;
  align-items: center;
}

.no-connection,
.not-connected {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 400px;
}

.connection-error {
  padding: 20px;
}

.explorer-content {
  padding: 20px;
}

.loading-container,
.empty-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 40px;
  color: #909399;
}

.loading-container .el-icon {
  font-size: 24px;
  margin-bottom: 10px;
}

.custom-tree-node {
  display: flex;
  align-items: center;
  gap: 5px;
}

.measurement-count {
  color: #909399;
  font-size: 12px;
}

.fade-enter-active, .fade-leave-active {
  transition: opacity 0.5s;
}
.fade-enter, .fade-leave-to {
  opacity: 0;
}
</style>