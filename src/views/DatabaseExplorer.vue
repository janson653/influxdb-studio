<template>
  <div class="database-explorer">
    <el-container>
      <el-header class="header">
        <div class="header-left">
          <h2>数据库浏览器</h2>
          <el-tag v-if="activeConnection" type="success">
            {{ activeConnection.name }}
          </el-tag>
        </div>
        <div class="header-right">
          <el-button @click="refreshData" :loading="isLoading">
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
        
        <div v-else class="explorer-content">
          <el-row :gutter="20">
            <el-col :span="8">
              <!-- 数据库列表 -->
              <el-card>
                <template #header>
                  <span>数据库</span>
                  <el-button 
                    style="float: right; padding: 3px 0" 
                    type="text"
                    @click="showCreateDatabaseDialog"
                  >
                    新建
                  </el-button>
                </template>
                
                <el-tree
                  ref="databaseTree"
                  :data="databaseTreeData"
                  :props="treeProps"
                  node-key="id"
                  :expand-on-click-node="false"
                  @node-click="handleDatabaseClick"
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
                  <span>{{ selectedItem.type === 'database' ? '数据库信息' : '测量值信息' }}</span>
                </template>
                
                <div v-if="selectedItem.type === 'database'">
                  <el-descriptions :column="2" border>
                    <el-descriptions-item label="数据库名称">
                      {{ selectedItem.name }}
                    </el-descriptions-item>
                    <el-descriptions-item label="测量值数量">
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
                    <el-button type="warning" @click="showDeleteDatabaseDialog">
                      删除数据库
                    </el-button>
                  </div>
                </div>
                
                <div v-else-if="selectedItem.type === 'measurement'">
                  <el-descriptions :column="2" border>
                    <el-descriptions-item label="测量值名称">
                      {{ selectedItem.name }}
                    </el-descriptions-item>
                    <el-descriptions-item label="所属数据库">
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
                    <el-button type="warning" @click="showDeleteMeasurementDialog">
                      删除测量值
                    </el-button>
                  </div>
                </div>
              </el-card>
              
              <el-card v-else>
                <template #header>
                  <span>欢迎</span>
                </template>
                <el-empty description="请选择一个数据库或测量值查看详细信息" />
              </el-card>
            </el-col>
          </el-row>
        </div>
      </el-main>
    </el-container>
    
    <!-- 新建数据库对话框 -->
    <el-dialog v-model="showCreateDatabase" title="新建数据库" width="400px">
      <el-form :model="newDatabaseForm" :rules="databaseRules" ref="databaseFormRef">
        <el-form-item label="数据库名称" prop="name">
          <el-input v-model="newDatabaseForm.name" placeholder="请输入数据库名称" />
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
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage, ElMessageBox } from 'element-plus'
import { 
  Refresh, 
  Folder, 
  Document, 
  FolderOpened 
} from '@element-plus/icons-vue'
import { useConnectionStore } from '../stores/connectionStore'

// 路由
const router = useRouter()

// 连接状态管理
const connectionStore = useConnectionStore()

// 响应式数据
const isLoading = ref(false)
const selectedItem = ref<any>(null)
const activeTab = ref('tags')
const showCreateDatabase = ref(false)
const newDatabaseForm = ref({ name: '' })
const databaseFormRef = ref()

// 树形数据配置
const treeProps = {
  children: 'children',
  label: 'name'
}

// 计算属性
const activeConnection = computed(() => connectionStore.activeConnectionConfig)

