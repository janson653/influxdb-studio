<template>
  <div id="app">
    <router-view />
  </div>
</template>

<script setup lang="ts">
// 主应用组件
// 使用 Vue 3 Composition API
</script>

<style>
/* 应用根样式 - 使用ide-theme.css中的变量 */
#app {
  height: 100vh;
  margin: 0;
  padding: 0;
}

/* 确保Element Plus组件使用我们的主题变量 */
:deep(.el-container), :deep(.el-main), :deep(.el-header), :deep(.el-aside), :deep(.el-footer) {
  background-color: var(--ide-bg-primary) !important;
}

:deep(.el-card) {
  background-color: var(--ide-bg-secondary) !important;
  border-color: var(--ide-border) !important;
  color: var(--ide-text-primary) !important;
}

:deep(.el-card__header) {
  background-color: var(--ide-bg-tertiary) !important;
  border-bottom-color: var(--ide-border) !important;
  color: var(--ide-text-primary) !important;
}

:deep(.el-button) {
  background-color: var(--ide-bg-tertiary) !important;
  border-color: var(--ide-border) !important;
  color: var(--ide-text-primary) !important;
}

:deep(.el-button:hover) {
  background-color: var(--ide-bg-hover) !important;
  border-color: var(--ide-border-light) !important;
}

:deep(.el-button.el-button--primary) {
  background-color: var(--ide-accent-primary) !important;
  border-color: var(--ide-accent-secondary) !important;
  color: white !important;
}

:deep(.el-button.el-button--primary:hover) {
  background-color: var(--ide-accent-secondary) !important;
  border-color: var(--ide-accent-primary) !important;
}

:deep(.el-input__wrapper) {
  background-color: var(--ide-bg-secondary) !important;
  border-color: var(--ide-border) !important;
}

:deep(.el-input__inner) {
  color: var(--ide-text-primary) !important;
}

:deep(.el-tree) {
  background-color: transparent !important;
  color: var(--ide-text-primary) !important;
}

:deep(.el-tree-node__content:hover) {
  background-color: var(--ide-bg-tertiary) !important;
}

:deep(.el-tree-node:focus > .el-tree-node__content) {
  background-color: var(--ide-bg-tertiary) !important;
}

:deep(.el-table), :deep(.el-table__expanded-cell) {
  background-color: var(--ide-bg-primary) !important;
}

:deep(.el-table th), :deep(.el-table tr) {
  background-color: var(--ide-bg-secondary) !important;
  color: var(--ide-text-primary) !important;
}

:deep(.el-table th.el-table__cell), :deep(.el-table td.el-table__cell) {
  border-bottom-color: var(--ide-border) !important;
}

:deep(.el-table--border .el-table__inner-wrapper::after), 
:deep(.el-table--border::after), 
:deep(.el-table--border::before), 
:deep(.el-table__inner-wrapper::before) {
  border-color: var(--ide-border) !important;
}

:deep(.el-table thead) {
  background-color: var(--ide-bg-secondary) !important;
}

:deep(.el-table tr:hover > td) {
  background-color: var(--ide-bg-tertiary) !important;
}

:deep(.el-dialog) {
  background-color: var(--ide-bg-secondary) !important;
  border-color: var(--ide-border) !important;
}

:deep(.el-dialog__title) {
  color: var(--ide-text-primary) !important;
}

:deep(.el-dialog__body) {
  color: var(--ide-text-primary) !important;
  background-color: var(--ide-bg-primary) !important;
}
</style> 