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
      <el-menu-item index="connections">
        <el-icon><Connection /></el-icon>
        <span>连接管理</span>
      </el-menu-item>
      <el-menu-item index="query">
        <el-icon><Search /></el-icon>
        <span>查询编辑器</span>
      </el-menu-item>
    </el-menu>
  </el-aside>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { Connection, Search } from '@element-plus/icons-vue'

const router = useRouter()
const activeMenu = ref('connections')

const handleMenuSelect = (key: string) => {
  activeMenu.value = key
  // 根据菜单项导航到不同路由
  switch (key) {
    case 'connections':
      router.push('/connections')
      break
    case 'query':
      router.push('/query')
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