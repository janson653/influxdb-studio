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
          <el-button @click="connectToDatabase" v-if="!isConnected" type="primary">
            <el-icon><Connection /></el-icon>
            连接数据库
          </el-button>
          <el-button @click="refreshData" :loading="isLoading" v-else>
            <el-icon><Refresh /></el-icon>
            刷新
          </el-button>
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
          <el-alert
            :title="`连接错误: ${connectionError}`"
            type="error"
            :closable="false"
            show-icon
          >
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
        
        <div v-else class="explorer-content">
          <el-row :gutter="20">
            <el-col :span="8">
              <!-- 数据库/存储桶列表 -->
              <el-card>
                <template #header>
                  <span>{{ getDatabaseListTitle() }}</span>
                  <el-button 
                    v-if="canCreateDatabase()"
                    style="float: right; padding: 3px 0" 
                    link
                    @click="showCreateDatabaseDialog"
                  >
                    新建
                  </el-button>
                </template>
                
                <div v-if="isLoading" class="loading-container">
                  <el-icon class="is-loading"><Loading /></el-icon>
                  <span>正在加载...</span>
                </div>
                
                <div v-else-if="databaseTreeData.length === 0" class="empty-container">
                  <el-empty :description="`暂无${getDatabaseNameLabel()}`" />
                </div>
                
                <el-tree
                  v-else
                  ref="databaseTree"
                  :data="databaseTreeData"
                  :props="treeProps"
                  node-key="id"
                  :expand-on-click-node="false"
                  @node-click="handleDatabaseClick"
                  @node-dblclick="handleDatabaseDoubleClick"
                >
                  <template #default="{ node, data }">
                    <span class="custom-tree-node">
                      <el-icon v-if="data.type === 'database'">
                        <Folder />
                      </el-icon>
                      <el-icon v-else-if="data.type === 'measurement'">
                        <Document />
                      </el-icon>
                      <el-icon v-else>
                        <FolderOpened />
                      </el-icon>
                      <span>{{ node.label }}</span>
                      <span v-if="data.type === 'measurement'" class="measurement-count">
                        ({{ data.count || 0 }})
                      </span>
                    </span>
                  </template>
                </el-tree>
              </el-card>
            </el-col>
            
            <el-col :span="16">
              <!-- 详细信息 -->
              <el-card v-if="selectedItem">
                <template #header>
                  <span>{{ getDetailTitle() }}</span>
                </template>
                
                <div v-if="selectedItem.type === 'database'">
                  <el-descriptions :column="2" border>
                    <el-descriptions-item :label="getDatabaseNameLabel()">
                      {{ selectedItem.name }}
                    </el-descriptions-item>
                    <el-descriptions-item :label="getMeasurementCountLabel()">
                      {{ selectedItem.measurementCount || 0 }}
                    </el-descriptions-item>
                    <el-descriptions-item label="系列数量">
                      {{ selectedItem.seriesCount || 0 }}
                    </el-descriptions-item>
                    <el-descriptions-item label="数据点数量">
                      {{ selectedItem.pointCount || 0 }}
                    </el-descriptions-item>
                  </el-descriptions>
                  
                  <div style="margin-top: 20px;">
                    <el-button type="primary" @click="openQueryEditor">
                      查询数据
                    </el-button>
                    <el-button 
                      v-if="canDeleteDatabase()"
                      type="warning" 
                      @click="showDeleteDatabaseDialog"
                    >
                      删除{{ getDatabaseNameLabel() }}
                    </el-button>
                  </div>
                </div>
                
                <div v-else-if="selectedItem.type === 'measurement'">
                  <el-descriptions :column="2" border>
                    <el-descriptions-item :label="getMeasurementNameLabel()">
                      {{ selectedItem.name }}
                    </el-descriptions-item>
                    <el-descriptions-item :label="getDatabaseNameLabel()">
                      {{ selectedItem.database }}
                    </el-descriptions-item>
                    <el-descriptions-item label="标签键数量">
                      {{ selectedItem.tagKeys?.length || 0 }}
                    </el-descriptions-item>
                    <el-descriptions-item label="字段键数量">
                      {{ selectedItem.fieldKeys?.length || 0 }}
                    </el-descriptions-item>
                  </el-descriptions>
                  
                  <el-tabs v-model="activeTab" style="margin-top: 20px;">
                    <el-tab-pane label="标签" name="tags">
                      <el-table :data="selectedItem.tagKeys || []" style="width: 100%">
                        <el-table-column prop="key" label="标签键" />
                        <el-table-column prop="values" label="标签值">
                          <template #default="{ row }">
                            <el-tag 
                              v-for="value in row.values.slice(0, 5)" 
                              :key="value"
                              size="small"
                              style="margin-right: 5px;"
                            >
                              {{ value }}
                            </el-tag>
                            <span v-if="row.values.length > 5" style="color: #909399;">
                              +{{ row.values.length - 5 }} 更多
                            </span>
                          </template>
                        </el-table-column>
                      </el-table>
                    </el-tab-pane>
                    
                    <el-tab-pane label="字段" name="fields">
                      <el-table :data="selectedItem.fieldKeys || []" style="width: 100%">
                        <el-table-column prop="key" label="字段键" />
                        <el-table-column prop="type" label="数据类型" />
                      </el-table>
                    </el-tab-pane>
                  </el-tabs>
                  
                  <div style="margin-top: 20px;">
                    <el-button type="primary" @click="openQueryEditor">
                      查询数据
                    </el-button>
                    <el-button 
                      v-if="canDeleteMeasurement()"
                      type="warning" 
                      @click="showDeleteMeasurementDialog"
                    >
                      删除{{ getMeasurementNameLabel() }}
                    </el-button>
                  </div>
                </div>
              </el-card>
              
              <el-card v-else>
                <template #header>
                  <span>欢迎</span>
                </template>
                <el-empty :description="`请选择一个${getDatabaseNameLabel()}或${getMeasurementNameLabel()}查看详细信息`" />
              </el-card>
            </el-col>
          </el-row>
        </div>
      </el-main>
    </el-container>
    
    <!-- 新建数据库对话框 -->
    <el-dialog v-model="showCreateDatabase" :title="`新建${getDatabaseNameLabel()}`" width="400px">
      <el-form :model="newDatabaseForm" :rules="databaseRules" ref="databaseFormRef">
        <el-form-item :label="`${getDatabaseNameLabel()}名称`" prop="name">
          <el-input v-model="newDatabaseForm.name" :placeholder="`请输入${getDatabaseNameLabel()}名称`" />
        </el-form-item>
      </el-form>
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="showCreateDatabase = false">取消</el-button>
          <el-button type="primary" @click="createDatabase">创建</el-button>
        </span>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch, reactive } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { ElMessage, ElMessageBox } from 'element-plus'
