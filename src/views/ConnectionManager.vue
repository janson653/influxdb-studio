<template>
  <div class="connection-manager">
    <el-container>
      <el-header class="header">
        <h2>连接管理</h2>
        <el-button type="primary" @click="showAddDialog">
          <el-icon><Plus /></el-icon>
          新建连接
        </el-button>
      </el-header>
      
      <el-main>
        <el-row :gutter="20">
          <el-col :span="16">
            <!-- 连接列表 -->
            <el-card>
              <template #header>
                <span>连接列表</span>
                <el-button 
                  v-if="connections.length > 0"
                  style="float: right; padding: 3px 0" 
                  type="text"
                  @click="testAllConnections"
                >
                  测试所有连接
                </el-button>
              </template>
              
              <div v-if="connections.length === 0" class="empty-state">
                <el-empty description="暂无连接配置">
                  <el-button type="primary" @click="showAddDialog">
                    添加第一个连接
                  </el-button>
                </el-empty>
              </div>
              
              <el-table 
                v-else
                :data="connections" 
                style="width: 100%"
                @row-click="handleRowClick"
              >
                <el-table-column prop="name" label="连接名称" width="200">
                  <template #default="{ row }">
                    <div class="connection-name">
                      <el-icon :color="getStatusColor(row.id)">
                        <CircleCheck v-if="getConnectionStatus(row.id) === 'connected'" />
                        <CircleClose v-else-if="getConnectionStatus(row.id) === 'error'" />
                        <Loading v-else-if="getConnectionStatus(row.id) === 'connecting'" />
                        <CircleCheckFilled v-else />
                      </el-icon>
                      <span>{{ row.name }}</span>
                    </div>
                  </template>
                </el-table-column>
                
                <el-table-column prop="host" label="主机地址" width="150" />
                <el-table-column prop="port" label="端口" width="80" />
                <el-table-column prop="database" label="默认数据库" width="120" />
                
                <el-table-column label="状态" width="100">
                  <template #default="{ row }">
                    <el-tag :type="getStatusType(row.id)">
                      {{ getStatusText(row.id) }}
                    </el-tag>
                  </template>
                </el-table-column>
                
                <el-table-column label="操作" width="200">
                  <template #default="{ row }">
                    <el-button-group>
                      <el-button 
                        size="small" 
                        @click.stop="testConnection(row.id)"
                        :loading="getConnectionStatus(row.id) === 'connecting'"
                      >
                        测试
                      </el-button>
                      <el-button 
                        size="small" 
                        type="primary" 
                        @click.stop="connectTo(row.id)"
                        :loading="getConnectionStatus(row.id) === 'connecting'"
                        :disabled="getConnectionStatus(row.id) === 'connecting'"
                      >
                        连接
                      </el-button>
                      <el-button 
                        size="small" 
                        type="warning" 
                        @click.stop="editConnection(row)"
                      >
                        编辑
                      </el-button>
                      <el-button 
                        size="small" 
                        type="danger" 
                        @click.stop="removeConnection(row.id)"
                      >
                        删除
                      </el-button>
                    </el-button-group>
                  </template>
                </el-table-column>
              </el-table>
            </el-card>
          </el-col>
          
          <el-col :span="8">
            <!-- 连接详情 -->
            <el-card v-if="selectedConnection">
              <template #header>
                <span>连接详情</span>
              </template>
              
              <el-descriptions :column="1" border>
                <el-descriptions-item label="连接名称">
                  {{ selectedConnection.name }}
                </el-descriptions-item>
                <el-descriptions-item label="主机地址">
                  {{ selectedConnection.host }}
                </el-descriptions-item>
                <el-descriptions-item label="端口">
                  {{ selectedConnection.port }}
                </el-descriptions-item>
                <el-descriptions-item label="默认数据库">
                  {{ selectedConnection.database || '未设置' }}
                </el-descriptions-item>
                <el-descriptions-item label="用户名">
                  {{ selectedConnection.username || '未设置' }}
                </el-descriptions-item>
                <el-descriptions-item label="SSL">
                  {{ selectedConnection.useSsl ? '启用' : '禁用' }}
                </el-descriptions-item>
                <el-descriptions-item label="超时时间">
                  {{ selectedConnection.timeout }}ms
                </el-descriptions-item>
              </el-descriptions>
              
              <div style="margin-top: 20px;">
                <el-button 
                  type="primary" 
                  @click="connectTo(selectedConnection.id)"
                  :loading="getConnectionStatus(selectedConnection.id) === 'connecting'"
                  :disabled="getConnectionStatus(selectedConnection.id) === 'connecting'"
                  style="width: 100%"
                >
                  连接到数据库
                </el-button>
              </div>
            </el-card>
            
            <!-- 快速操作 -->
            <el-card v-else>
              <template #header>
                <span>快速操作</span>
              </template>
              
              <el-space direction="vertical" style="width: 100%">
                <el-button type="primary" @click="showAddDialog" style="width: 100%">
                  新建连接
                </el-button>
                <el-button @click="importConnections" style="width: 100%">
                  导入连接
                </el-button>
                <el-button @click="exportConnections" style="width: 100%">
                  导出连接
                </el-button>
              </el-space>
            </el-card>
          </el-col>
        </el-row>
      </el-main>
    </el-container>
    
    <!-- 添加/编辑连接对话框 -->
    <ConnectionDialog 
      v-model="showDialog"
      :connection="editingConnection"
      @save="handleSaveConnection"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage, ElMessageBox } from 'element-plus'
