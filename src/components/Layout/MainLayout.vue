<template>
  <el-container style="min-height: 100vh">
    <Sidebar />
    <el-container>
      <el-main style="margin: 16px">
        <!-- 主内容区域将在这里渲染 -->
        <router-view />
      </el-main>
    </el-container>
  </el-container>
</template>

<script setup lang="ts">
import Sidebar from './Sidebar.vue'
</script>

<style scoped>
.el-main {
  background-color: #f5f5f5;
}
</style> 