import {
  Refresh, Folder, Document, Setting, Plus, Link, Edit, Delete, ArrowLeft,
  Connection, Loading, FolderOpened
} from '@element-plus/icons-vue'
import { invoke } from '@tauri-apps/api/core'
import { useConnectionStore } from '../stores/connectionStore'
import { InfluxDBVersion } from '../types/influxdb'
import { eventBus, Events } from '../utils/eventBus'
import { debugHelper } from '../utils/debugHelper'

// 路由
const router = useRouter()

// 状态管理
const connectionStore = useConnectionStore()

// 响应式数据
const isLoading = ref(false)
const isConnecting = ref(false)
const selectedItem = ref<any>(null)
const activeTab = ref('tags')
const showCreateDatabase = ref(false)
const newDatabaseForm = ref({
  name: ''
})

// 数据库树数据
const databaseTreeData = ref<any[]>([])

// 表单验证规则
const databaseRules = {
  name: [
    { required: true, message: '请输入名称', trigger: 'blur' },
    { min: 1, max: 50, message: '名称长度在 1 到 50 个字符', trigger: 'blur' }
  ]
}

// 树配置
const treeProps = {
  children: 'children',
  label: 'name'
}

// 计算属性
const activeConnection = computed(() => connectionStore.activeConnectionConfig)

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
  
  switch (activeConnection.value.version) {
    case InfluxDBVersion.V1: return '数据库浏览器'
    case InfluxDBVersion.V2: return '存储桶浏览器'
    case InfluxDBVersion.V3: return '数据库浏览器'
    default: return '数据库浏览器'
  }
}

