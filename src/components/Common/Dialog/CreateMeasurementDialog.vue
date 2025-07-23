<template>
  <el-dialog 
    v-model="visible" 
    :title="`创建${getMeasurementNameLabel()}`" 
    width="600px"
    :close-on-click-modal="false"
    @close="handleClose"
  >
    <el-form 
      :model="form" 
      :rules="rules" 
      ref="formRef" 
      label-width="120px"
      class="create-measurement-form"
    >
      <el-form-item :label="`${getMeasurementNameLabel()}名称`" prop="name">
        <el-input 
          v-model="form.name" 
          :placeholder="`请输入${getMeasurementNameLabel()}名称`"
          maxlength="64"
          show-word-limit
          clearable
        />
        <div class="form-help">
          名称只能包含字母、数字、下划线和连字符，不能以数字开头
        </div>
      </el-form-item>

      <el-form-item label="所属数据库" prop="database">
        <el-select 
          v-model="form.database" 
          placeholder="请选择数据库"
          clearable
          filterable
        >
          <el-option 
            v-for="db in databases" 
            :key="db.name" 
            :label="db.name" 
            :value="db.name" 
          />
        </el-select>
      </el-form-item>

      <el-form-item label="字段配置">
        <div class="fields-container">
          <div 
            v-for="(field, index) in form.fields" 
            :key="index" 
            class="field-item"
          >
            <el-input 
              v-model="field.name" 
              placeholder="字段名"
              style="width: 200px;"
            />
            <el-select 
              v-model="field.type" 
              placeholder="类型"
              style="width: 120px;"
            >
              <el-option label="float" value="float" />
              <el-option label="integer" value="integer" />
              <el-option label="string" value="string" />
              <el-option label="boolean" value="boolean" />
            </el-select>
            <el-button 
              type="danger" 
              size="small" 
              @click="removeField(index)"
              :disabled="form.fields.length <= 1"
            >
              删除
            </el-button>
          </div>
          <el-button type="primary" size="small" @click="addField">
            添加字段
          </el-button>
        </div>
      </el-form-item>

      <el-form-item label="标签配置">
        <div class="tags-container">
          <div 
            v-for="(tag, index) in form.tags" 
            :key="index" 
            class="tag-item"
          >
            <el-input 
              v-model="tag.key" 
              placeholder="标签键"
              style="width: 200px;"
            />
            <el-input 
              v-model="tag.value" 
              placeholder="标签值"
              style="width: 200px;"
            />
            <el-button 
              type="danger" 
              size="small" 
              @click="removeTag(index)"
            >
              删除
            </el-button>
          </div>
          <el-button type="primary" size="small" @click="addTag">
            添加标签
          </el-button>
        </div>
      </el-form-item>

      <el-form-item label="时间精度" prop="precision">
        <el-select v-model="form.precision" placeholder="选择时间精度">
          <el-option label="纳秒 (ns)" value="ns" />
          <el-option label="微秒 (u)" value="u" />
          <el-option label="毫秒 (ms)" value="ms" />
          <el-option label="秒 (s)" value="s" />
          <el-option label="分钟 (m)" value="m" />
          <el-option label="小时 (h)" value="h" />
        </el-select>
        <div class="form-help">
          数据写入时的时间精度，默认为纳秒
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
          :disabled="!form.name.trim() || !form.database"
        >
          创建
        </el-button>
      </div>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { ref, reactive, computed, watch, onMounted } from 'vue'
import { ElMessage, type FormInstance, type FormRules } from 'element-plus'
import { useMeasurementStore } from '../../../stores/measurementStore'
import { useDatabaseStore } from '../../../stores/databaseStore'
// import { useConnectionStore } from '../../../stores/connectionStore'

interface Props {
  modelValue: boolean
  selectedDatabase?: string
}

interface Emits {
  (e: 'update:modelValue', value: boolean): void
  (e: 'created', measurement: any): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const measurementStore = useMeasurementStore()
const databaseStore = useDatabaseStore()
// const connectionStore = useConnectionStore()

const visible = computed({
  get: () => props.modelValue,
  set: (value) => emit('update:modelValue', value)
})

const formRef = ref<FormInstance>()
const loading = ref(false)
const databases = ref<any[]>([])

const form = reactive({
  name: '',
  database: '',
  fields: [{ name: '', type: 'float' }],
  tags: [{ key: '', value: '' }],
  precision: 'ns'
})

const rules: FormRules = {
  name: [
    { required: true, message: '请输入measurement名称', trigger: 'blur' },
    { 
      pattern: /^[a-zA-Z_][a-zA-Z0-9_-]*$/, 
      message: '名称只能包含字母、数字、下划线和连字符，不能以数字开头', 
      trigger: 'blur' 
    },
    { min: 1, max: 64, message: '名称长度在1到64个字符之间', trigger: 'blur' }
  ],
  database: [
    { required: true, message: '请选择数据库', trigger: 'change' }
  ]
}

const getMeasurementNameLabel = () => {
  return 'measurement'
}

const loadDatabases = async () => {
  try {
    await databaseStore.fetchDatabases()
    databases.value = databaseStore.databases || []
  } catch (error) {
    console.error('加载数据库列表失败:', error)
  }
}

const addField = () => {
  form.fields.push({ name: '', type: 'float' })
}

const removeField = (index: number) => {
  if (form.fields.length > 1) {
    form.fields.splice(index, 1)
  }
}

const addTag = () => {
  form.tags.push({ key: '', value: '' })
}

const removeTag = (index: number) => {
  form.tags.splice(index, 1)
}

const handleClose = () => {
  visible.value = false
  resetForm()
}

const resetForm = () => {
  form.name = ''
  form.database = props.selectedDatabase || ''
  form.fields = [{ name: '', type: 'float' }]
  form.tags = [{ key: '', value: '' }]
  form.precision = 'ns'
  formRef.value?.clearValidate()
}

const handleSubmit = async () => {
  if (!formRef.value) return
  
  try {
    await formRef.value.validate()
    loading.value = true
    
    // 过滤空字段和标签
    const validFields = form.fields.filter(f => f.name.trim())
    const validTags = form.tags.filter(t => t.key.trim() && t.value.trim())
    
    if (validFields.length === 0) {
      ElMessage.warning('请至少添加一个字段')
      return
    }
    
    const result = await measurementStore.createMeasurement(
      form.name.trim(),
      validFields.map(f => ({ name: f.name, type: f.type, value: 0 })),
      validTags.map(t => ({ name: t.key, value: t.value }))
    )
    
    ElMessage.success(`${getMeasurementNameLabel()}创建成功`)
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
    loadDatabases()
  }
})

onMounted(() => {
  if (props.selectedDatabase) {
    form.database = props.selectedDatabase
  }
})
</script>

<style scoped>
.create-measurement-form {
  padding: 20px 0;
}

.form-help {
  font-size: 12px;
  color: #909399;
  margin-top: 4px;
  line-height: 1.4;
}

.fields-container,
.tags-container {
  border: 1px solid #dcdfe6;
  border-radius: 4px;
  padding: 12px;
  background-color: #fafafa;
}

.field-item,
.tag-item {
  display: flex;
  gap: 8px;
  margin-bottom: 8px;
  align-items: center;
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
</style> 