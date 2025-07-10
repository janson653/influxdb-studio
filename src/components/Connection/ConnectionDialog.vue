<template>
  <el-dialog
    v-model="visible"
    :title="isEdit ? '编辑连接' : '新建连接'"
    width="500px"
    @close="handleClose"
  >
    <el-form
      ref="formRef"
      :model="form"
      :rules="rules"
      label-width="100px"
      @submit.prevent="handleSubmit"
    >
      <el-form-item label="连接名称" prop="name">
        <el-input 
          v-model="form.name" 
          placeholder="请输入连接名称"
          maxlength="50"
          show-word-limit
        />
      </el-form-item>
      
      <el-form-item label="主机地址" prop="host">
        <el-input 
          v-model="form.host" 
          placeholder="例如: localhost 或 192.168.1.100"
        />
      </el-form-item>
      
      <el-form-item label="端口" prop="port">
        <el-input-number 
          v-model="form.port" 
          :min="1" 
          :max="65535"
          placeholder="8086"
          style="width: 100%"
        />
      </el-form-item>
      
      <el-form-item label="默认数据库" prop="database">
        <el-input 
          v-model="form.database" 
          placeholder="可选，连接后选择数据库"
        />
      </el-form-item>
      
      <el-form-item label="用户名" prop="username">
        <el-input 
          v-model="form.username" 
          placeholder="可选，留空表示匿名访问"
        />
      </el-form-item>
      
      <el-form-item label="密码" prop="password">
        <el-input 
          v-model="form.password" 
          type="password"
          placeholder="可选，留空表示匿名访问"
          show-password
        />
      </el-form-item>
      
      <el-form-item label="SSL/TLS" prop="useSsl">
        <el-switch 
          v-model="form.useSsl"
          active-text="启用"
          inactive-text="禁用"
        />
      </el-form-item>
      
      <el-form-item label="超时时间" prop="timeout">
        <el-input-number 
          v-model="form.timeout" 
          :min="1000" 
          :max="60000"
          :step="1000"
          placeholder="5000"
          style="width: 100%"
        />
        <span style="margin-left: 8px; color: #909399;">毫秒</span>
      </el-form-item>
    </el-form>
    
    <template #footer>
      <span class="dialog-footer">
        <el-button @click="handleClose">取消</el-button>
        <el-button 
          type="primary" 
          @click="handleSubmit"
          :loading="isSubmitting"
        >
          {{ isEdit ? '保存' : '创建' }}
        </el-button>
      </span>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { ref, reactive, computed, watch } from 'vue'
import { ElMessage } from 'element-plus'
import type { FormInstance, FormRules } from 'element-plus'
import type { ConnectionConfig } from '../../stores/connectionStore'

// Props
interface Props {
  modelValue: boolean
  connection?: ConnectionConfig | null
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: false,
  connection: null
})

// Emits
const emit = defineEmits<{
  'update:modelValue': [value: boolean]
  'save': [connection: ConnectionConfig]
}>()

// 响应式数据
const formRef = ref<FormInstance>()
const isSubmitting = ref(false)

// 表单数据
const form = reactive({
  id: '',
  name: '',
  host: 'localhost',
  port: 8086,
  database: '',
  username: '',
  password: '',
  useSsl: false,
  timeout: 5000
})

// 表单验证规则
const rules: FormRules = {
  name: [
    { required: true, message: '请输入连接名称', trigger: 'blur' },
    { min: 1, max: 50, message: '连接名称长度在 1 到 50 个字符', trigger: 'blur' }
  ],
  host: [
    { required: true, message: '请输入主机地址', trigger: 'blur' },
    { 
      pattern: /^[a-zA-Z0-9.-]+$/, 
      message: '主机地址格式不正确', 
      trigger: 'blur' 
    }
  ],
  port: [
    { required: true, message: '请输入端口号', trigger: 'blur' },
    { type: 'number', min: 1, max: 65535, message: '端口号必须在 1-65535 之间', trigger: 'blur' }
  ],
  timeout: [
    { required: true, message: '请输入超时时间', trigger: 'blur' },
    { type: 'number', min: 1000, max: 60000, message: '超时时间必须在 1000-60000 毫秒之间', trigger: 'blur' }
  ]
}

// 计算属性
const visible = computed({
  get: () => props.modelValue,
  set: (value) => emit('update:modelValue', value)
})

const isEdit = computed(() => !!props.connection)

// 监听连接数据变化
watch(() => props.connection, (newConnection) => {
  if (newConnection) {
    // 编辑模式，填充表单数据
    Object.assign(form, newConnection)
  } else {
    // 新建模式，重置表单数据
    resetForm()
  }
}, { immediate: true })

// 方法
const resetForm = () => {
  Object.assign(form, {
    id: '',
    name: '',
    host: 'localhost',
    port: 8086,
    database: '',
    username: '',
    password: '',
    useSsl: false,
    timeout: 5000
  })
  
  // 清除表单验证
  formRef.value?.clearValidate()
}

const handleClose = () => {
  visible.value = false
  resetForm()
}

const handleSubmit = async () => {
  if (!formRef.value) return
  
  try {
    await formRef.value.validate()
    
    isSubmitting.value = true
    
    // 生成连接ID（如果是新建）
    if (!form.id) {
      form.id = `conn_${Date.now()}`
    }
    
    // 创建连接配置对象
    const connectionConfig: ConnectionConfig = {
      id: form.id,
      name: form.name,
      host: form.host,
      port: form.port,
      database: form.database || undefined,
      username: form.username || undefined,
      password: form.password || undefined,
      useSsl: form.useSsl,
      timeout: form.timeout
    }
    
    // 触发保存事件
    emit('save', connectionConfig)
    
  } catch (error) {
    console.error('表单验证失败:', error)
  } finally {
    isSubmitting.value = false
  }
}

// 暴露方法给父组件
defineExpose({
  resetForm
})
</script>

<style scoped>
.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
}

.el-form-item {
  margin-bottom: 20px;
}
</style> 