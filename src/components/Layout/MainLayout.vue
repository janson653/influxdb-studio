<template>
  <el-container style="min-height: 100vh">
    <Sidebar />
    <el-container>
      <!-- 添加顶部导航栏 -->
      <el-header style="height: 50px; background-color: #fff; border-bottom: 1px solid #e4e7ed; padding: 0 20px; display: flex; align-items: center; justify-content: space-between;">
        <div class="nav-left">
          <el-button @click="goHome" type="text" size="small">
            <el-icon><House /></el-icon>
            首页
          </el-button>
          <el-divider direction="vertical" />
          <span class="current-page">{{ currentPageTitle }}</span>
        </div>
        <div class="nav-right">
          <el-button @click="goBack" type="text" size="small">
            <el-icon><ArrowLeft /></el-icon>
            返回
          </el-button>
        </div>
      </el-header>
      
      <el-main style="margin: 16px">
        <!-- 主内容区域将在这里渲染 -->
        <router-view />
      </el-main>
    </el-container>
  </el-container>
</template>

<script setup lang="ts">
import { computed, onMounted, onUnmounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { House, ArrowLeft } from '@element-plus/icons-vue'
import Sidebar from './Sidebar.vue'

const router = useRouter()
const route = useRoute()

// 当前页面标题
const currentPageTitle = computed(() => {
  return route.meta.title || '首页'
})

// 回到首页
const goHome = () => {
  router.push('/home')
}

// 返回上一页
const goBack = () => {
  router.back()
}

// 键盘快捷键处理
const handleKeydown = (event: KeyboardEvent) => {
  // Ctrl+H 回到首页
  if (event.ctrlKey && event.key === 'h') {
    event.preventDefault()
    goHome()
  }
  
  // Ctrl+← 返回上一页
  if (event.ctrlKey && event.key === 'ArrowLeft') {
    event.preventDefault()
    goBack()
  }
  
  // Alt+← 返回上一页 (浏览器默认行为)
  if (event.altKey && event.key === 'ArrowLeft') {
    event.preventDefault()
    goBack()
  }
}

// 生命周期钩子
onMounted(() => {
  document.addEventListener('keydown', handleKeydown)
})

onUnmounted(() => {
  document.removeEventListener('keydown', handleKeydown)
})
</script>

<style scoped>
.el-main {
  background-color: #f5f5f5;
}

.nav-left {
  display: flex;
  align-items: center;
}

.current-page {
  font-weight: 500;
  color: #303133;
}

.nav-right {
  display: flex;
  align-items: center;
}
</style> 