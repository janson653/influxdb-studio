<template>
  <div ref="editorContainer" class="monaco-editor-container">
    <div v-if="loading" class="editor-loading">
      <el-icon class="is-loading"><Loading /></el-icon>
      <span>正在加载编辑器...</span>
    </div>
    <div v-if="error" class="editor-error">
      <el-icon><Warning /></el-icon>
      <span>编辑器加载失败</span>
      <el-button size="small" @click="retryLoad">重试</el-button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, watch } from 'vue'
import { Loading, Warning } from '@element-plus/icons-vue'
import { ElMessage } from 'element-plus'
import { loadMonaco, resetMonaco } from '../../utils/monacoLoader'

// Props
interface Props {
  modelValue: string
  language?: string
  options?: any
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: '',
  language: 'sql',
  options: () => ({})
})

// Emits
const emit = defineEmits<{
  'update:modelValue': [value: string]
  'change': [value: string]
}>()

// 响应式数据
const editorContainer = ref<HTMLDivElement>()
const loading = ref(true)
const error = ref(false)
let editor: any = null
let monaco: any = null

// 默认编辑器选项
const defaultOptions = {
  theme: 'vs-dark',
  fontSize: 14,
  minimap: { enabled: false },
  scrollBeyondLastLine: false,
  automaticLayout: true,
  wordWrap: 'on' as const,
  lineNumbers: 'on' as const,
  roundedSelection: false,
  scrollbar: {
    vertical: 'visible',
    horizontal: 'visible'
  }
}

// 加载 Monaco Editor
const loadMonacoEditor = async () => {
  try {
    loading.value = true
    error.value = false
    
    // 使用加载器工具加载 Monaco Editor
    monaco = await loadMonaco({ timeout: 15000 })
    
    if (monaco && monaco.editor) {
      initEditor()
    } else {
      throw new Error('Monaco Editor 加载失败')
    }
  } catch (err) {
    console.error('Monaco Editor 加载错误:', err)
    error.value = true
    ElMessage.error('编辑器加载失败，请刷新页面重试')
  } finally {
    loading.value = false
  }
}

// 初始化编辑器
const initEditor = () => {
  if (!editorContainer.value || !monaco) return

  // 合并选项
  const options = {
    ...defaultOptions,
    ...props.options,
    value: props.modelValue,
    language: props.language
  }

  try {
    // 创建编辑器实例
    editor = monaco.editor.create(editorContainer.value, options)

    // 监听内容变化
    editor.onDidChangeModelContent(() => {
      const value = editor?.getValue() || ''
      emit('update:modelValue', value)
      emit('change', value)
    })

    // 监听编辑器就绪
    editor.onDidLayoutChange(() => {
      // 编辑器布局完成后的处理
    })

    // 监听编辑器焦点
    editor.onDidFocusEditorWidget(() => {
      // 编辑器获得焦点时的处理
    })
  } catch (err) {
    console.error('编辑器初始化错误:', err)
    error.value = true
    ElMessage.error('编辑器初始化失败')
  }
}

// 销毁编辑器
const destroyEditor = () => {
  if (editor) {
    try {
      editor.dispose()
    } catch (err) {
      console.error('编辑器销毁错误:', err)
    }
    editor = null
  }
}

// 重试加载
const retryLoad = () => {
  destroyEditor()
  resetMonaco() // 重置加载器状态
  loadMonacoEditor()
}

// 监听 modelValue 变化
watch(() => props.modelValue, (newValue) => {
  if (editor && editor.getValue() !== newValue) {
    editor.setValue(newValue)
  }
})

// 监听语言变化
watch(() => props.language, (newLanguage) => {
  if (editor && monaco) {
    try {
      monaco.editor.setModelLanguage(editor.getModel(), newLanguage)
    } catch (err) {
      console.error('语言切换错误:', err)
    }
  }
})

// 生命周期
onMounted(() => {
  loadMonacoEditor()
})

onBeforeUnmount(() => {
  destroyEditor()
})

// 暴露方法给父组件
defineExpose({
  getValue: () => editor?.getValue() || '',
  setValue: (value: string) => editor?.setValue(value),
  focus: () => editor?.focus(),
  getEditor: () => editor,
  retryLoad
})
</script>

<style scoped>
.monaco-editor-container {
  width: 100%;
  height: 100%;
  min-height: 200px;
  position: relative;
}

.editor-loading,
.editor-error {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 10px;
  color: #909399;
}

.editor-error {
  color: #f56c6c;
}

.editor-loading .el-icon,
.editor-error .el-icon {
  font-size: 24px;
}
</style> 