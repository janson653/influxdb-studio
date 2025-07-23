<template>
  <div class="home-container">
    <IDELayout>
      <!-- 使用默认的数据库树结构 -->
      
      <!-- 内容区域插槽 -->
      <template #content>
        <!-- 使用默认的SQL编辑器内容 -->
      </template>
      
      <!-- 使用默认的结果表格 -->
    </IDELayout>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { ElMessage } from 'element-plus'
import { useConnectionStore } from '../stores/connectionStore'
import { useDatabaseStore } from '../stores/databaseStore'
import IDELayout from '../components/Layout/IDELayout.vue'

// 状态管理
const connectionStore = useConnectionStore()
const databaseStore = useDatabaseStore()

// 响应式数据
const selectedDatabase = ref<string | null>(null)
const selectedMeasurement = ref<string | null>(null)

// 方法
const handleDatabaseSelected = (databaseName: string) => {
  selectedDatabase.value = databaseName
  selectedMeasurement.value = null
  console.log('选中数据库:', databaseName)
}

const handleMeasurementSelected = (databaseName: string, measurementName: string) => {
  selectedDatabase.value = databaseName
  selectedMeasurement.value = measurementName
  console.log('选中测量值:', databaseName, measurementName)
}

const handleQueryRequested = (databaseName: string, measurementName: string) => {
  selectedDatabase.value = databaseName
  selectedMeasurement.value = measurementName
  // 自动生成查询语句
  const query = `SELECT * FROM "${measurementName}" LIMIT 100`
  console.log('请求查询:', query)
  // TODO: 执行查询
}

// 生命周期
onMounted(async () => {
  // 加载保存的连接
  connectionStore.loadConnections()
  
  // 如果有活跃连接，自动连接
  const activeConnection = connectionStore.activeConnectionConfig
  if (activeConnection) {
    try {
      await connectionStore.connectTo(activeConnection.id)
      await databaseStore.fetchDatabases()
    } catch (error) {
      ElMessage.error('自动连接失败，请手动连接数据库')
    }
  }
})
</script>

<style scoped>
.home-container {
  height: 100vh;
  width: 100vw;
  overflow: hidden;
}
</style> 