<template>
  <el-dialog 
    v-model="visible" 
    title="键盘快捷键" 
    width="700px"
    :close-on-click-modal="true"
    class="shortcut-help-dialog"
  >
    <div class="shortcut-content">
      <el-tabs v-model="activeTab" class="shortcut-tabs">
        <el-tab-pane label="通用操作" name="general">
          <div class="shortcut-section">
            <h4>导航</h4>
            <div class="shortcut-list">
              <div class="shortcut-item">
                <span class="shortcut-key">Ctrl + R</span>
                <span class="shortcut-description">刷新数据</span>
              </div>
              <div class="shortcut-item">
                <span class="shortcut-key">Ctrl + N</span>
                <span class="shortcut-description">新建数据库</span>
              </div>
              <div class="shortcut-item">
                <span class="shortcut-key">Delete / Backspace</span>
                <span class="shortcut-description">删除选中项</span>
              </div>
              <div class="shortcut-item">
                <span class="shortcut-key">F5</span>
                <span class="shortcut-description">刷新当前视图</span>
              </div>
            </div>
          </div>

          <div class="shortcut-section">
            <h4>编辑器</h4>
            <div class="shortcut-list">
              <div class="shortcut-item">
                <span class="shortcut-key">Ctrl + Enter</span>
                <span class="shortcut-description">执行查询</span>
              </div>
              <div class="shortcut-item">
                <span class="shortcut-key">Ctrl + S</span>
                <span class="shortcut-description">保存查询</span>
              </div>
              <div class="shortcut-item">
                <span class="shortcut-key">Ctrl + Z</span>
                <span class="shortcut-description">撤销</span>
              </div>
              <div class="shortcut-item">
                <span class="shortcut-key">Ctrl + Y</span>
                <span class="shortcut-description">重做</span>
              </div>
            </div>
          </div>

          <div class="shortcut-section">
            <h4>界面</h4>
            <div class="shortcut-list">
              <div class="shortcut-item">
                <span class="shortcut-key">Ctrl + B</span>
                <span class="shortcut-description">切换侧边栏</span>
              </div>
              <div class="shortcut-item">
                <span class="shortcut-key">Ctrl + J</span>
                <span class="shortcut-description">切换底部面板</span>
              </div>
              <div class="shortcut-item">
                <span class="shortcut-key">F1</span>
                <span class="shortcut-description">显示此帮助</span>
              </div>
              <div class="shortcut-item">
                <span class="shortcut-key">Ctrl + ,</span>
                <span class="shortcut-description">打开设置</span>
              </div>
            </div>
          </div>
        </el-tab-pane>

        <el-tab-pane label="数据库操作" name="database">
          <div class="shortcut-section">
            <h4>数据库管理</h4>
            <div class="shortcut-list">
              <div class="shortcut-item">
                <span class="shortcut-key">Ctrl + Shift + N</span>
                <span class="shortcut-description">新建测量值</span>
              </div>
              <div class="shortcut-item">
                <span class="shortcut-key">Ctrl + D</span>
                <span class="shortcut-description">删除数据库/测量值</span>
              </div>
              <div class="shortcut-item">
                <span class="shortcut-key">Ctrl + Q</span>
                <span class="shortcut-description">查询选中项</span>
              </div>
              <div class="shortcut-item">
                <span class="shortcut-key">Enter</span>
                <span class="shortcut-description">展开/折叠节点</span>
              </div>
            </div>
          </div>

          <div class="shortcut-section">
            <h4>导航</h4>
            <div class="shortcut-list">
              <div class="shortcut-item">
                <span class="shortcut-key">↑ / ↓</span>
                <span class="shortcut-description">上下选择</span>
              </div>
              <div class="shortcut-item">
                <span class="shortcut-key">← / →</span>
                <span class="shortcut-description">展开/折叠</span>
              </div>
              <div class="shortcut-item">
                <span class="shortcut-key">Home</span>
                <span class="shortcut-description">选择第一个</span>
              </div>
              <div class="shortcut-item">
                <span class="shortcut-key">End</span>
                <span class="shortcut-description">选择最后一个</span>
              </div>
            </div>
          </div>
        </el-tab-pane>

        <el-tab-pane label="查询编辑器" name="query">
          <div class="shortcut-section">
            <h4>查询执行</h4>
            <div class="shortcut-list">
              <div class="shortcut-item">
                <span class="shortcut-key">Ctrl + Enter</span>
                <span class="shortcut-description">执行查询</span>
              </div>
              <div class="shortcut-item">
                <span class="shortcut-key">Ctrl + Shift + Enter</span>
                <span class="shortcut-description">执行选中部分</span>
              </div>
              <div class="shortcut-item">
                <span class="shortcut-key">Ctrl + R</span>
                <span class="shortcut-description">重新执行</span>
              </div>
              <div class="shortcut-item">
                <span class="shortcut-key">Ctrl + L</span>
                <span class="shortcut-description">清空编辑器</span>
              </div>
            </div>
          </div>

          <div class="shortcut-section">
            <h4>编辑器操作</h4>
            <div class="shortcut-list">
              <div class="shortcut-item">
                <span class="shortcut-key">Ctrl + /</span>
                <span class="shortcut-description">注释/取消注释</span>
              </div>
              <div class="shortcut-item">
                <span class="shortcut-key">Ctrl + D</span>
                <span class="shortcut-description">选择下一个相同项</span>
              </div>
              <div class="shortcut-item">
                <span class="shortcut-key">Ctrl + F</span>
                <span class="shortcut-description">查找</span>
              </div>
              <div class="shortcut-item">
                <span class="shortcut-key">Ctrl + H</span>
                <span class="shortcut-description">替换</span>
              </div>
            </div>
          </div>
        </el-tab-pane>
      </el-tabs>
    </div>

    <template #footer>
      <div class="dialog-footer">
        <el-button @click="handleClose">关闭</el-button>
        <el-button type="primary" @click="printShortcuts">打印快捷键</el-button>
      </div>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { ElMessage } from 'element-plus'

