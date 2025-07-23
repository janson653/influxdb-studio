<template>
  <el-dialog 
    v-model="visible" 
    :title="`创建${getDatabaseNameLabel()}`" 
    width="500px"
    :close-on-click-modal="false"
    @close="handleClose"
  >
    <el-form 
      :model="form" 
      :rules="rules" 
      ref="formRef" 
      label-width="120px"
      class="create-database-form"
    >
      <el-form-item :label="`${getDatabaseNameLabel()}名称`" prop="name">
        <el-input 
          v-model="form.name" 
          :placeholder="`请输入${getDatabaseNameLabel()}名称`"
          maxlength="64"
          show-word-limit
          clearable
        />
        <div class="form-help">
          名称只能包含字母、数字、下划线和连字符，不能以数字开头
        </div>
      </el-form-item>

      <el-form-item label="保留策略" prop="retentionPolicy">
        <el-input 
          v-model="form.retentionPolicy" 
          placeholder="例如: 30d, 1w, 1h"
          clearable
        />
        <div class="form-help">
          可选，格式：数字+单位（ns, u, ms, s, m, h, d, w）
        </div>
      </el-form-item>

      <el-form-item label="分片持续时间" prop="shardDuration">
        <el-input 
          v-model="form.shardDuration" 
          placeholder="例如: 1h, 1d"
          clearable
        />
        <div class="form-help">
          可选，默认根据保留策略自动计算
        </div>
      </el-form-item>

      <el-form-item label="副本数量" prop="replicationFactor">
        <el-input-number 
          v-model="form.replicationFactor" 
          :min="1" 
          :max="3"
          controls-position="right"
        />
        <div class="form-help">
          数据副本数量，影响数据可用性
        </div>
      </el-form-item>
    </el-form>

    <template #footer>
      <div class="dialog-footer">
        <el-button @click="handleClose">取消</el-button>
        <el-button 
          type="primary" 
          @click="handleSubmit" 
          :loading="loading"
          :disabled="!form.name.trim()"
        >
          创建
        </el-button>
      </div>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { ref, reactive, computed, watch } from 'vue'
import { ElMessage, type FormInstance, type FormRules } from 'element-plus'
import { useDatabaseStore } from '../../../stores/databaseStore'
import { useConnectionStore } from '../../../stores/connectionStore'
import { InfluxDBVersion } from '../../../types/influxdb'

interface Props {
  modelValue: boolean
}

interface Emits {
  (e: 'update:modelValue', value: boolean): void
  (e: 'created', database: any): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const databaseStore = useDatabaseStore()
const connectionStore = useConnectionStore()

const visible = computed({
  get: () => props.modelValue,
  set: (value) => emit('update:modelValue', value)
})

const formRef = ref<FormInstance>()
const loading = ref(false)

const form = reactive({
  name: '',
  retentionPolicy: '',
  shardDuration: '',
  replicationFactor: 1
})

const rules: FormRules = {
  name: [
    { required: true, message: '请输入数据库名称', trigger: 'blur' },
    { 
      pattern: /^[a-zA-Z_][a-zA-Z0-9_-]*$/, 
      message: '名称只能包含字母、数字、下划线和连字符，不能以数字开头', 
      trigger: 'blur' 
    },
    { min: 1, max: 64, message: '名称长度在1到64个字符之间', trigger: 'blur' }
  ],
  retentionPolicy: [
    { 
      pattern: /^$|^(\d+[ns|u|ms|s|m|h|d|w])+$/, 
      message: '保留策略格式不正确', 
      trigger: 'blur' 
    }
  ],
  shardDuration: [
    { 
      pattern: /^$|^(\d+[ns|u|ms|s|m|h|d|w])+$/, 
      message: '分片持续时间格式不正确', 
      trigger: 'blur' 
    }
  ]
}

const getDatabaseNameLabel = () => {
  const activeConnection = connectionStore.activeConnectionConfig
  return activeConnection?.version === InfluxDBVersion.V2 ? '存储桶' : '数据库'
}

const handleClose = () => {
  visible.value = false
  resetForm()
}

const resetForm = () => {
  form.name = ''
  form.retentionPolicy = ''
  form.shardDuration = ''
  form.replicationFactor = 1
  formRef.value?.clearValidate()
}

const handleSubmit = async () => {
  if (!formRef.value) return
  
  try {
    await formRef.value.validate()
    loading.value = true
    
    const result = await databaseStore.createDatabase(
      form.name.trim(),
      form.retentionPolicy ? { 
        name: form.retentionPolicy,
        duration: '7d',
        shardGroupDuration: '1d',
        replicaN: 1,
        default: false
      } : undefined
    )
    
    ElMessage.success(`${getDatabaseNameLabel()}创建成功`)
    emit('created', result)
    handleClose()
  } catch (error: any) {
    ElMessage.error(error.message || '创建失败')
  } finally {
    loading.value = false
  }
}

// 监听对话框打开，重置表单
watch(visible, (newVal) => {
  if (newVal) {
    resetForm()
  }
})
</script>

<style scoped>
.create-database-form {
  padding: 20px 0;
}

.form-help {
  font-size: 12px;
  color: #909399;
  margin-top: 4px;
  line-height: 1.4;
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
}

:deep(.el-form-item__label) {
  font-weight: 500;
  color: #303133;
}

:deep(.el-input-number) {
  width: 100%;
}
</style>