const getDatabaseListTitle = () => {
  if (!activeConnection.value) return '数据库'
  
  switch (activeConnection.value.version) {
    case InfluxDBVersion.V1: return '数据库'
    case InfluxDBVersion.V2: return '存储桶'
    case InfluxDBVersion.V3: return '数据库'
    default: return '数据库'
  }
}

const getDatabaseNameLabel = () => {
  if (!activeConnection.value) return '数据库'
  
  switch (activeConnection.value.version) {
    case InfluxDBVersion.V1: return '数据库'
    case InfluxDBVersion.V2: return '存储桶'
    case InfluxDBVersion.V3: return '数据库'
    default: return '数据库'
  }
}

const getMeasurementNameLabel = () => {
  if (!activeConnection.value) return '测量值'
  
  switch (activeConnection.value.version) {
    case InfluxDBVersion.V1: return '测量值'
    case InfluxDBVersion.V2: return '测量值'
    case InfluxDBVersion.V3: return '表'
    default: return '测量值'
  }
}

const getMeasurementCountLabel = () => {
  if (!activeConnection.value) return '测量值数量'
  
  switch (activeConnection.value.version) {
    case InfluxDBVersion.V1: return '测量值数量'
    case InfluxDBVersion.V2: return '测量值数量'
    case InfluxDBVersion.V3: return '表数量'
    default: return '测量值数量'
  }
}

const getDetailTitle = () => {
  if (!selectedItem.value) return ''
  
  if (selectedItem.value.type === 'database') {
    return `${getDatabaseNameLabel()}信息`
  } else {
    return `${getMeasurementNameLabel()}信息`
  }
}

const canCreateDatabase = () => {
  if (!activeConnection.value) return false
  
  // InfluxDB 2.x 通常不允许通过 API 创建 bucket
  return activeConnection.value.version !== InfluxDBVersion.V2
}

const canDeleteDatabase = () => {
  if (!activeConnection.value) return false
  
  // InfluxDB 2.x 通常不允许通过 API 删除 bucket
  return activeConnection.value.version !== InfluxDBVersion.V2
}

const canDeleteMeasurement = () => {
  if (!activeConnection.value) return false
  
  // 简化实现，实际应该根据权限判断
  return true
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
  newDatabaseForm.value.name = ''
  showCreateDatabase.value = true
}

const createDatabase = async () => {
  if (!activeConnection.value || !newDatabaseForm.value.name || !isConnected.value) return
  
  const backendConnectionId = connectionStatus.value?.backendConnectionId
  
  if (!backendConnectionId) {
    ElMessage.error('连接未建立，请先连接到数据库')
    return
  }
  
  try {
    const response = await invoke('create_database', { 
      connectionId: backendConnectionId,
      database: newDatabaseForm.value.name
    }) as any
    
    if (response.success) {
      ElMessage.success(`${getDatabaseNameLabel()}创建成功`)
      showCreateDatabase.value = false
      await loadDatabases()
    } else {
      ElMessage.error(response.error || `${getDatabaseNameLabel()}创建失败`)
    }
  } catch (error) {
    ElMessage.error(`${getDatabaseNameLabel()}创建失败`)
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
</style> 