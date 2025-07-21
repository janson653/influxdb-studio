<template>
  <el-dialog
    v-model="visible"
    :title="isEdit ? '编辑连接' : '新建连接'"
    width="600px"
    @close="handleClose"
  >
    <el-form
      ref="formRef"
      :model="form"
      :rules="rules"
      label-width="120px"
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
      
      <el-form-item label="InfluxDB 版本" prop="version">
        <el-select 
          v-model="form.version" 
          placeholder="请选择 InfluxDB 版本"
          style="width: 100%"
          @change="handleVersionChange"
        >
          <el-option label="InfluxDB v1.x" :value="InfluxDBVersion.V1" />
          <el-option label="InfluxDB v2.x (开发中)" :value="InfluxDBVersion.V2" disabled />
          <el-option label="InfluxDB v3.x (开发中)" :value="InfluxDBVersion.V3" disabled />
        </el-select>
      </el-form-item>
      
      <!-- 通用配置 -->
      <el-divider content-position="left">连接配置</el-divider>
      
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
      
      <!-- InfluxDB v1.x 配置 -->
      <template v-if="form.version === InfluxDBVersion.V1">
        <el-divider content-position="left">InfluxDB v1.x 配置</el-divider>
        
        <el-form-item label="数据库" prop="database">
          <el-input 
            v-model="form.database" 
            placeholder="请输入数据库名称"
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
      </template>
      
      <!-- InfluxDB v2.x 配置 -->
      <template v-if="form.version === InfluxDBVersion.V2">
        <el-divider content-position="left">InfluxDB v2.x 配置</el-divider>
        
        <el-form-item label="Token" prop="token">
          <el-input 
            v-model="form.token" 
            type="password"
            placeholder="请输入 InfluxDB 2.x Token"
            show-password
          />
        </el-form-item>
        
        <el-form-item label="组织(Org)" prop="org">
          <el-input 
            v-model="form.org" 
            placeholder="请输入组织名称"
          />
        </el-form-item>
        
        <el-form-item label="存储桶(Bucket)" prop="bucket">
          <el-input 
            v-model="form.bucket" 
            placeholder="可选，连接后选择存储桶"
          />
        </el-form-item>
      </template>
      
      <!-- InfluxDB v3.x 配置 -->
      <template v-if="form.version === InfluxDBVersion.V3">
        <el-divider content-position="left">InfluxDB v3.x 配置</el-divider>
        
        <el-form-item label="Token" prop="token">
          <el-input 
            v-model="form.token" 
            type="password"
            placeholder="请输入 InfluxDB 3.x Token"
            show-password
          />
        </el-form-item>
        
        <el-form-item label="数据库" prop="database">
          <el-input 
            v-model="form.database" 
            placeholder="请输入数据库名称"
          />
        </el-form-item>
      </template>
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
        <el-button @click="testConnection" :loading="isTesting">
          测试连接
        </el-button>
      </span>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { ref, reactive, computed, watch, nextTick } from 'vue'
import type { FormInstance, FormRules } from 'element-plus'
import type { ConnectionProfile } from '../../types/influxdb'
import { InfluxDBVersion } from '../../types/influxdb'
import { ElMessage } from 'element-plus' // 导入 ElMessage

// Props
interface Props {
  modelValue: boolean
  connection?: ConnectionProfile | null
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: false,
  connection: null
})

// Emits
const emit = defineEmits<{
  'update:modelValue': [value: boolean]
  'save': [connection: ConnectionProfile]
}>()

// 响应式数据
const formRef = ref<FormInstance>()
const isSubmitting = ref(false)
const isTesting = ref(false) // 新增：测试连接状态

// 表单数据
const form = reactive({
  id: '',
  name: '',
  version: InfluxDBVersion.V1,
  host: 'localhost',
  port: 8086,
  useSsl: false,
  timeout: 5000,
  // v1.x 字段
  database: '',
  username: '',
  password: '',
  // v2.x 字段
  token: '',
  org: '',
  bucket: '',
  // v3.x 字段 (database 复用 v1.x 的字段)
})