const databaseTreeData = computed(() => {
  if (!activeConnection.value) return []
  
  // 模拟数据库树形数据
  return [
    {
      id: 'db1',
      name: 'test_db',
      type: 'database',
      measurementCount: 3,
      seriesCount: 10,
      pointCount: 1000,
      children: [
        {
          id: 'm1',
          name: 'cpu_usage',
          type: 'measurement',
          database: 'test_db',
          count: 100,
          tagKeys: [
            { key: 'host', values: ['server1', 'server2', 'server3'] },
            { key: 'region', values: ['us-west', 'us-east'] }
          ],
          fieldKeys: [
            { key: 'value', type: 'float' },
            { key: 'unit', type: 'string' }
          ]
        },
        {
          id: 'm2',
          name: 'memory_usage',
          type: 'measurement',
          database: 'test_db',
          count: 50,
          tagKeys: [
            { key: 'host', values: ['server1', 'server2'] }
          ],
          fieldKeys: [
            { key: 'value', type: 'float' },
            { key: 'unit', type: 'string' }
          ]
        }
      ]
    }
  ]
})

// 数据库表单验证规则
const databaseRules = {
  name: [
    { required: true, message: '请输入数据库名称', trigger: 'blur' },
    { pattern: /^[a-zA-Z_][a-zA-Z0-9_]*$/, message: '数据库名称格式不正确', trigger: 'blur' }
  ]
}

// 方法
const refreshData = async () => {
  isLoading.value = true
  try {
    // TODO: 刷新数据库列表
    await new Promise(resolve => setTimeout(resolve, 1000))
    ElMessage.success('数据已刷新')
  } catch (error) {
    ElMessage.error('刷新失败')
  } finally {
    isLoading.value = false
  }
}

const handleDatabaseClick = (data: any) => {
  selectedItem.value = data
}

const showCreateDatabaseDialog = () => {
  showCreateDatabase.value = true
  newDatabaseForm.value.name = ''
}

const createDatabase = async () => {
  try {
    await databaseFormRef.value?.validate()
    // TODO: 调用 Tauri 命令创建数据库
    ElMessage.success('数据库创建成功')
    showCreateDatabase.value = false
    refreshData()
  } catch (error) {
    console.error('创建数据库失败:', error)
  }
}

const openQueryEditor = () => {
  if (selectedItem.value) {
    // 跳转到查询编辑器，并传递选中的数据库/测量值信息
    router.push({
      path: '/query',
      query: {
        database: selectedItem.value.database || selectedItem.value.name,
        measurement: selectedItem.value.type === 'measurement' ? selectedItem.value.name : undefined
      }
    })
  }
}

const showDeleteDatabaseDialog = async () => {
  if (!selectedItem.value || selectedItem.value.type !== 'database') return
  
  try {
    await ElMessageBox.confirm(
      `确定要删除数据库 "${selectedItem.value.name}" 吗？此操作不可恢复！`,
      '确认删除',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning',
      }
    )
    
    // TODO: 调用 Tauri 命令删除数据库
    ElMessage.success('数据库删除成功')
    refreshData()
  } catch {
    // 用户取消删除
  }
}

const showDeleteMeasurementDialog = async () => {
  if (!selectedItem.value || selectedItem.value.type !== 'measurement') return
  
  try {
    await ElMessageBox.confirm(
      `确定要删除测量值 "${selectedItem.value.name}" 吗？此操作不可恢复！`,
      '确认删除',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning',
      }
    )
    
    // TODO: 调用 Tauri 命令删除测量值
    ElMessage.success('测量值删除成功')
    refreshData()
  } catch {
    // 用户取消删除
  }
}

// 生命周期
onMounted(() => {
  if (!activeConnection.value) {
    ElMessage.warning('请先连接到数据库')
  } else {
    // 延迟加载数据，确保组件完全挂载
    setTimeout(() => {
      refreshData()
    }, 100)
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
  gap: 15px;
}

.header-left h2 {
  margin: 0;
  color: #303133;
}

.no-connection {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 400px;
}

.explorer-content {
  height: calc(100vh - 80px);
}

.custom-tree-node {
  display: flex;
  align-items: center;
  gap: 8px;
  width: 100%;
}

.measurement-count {
  color: #909399;
  font-size: 0.9rem;
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
}
</style> 