<template>
  <div class="home">
    <el-container>
      <el-header class="header">
        <h1>InfluxDB Studio</h1>
        <p>现代化的 InfluxDB 数据库管理工具</p>
      </el-header>
      
      <el-main class="main">
        <el-row :gutter="20">
          <el-col :span="8">
            <el-card class="feature-card" @click="navigateTo('/connection')">
              <div class="card-content">
                <el-icon size="48" color="#409EFF">
                  <Connection />
                </el-icon>
                <h3>连接管理</h3>
                <p>管理 InfluxDB 服务器连接，支持多连接并发</p>
              </div>
            </el-card>
          </el-col>
          
          <el-col :span="8">
            <el-card class="feature-card" @click="navigateTo('/database')">
              <div class="card-content">
                <el-icon size="48" color="#67C23A">
                  <DataAnalysis />
                </el-icon>
                <h3>数据库浏览器</h3>
                <p>浏览和管理数据库、测量值、标签和字段</p>
              </div>
            </el-card>
          </el-col>
          
          <el-col :span="8">
            <el-card class="feature-card" @click="navigateTo('/query')">
              <div class="card-content">
                <el-icon size="48" color="#E6A23C">
                  <Edit />
                </el-icon>
                <h3>查询编辑器</h3>
                <p>强大的查询编辑器，支持语法高亮和自动补全</p>
              </div>
            </el-card>
          </el-col>
        </el-row>
        
        <el-row :gutter="20" style="margin-top: 20px;">
          <el-col :span="12">
            <el-card>
              <template #header>
                <span>最近连接</span>
              </template>
              <div v-if="recentConnections.length === 0" class="empty-state">
                <el-empty description="暂无连接记录" />
              </div>
              <el-list v-else>
                <el-list-item 
                  v-for="connection in recentConnections" 
                  :key="connection.id"
                  @click="connectTo(connection)"
                >
                  <el-icon><Connection /></el-icon>
                  <span>{{ connection.name }}</span>
                  <span class="connection-info">{{ connection.host }}:{{ connection.port }}</span>
                </el-list-item>
              </el-list>
            </el-card>
          </el-col>
          
          <el-col :span="12">
            <el-card>
              <template #header>
                <span>快速操作</span>
              </template>
              <el-button-group style="width: 100%;">
                <el-button type="primary" @click="navigateTo('/connection')">
                  新建连接
                </el-button>
                <el-button @click="navigateTo('/query')">
                  开始查询
                </el-button>
                <el-button @click="navigateTo('/settings')">
                  设置
                </el-button>
              </el-button-group>
            </el-card>
          </el-col>
        </el-row>
      </el-main>
    </el-container>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { Connection, DataAnalysis, Edit } from '@element-plus/icons-vue'

// 路由导航
const router = useRouter()

// 最近连接数据
const recentConnections = ref([
  {
    id: '1',
    name: '本地 InfluxDB',
    host: 'localhost',
    port: 8086
  }
])

// 导航方法
const navigateTo = (path: string) => {
  router.push(path)
}

// 连接到指定服务器
const connectTo = (connection: any) => {
  // TODO: 实现连接逻辑
  console.log('连接到:', connection)
  navigateTo('/database')
}
</script>

<style scoped>
.home {
  height: 100vh;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}

.header {
  text-align: center;
  color: white;
  padding: 40px 0;
}

.header h1 {
  margin: 0;
  font-size: 2.5rem;
  font-weight: 300;
}

.header p {
  margin: 10px 0 0 0;
  font-size: 1.1rem;
  opacity: 0.9;
}

.main {
  padding: 20px;
  max-width: 1200px;
  margin: 0 auto;
}

.feature-card {
  cursor: pointer;
  transition: all 0.3s ease;
  height: 200px;
}

.feature-card:hover {
  transform: translateY(-5px);
  box-shadow: 0 8px 25px rgba(0, 0, 0, 0.15);
}

.card-content {
  text-align: center;
  padding: 20px;
}

.card-content h3 {
  margin: 15px 0 10px 0;
  color: #303133;
}

.card-content p {
  color: #606266;
  line-height: 1.5;
}

.empty-state {
  padding: 40px 0;
}

.connection-info {
  color: #909399;
  font-size: 0.9rem;
}

.el-list-item {
  cursor: pointer;
  padding: 10px 0;
}

.el-list-item:hover {
  background-color: #f5f7fa;
}
</style> 