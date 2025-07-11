<template>
  <el-aside width="200px" style="background-color: #001529">
    <div class="logo-placeholder">
      <!-- Logo Placeholder -->
    </div>
    <el-menu
      mode="vertical"
      background-color="#001529"
      text-color="#fff"
      active-text-color="#1890ff"
      :default-active="activeMenu"
      @select="handleMenuSelect"
    >
      <el-menu-item index="home">
        <el-icon><House /></el-icon>
        <span>首页</span>
      </el-menu-item>
      <el-menu-item index="connections">
        <el-icon><Connection /></el-icon>
        <span>连接管理</span>
      </el-menu-item>
      <el-menu-item index="database">
        <el-icon><DataAnalysis /></el-icon>
        <span>数据库浏览器</span>
      </el-menu-item>
      <el-menu-item index="query">
        <el-icon><Search /></el-icon>
        <span>查询编辑器</span>
      </el-menu-item>
      <el-menu-item index="settings">
        <el-icon><Setting /></el-icon>
        <span>设置</span>
      </el-menu-item>
    </el-menu>
  </el-aside>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { Connection, Search, House, DataAnalysis, Setting } from '@element-plus/icons-vue'

const router = useRouter()
const route = useRoute()
const activeMenu = ref('home')

// 监听路由变化，更新激活菜单项
watch(() => route.path, (newPath) => {
  console.log('当前路由路径:', newPath)
  switch (newPath) {
    case '/home':
      activeMenu.value = 'home'
      break
    case '/connection':
      activeMenu.value = 'connections'
      break
    case '/database':
      activeMenu.value = 'database'
      break
    case '/query':
      activeMenu.value = 'query'
      break
    case '/settings':
      activeMenu.value = 'settings'
      break
  }
}, { immediate: true })

const handleMenuSelect = (key: string) => {
  console.log('菜单选择:', key)
  activeMenu.value = key
  // 根据菜单项导航到不同路由
  switch (key) {
    case 'home':
      console.log('导航到首页')
      router.push('/home')
      break
    case 'connections':
      console.log('导航到连接管理')
      router.push('/connection')
      break
    case 'database':
      console.log('导航到数据库浏览器')
      router.push('/database')
      break
    case 'query':
      console.log('导航到查询编辑器')
      router.push('/query')
      break
    case 'settings':
      console.log('导航到设置')
      router.push('/settings')
      break
  }
}
</script>

<style scoped>
.logo-placeholder {
  height: 32px;
  margin: 16px;
  background: rgba(255, 255, 255, 0.2);
  border-radius: 4px;
}

.el-menu {
  border-right: none;
}
</style> 