interface Props {
  modelValue: boolean
}

interface Emits {
  (e: 'update:modelValue', value: boolean): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const visible = ref(props.modelValue)
const activeTab = ref('general')

// 监听modelValue变化
watch(() => props.modelValue, (newVal) => {
  visible.value = newVal
})

// 监听visible变化
watch(visible, (newVal) => {
  emit('update:modelValue', newVal)
})

const handleClose = () => {
  visible.value = false
}

const printShortcuts = () => {
  // 创建打印内容
  const printContent = `
    <html>
      <head>
        <title>InfluxDB Studio 快捷键</title>
        <style>
          body { font-family: Arial, sans-serif; margin: 20px; }
          .shortcut-section { margin-bottom: 20px; }
          .shortcut-item { display: flex; margin: 5px 0; }
          .shortcut-key { font-weight: bold; width: 200px; }
          .shortcut-description { flex: 1; }
        </style>
      </head>
      <body>
        <h1>InfluxDB Studio 快捷键</h1>
        <div class="shortcut-section">
          <h3>通用操作</h3>
          <div class="shortcut-item">
            <span class="shortcut-key">Ctrl + R</span>
            <span class="shortcut-description">刷新数据</span>
          </div>
          <div class="shortcut-item">
            <span class="shortcut-key">Ctrl + N</span>
            <span class="shortcut-description">新建数据库</span>
          </div>
          <div class="shortcut-item">
            <span class="shortcut-key">Delete / Backspace</span>
            <span class="shortcut-description">删除选中项</span>
          </div>
        </div>
        <div class="shortcut-section">
          <h3>查询编辑器</h3>
          <div class="shortcut-item">
            <span class="shortcut-key">Ctrl + Enter</span>
            <span class="shortcut-description">执行查询</span>
          </div>
          <div class="shortcut-item">
            <span class="shortcut-key">Ctrl + S</span>
            <span class="shortcut-description">保存查询</span>
          </div>
        </div>
      </body>
    </html>
  `
  
  const printWindow = window.open('', '_blank')
  if (printWindow) {
    printWindow.document.write(printContent)
    printWindow.document.close()
    printWindow.print()
  } else {
    ElMessage.warning('无法打开打印窗口，请检查浏览器设置')
  }
}
</script>

<style scoped>
.shortcut-help-dialog {
  --shortcut-bg: var(--geek-bg-secondary);
  --shortcut-border: var(--geek-border);
  --shortcut-text: var(--geek-text-primary);
  --shortcut-key-bg: var(--geek-bg-tertiary);
}

.shortcut-content {
  max-height: 500px;
  overflow-y: auto;
}

.shortcut-tabs {
  height: 100%;
}

.shortcut-section {
  margin-bottom: 24px;
}

.shortcut-section h4 {
  margin: 0 0 12px 0;
  color: var(--shortcut-text);
  font-size: 14px;
  font-weight: 600;
  border-bottom: 1px solid var(--shortcut-border);
  padding-bottom: 6px;
}

.shortcut-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.shortcut-item {
  display: flex;
  align-items: center;
  padding: 8px 12px;
  background: var(--shortcut-bg);
  border: 1px solid var(--shortcut-border);
  border-radius: 4px;
  transition: all 0.2s ease;
}

.shortcut-item:hover {
  background: var(--geek-bg-highlight);
}

.shortcut-key {
  background: var(--shortcut-key-bg);
  border: 1px solid var(--shortcut-border);
  border-radius: 3px;
  padding: 4px 8px;
  font-family: var(--geek-font-mono);
  font-size: 12px;
  color: var(--geek-accent-orange);
  min-width: 120px;
  text-align: center;
  margin-right: 12px;
}

.shortcut-description {
  color: var(--shortcut-text);
  font-size: 13px;
  flex: 1;
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
}

/* 自定义滚动条 */
.shortcut-content::-webkit-scrollbar {
  width: 6px;
}

.shortcut-content::-webkit-scrollbar-track {
  background: var(--geek-bg-primary);
}

.shortcut-content::-webkit-scrollbar-thumb {
  background: var(--geek-border);
  border-radius: 3px;
}

.shortcut-content::-webkit-scrollbar-thumb:hover {
  background: var(--geek-text-secondary);
}
</style> 