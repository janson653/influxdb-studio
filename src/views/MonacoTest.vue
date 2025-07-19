<template>
  <div class="monaco-test">
    <h1>Monaco Editor 测试页面</h1>
    
    <div class="test-controls">
      <el-button @click="testLoad" :loading="testing">测试加载</el-button>
      <el-button @click="clearEditor">清空编辑器</el-button>
      <el-button @click="setSampleCode">设置示例代码</el-button>
    </div>

    <div class="editor-wrapper">
      <MonacoEditor
        v-model="code"
        :language="language"
        :options="editorOptions"
        @change="handleCodeChange"
      />
    </div>

    <div class="status">
      <p>当前语言: {{ language }}</p>
      <p>代码长度: {{ code.length }}</p>
      <p>加载状态: {{ loading ? '加载中' : '已加载' }}</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import MonacoEditor from '../components/Common/MonacoEditor.vue'
import { ElMessage } from 'element-plus'

const code = ref('SELECT * FROM measurement LIMIT 10')
const language = ref('sql')
const testing = ref(false)
const loading = ref(false)

const editorOptions = {
  theme: 'vs-dark',
  fontSize: 14,
  minimap: { enabled: true },
  scrollBeyondLastLine: false,
  automaticLayout: true,
  wordWrap: 'on' as const
}

const testLoad = async () => {
  testing.value = true
  try {
    // 模拟测试加载
    await new Promise(resolve => setTimeout(resolve, 1000))
    ElMessage.success('Monaco Editor 加载测试成功')
  } catch (error) {
    ElMessage.error('加载测试失败')
  } finally {
    testing.value = false
  }
}

const clearEditor = () => {
  code.value = ''
  ElMessage.info('编辑器已清空')
}

const setSampleCode = () => {
  code.value = `-- SQL 示例查询
SELECT 
  time,
  value,
  tag1,
  tag2
FROM measurement
WHERE time > now() - 1h
  AND tag1 = 'example'
ORDER BY time DESC
LIMIT 100`
  
  ElMessage.success('示例代码已设置')
}

const handleCodeChange = (value: string) => {
  console.log('代码变化:', value)
}
</script>

<style scoped>
.monaco-test {
  padding: 20px;
  max-width: 1200px;
  margin: 0 auto;
}

.test-controls {
  margin-bottom: 20px;
  display: flex;
  gap: 10px;
}

.editor-wrapper {
  height: 400px;
  border: 1px solid #dcdfe6;
  border-radius: 4px;
  margin-bottom: 20px;
}

.status {
  background-color: #f5f7fa;
  padding: 15px;
  border-radius: 4px;
}

.status p {
  margin: 5px 0;
  color: #606266;
}
</style> 