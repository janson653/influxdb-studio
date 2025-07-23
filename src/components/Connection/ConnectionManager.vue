<template>
  <el-dialog
    v-model="visible"
    title="连接管理"
    width="800px"
    @close="handleClose"
  >
    <div class="connection-manager">
      <!-- 连接列表 -->
      <div class="connection-list">
        <div class="list-header">
          <h3>已保存的连接</h3>
          <el-button type="primary" size="small" @click="showCreateDialog = true">
            <el-icon><Plus /></el-icon>
            新建连接
          </el-button>
        </div>
        
        <div class="connection-items">
          <div 
            v-for="connection in connections" 
            :key="connection.id"
            class="connection-item"
            :class="{ active: connection.id === activeConnectionId }"
            @click="selectConnection(connection)"
          >
            <div class="connection-info">
              <div class="connection-name">{{ connection.name }}</div>
              <div class="connection-details">
                {{ getConnectionUrl(connection) }}
              </div>
            </div>
            <div class="connection-actions">
              <el-button 
                v-if="connection.id === activeConnectionId"
                type="success" 
                size="small"
                disabled
              >
                已连接
              </el-button>
              <el-button 
                v-else
                type="primary" 
                size="small"
                @click.stop="connectTo(connection.id)"
                :loading="connectingId === connection.id"
              >
                连接
              </el-button>
              <el-button 
                type="default" 
                size="small"
                @click.stop="editConnection(connection)"
              >
                编辑
              </el-button>
              <el-button 
                type="danger" 
                size="small"
                @click.stop="deleteConnection(connection.id)"
              >
                删除
              </el-button>
            </div>
          </div>
          
          <div v-if="connections.length === 0" class="empty-state">
            <el-empty description="暂无保存的连接">
              <el-button type="primary" @click="showCreateDialog = true">
                创建第一个连接
              </el-button>
            </el-empty>
          </div>
        </div>
      </div>
    </div>
    
    <!-- 创建/编辑连接对话框 -->
    <ConnectionDialog 
      v-model="showCreateDialog"
      :connection="editingConnection"
      @save="handleConnectionSave"
    />
  </el-dialog>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Plus } from '@element-plus/icons-vue'
import { useConnectionStore } from '../../stores/connectionStore'
import type { ConnectionProfile } from '../../types/influxdb'
import { getConnectionUrl } from '../../types/influxdb'
import ConnectionDialog from './ConnectionDialog.vue'

// Props
interface Props {
  modelValue: boolean
}

const props = defineProps<Props>()

// Emits
const emit = defineEmits<{
  'update:modelValue': [value: boolean]
  'connection-selected': [connectionId: string]
}>()

// Stores
const connectionStore = useConnectionStore()

// 响应式数据
const showCreateDialog = ref(false)
const editingConnection = ref<ConnectionProfile | null>(null)
const connectingId = ref<string | null>(null)

// 计算属性
const visible = computed({
  get: () => props.modelValue,
  set: (value) => emit('update:modelValue', value)
})

const connections = computed(() => connectionStore.connections)
const activeConnectionId = computed(() => connectionStore.activeConnectionId)

// 方法
const selectConnection = (connection: ConnectionProfile) => {
  emit('connection-selected', connection.id)
}

const connectTo = async (connectionId: string) => {
  try {
    connectingId.value = connectionId
    const success = await connectionStore.connectTo(connectionId)
    
    if (success) {
      ElMessage.success('连接成功')
      emit('connection-selected', connectionId)
    } else {
      ElMessage.error('连接失败')
    }
  } catch (error: any) {
    ElMessage.error(`连接失败: ${error.message || error}`)
  } finally {
    connectingId.value = null
  }
}

const editConnection = (connection: ConnectionProfile) => {
  editingConnection.value = connection
  showCreateDialog.value = true
}

const deleteConnection = async (connectionId: string) => {
  try {
    await ElMessageBox.confirm(
      '确定要删除这个连接吗？此操作不可恢复。',
      '确认删除',
      {
        confirmButtonText: '删除',
        cancelButtonText: '取消',
        type: 'warning'
      }
    )
    
    connectionStore.removeConnection(connectionId)
    ElMessage.success('连接已删除')
  } catch (error) {
    // 用户取消删除
  }
}

const handleConnectionSave = (connection: ConnectionProfile) => {
  if (editingConnection.value) {
    // 编辑模式
    connectionStore.updateConnection(editingConnection.value.id, connection)
    ElMessage.success('连接已更新')
  } else {
    // 新建模式
    connectionStore.addConnection(connection)
    ElMessage.success('连接已创建')
  }
  
  showCreateDialog.value = false
  editingConnection.value = null
}

const handleClose = () => {
  visible.value = false
  showCreateDialog.value = false
  editingConnection.value = null
}

// 监听对话框打开
watch(visible, (newVal) => {
  if (newVal) {
    // 刷新连接列表
    connectionStore.loadConnections()
  }
})
</script>

<style scoped>
.connection-manager {
  max-height: 500px;
  overflow-y: auto;
}

.connection-list {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.list-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.list-header h3 {
  margin: 0;
  color: var(--ide-text-primary);
}

.connection-items {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.connection-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 15px;
  background-color: var(--ide-bg-secondary);
  border: 1px solid var(--ide-border);
  border-radius: var(--ide-border-radius-lg);
  cursor: pointer;
  transition: all 0.2s ease;
}

.connection-item:hover {
  background-color: var(--ide-bg-tertiary);
  border-color: var(--ide-border-light);
}

.connection-item.active {
  border-color: var(--ide-accent-primary);
  background-color: var(--ide-bg-tertiary);
}

.connection-info {
  flex: 1;
}

.connection-name {
  font-weight: 500;
  color: var(--ide-text-primary);
  margin-bottom: 5px;
}

.connection-details {
  font-size: 12px;
  color: var(--ide-text-secondary);
}

.connection-actions {
  display: flex;
  gap: 8px;
}

.empty-state {
  padding: 40px;
  text-align: center;
}

:deep(.el-dialog) {
  background-color: var(--ide-bg-secondary);
}

:deep(.el-dialog__title) {
  color: var(--ide-text-primary);
}

:deep(.el-empty__description) {
  color: var(--ide-text-secondary);
}
</style> 