// 动态表单验证规则
const getRules = (): FormRules => {
  const baseRules: FormRules = {
    name: [
      { required: true, message: '请输入连接名称', trigger: 'blur' },
      { min: 1, max: 50, message: '连接名称长度在 1 到 50 个字符', trigger: 'blur' }
    ],
    version: [
      { required: true, message: '请选择 InfluxDB 版本', trigger: 'change' }
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

  // 目前只支持 v1.x 版本
  if (form.version === InfluxDBVersion.V1) {
    baseRules.database = [
      { required: true, message: '请输入数据库名称', trigger: 'blur' }
    ]
  }

  return baseRules
}

const rules = computed(() => getRules())

// 计算属性
const visible = computed({
  get: () => props.modelValue,
  set: (value) => emit('update:modelValue', value)
})

const isEdit = computed(() => !!props.connection)

// 方法
const resetFormData = () => {
  Object.assign(form, {
    id: '',
    name: '',
    version: InfluxDBVersion.V1,
    host: 'localhost',
    port: 8086,
    useSsl: false,
    timeout: 5000,
    database: '',
    username: '',
    password: '',
    token: '',
    org: '',
    bucket: ''
  })
}

const handleVersionChange = () => {
  // 如果选择了开发中的版本，强制切换回 v1.x
  if (form.version === InfluxDBVersion.V2 || form.version === InfluxDBVersion.V3) {
    ElMessage.warning('InfluxDB v2.x 和 v3.x 支持正在开发中，请使用 v1.x 版本')
    form.version = InfluxDBVersion.V1
    return
  }
  
  // 版本切换时清空相关字段
  if (form.version === InfluxDBVersion.V1) {
    form.token = ''
    form.org = ''
    form.bucket = ''
  }
  
  // 重新验证表单
  nextTick(() => {
    formRef.value?.clearValidate()
  })
}

// 监听连接数据变化
watch(() => props.connection, (newConnection) => {
  if (newConnection) {
    // 编辑模式，填充表单数据
    Object.assign(form, {
      id: newConnection.id,
      name: newConnection.name,
      version: newConnection.version,
      host: newConnection.config.host,
      port: newConnection.config.port,
      useSsl: newConnection.config.useSsl,
      timeout: newConnection.config.timeout,
      // 目前只支持 v1.x 版本
      database: (newConnection.config as any).database,
      username: (newConnection.config as any).username || '',
      password: (newConnection.config as any).password || ''
    })
  } else {
    // 新建模式，重置表单数据
    resetFormData()
  }
}, { immediate: true })

const resetForm = () => {
  resetFormData()
  
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
    
    // 目前只支持 v1.x 版本
    let config: any = {
      host: form.host,
      port: form.port,
      useSsl: form.useSsl,
      timeout: form.timeout,
      database: form.database,
      username: form.username || undefined,
      password: form.password || undefined
    }
    
    // 创建连接配置对象
    const connectionProfile: ConnectionProfile = {
      id: form.id,
      name: form.name,
      version: form.version,
      config,
      created_at: props.connection?.created_at || Date.now(),
      updated_at: Date.now()
    }
    
    // 触发保存事件
    emit('save', connectionProfile)
    
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

// 测试连接方法
const testConnection = async () => {
  if (!formRef.value) return
  
  try {
    await formRef.value.validate()
    
    isTesting.value = true
    
    // 目前只支持 v1.x 版本
    let config: any = {
      host: form.host,
      port: form.port,
      useSsl: form.useSsl,
      timeout: form.timeout,
      database: form.database,
      username: form.username || undefined,
      password: form.password || undefined
    }
    
    // 创建测试连接配置
    const testProfile: ConnectionProfile = {
      id: 'test_' + Date.now(),
      name: '测试连接',
      version: form.version,
      config,
      created_at: Date.now(),
      updated_at: Date.now()
    }
    
    // 调用后端测试连接
    const { invoke } = await import('@tauri-apps/api/core')
    const response = await invoke('test_connection', { profile: testProfile }) as any
    
    if (response.success) {
      ElMessage.success('连接测试成功！')
    } else {
      ElMessage.error(`连接测试失败：${response.error || '未知错误'}`)
    }
    
  } catch (error) {
    console.error('连接测试失败:', error)
    ElMessage.error(`连接测试失败：${error}`)
  } finally {
    isTesting.value = false
  }
}
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