import { 
  Plus, 
  CircleCheck, 
  CircleClose, 
  Loading, 
  CircleCheckFilled 
} from '@element-plus/icons-vue'
import { useConnectionStore } from '../stores/connectionStore'
import type { ConnectionConfig } from '../stores/connectionStore'
import ConnectionDialog from '../components/Connection/ConnectionDialog.vue'

// 路由
const router = useRouter()

// 连接状态管理
const connectionStore = useConnectionStore()

// 响应式数据
const showDialog = ref(false)
const editingConnection = ref<ConnectionConfig | null>(null)
const selectedConnection = ref<ConnectionConfig | null>(null)

// 计算属性
const connections = computed(() => connectionStore.connections)

// 方法
const showAddDialog = () => {
  editingConnection.value = null
  showDialog.value = true
}

const editConnection = (connection: ConnectionConfig) => {
  editingConnection.value = { ...connection }
  showDialog.value = true
}

const handleSaveConnection = (connection: ConnectionConfig) => {
  connectionStore.addConnection(connection)
  ElMessage.success('连接配置已保存')
  showDialog.value = false
}

const removeConnection = async (id: string) => {
  try {
    await ElMessageBox.confirm(
      '确定要删除这个连接配置吗？',
      '确认删除',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning',
      }
    )
    
    connectionStore.removeConnection(id)
    if (selectedConnection.value?.id === id) {
      selectedConnection.value = null
    }
    ElMessage.success('连接配置已删除')
  } catch {
    // 用户取消删除
  }
}

const testConnection = async (id: string) => {
  try {
    const success = await connectionStore.testConnection(id)
    if (success) {
      ElMessage.success('连接测试成功')
    } else {
      ElMessage.error('连接测试失败')
    }
  } catch (error) {
    ElMessage.error(`连接测试失败: ${error}`)
  }
}

const connectTo = async (id: string) => {
  try {
    const success = await connectionStore.connectTo(id)
    if (success) {
      ElMessage.success('连接成功')
      router.push('/database')
    } else {
      ElMessage.error('连接失败')
    }
  } catch (error) {
    ElMessage.error(`连接失败: ${error}`)
  }
}

const testAllConnections = async () => {
  ElMessage.info('开始测试所有连接...')
  for (const connection of connections.value) {
    await connectionStore.testConnection(connection.id)
  }
  ElMessage.success('所有连接测试完成')
}

const handleRowClick = (row: ConnectionConfig) => {
  selectedConnection.value = row
}

const getConnectionStatus = (id: string) => {
  return connectionStore.connectionStatus[id]?.status || 'disconnected'
}

const getStatusColor = (id: string) => {
  const status = getConnectionStatus(id)
  switch (status) {
    case 'connected': return '#67C23A'
    case 'error': return '#F56C6C'
    case 'connecting': return '#E6A23C'
    default: return '#909399'
  }
}

const getStatusType = (id: string) => {
  const status = getConnectionStatus(id)
  switch (status) {
    case 'connected': return 'success'
    case 'error': return 'danger'
    case 'connecting': return 'warning'
    default: return 'info'
  }
}

const getStatusText = (id: string) => {
  const status = getConnectionStatus(id)
  switch (status) {
    case 'connected': return '已连接'
    case 'error': return '连接错误'
    case 'connecting': return '连接中'
    default: return '未连接'
  }
}

const importConnections = () => {
  // TODO: 实现导入功能
  ElMessage.info('导入功能开发中...')
}

const exportConnections = () => {
  // TODO: 实现导出功能
  ElMessage.info('导出功能开发中...')
}
</script>

<style scoped>
.connection-manager {
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

.header h2 {
  margin: 0;
  color: #303133;
}

.empty-state {
  padding: 40px 0;
}

.connection-name {
  display: flex;
  align-items: center;
  gap: 8px;
}

.el-table {
  margin-top: 10px;
}

.el-table .el-table__row {
  cursor: pointer;
}

.el-table .el-table__row:hover {
  background-color: #f5f7fa;
}
</style> 