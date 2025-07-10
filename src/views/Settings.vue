<template>
  <div class="settings">
    <el-container>
      <el-header class="header">
        <h2>设置</h2>
      </el-header>
      
      <el-main>
        <el-row :gutter="20">
          <el-col :span="8">
            <!-- 设置导航 -->
            <el-card>
              <template #header>
                <span>设置分类</span>
              </template>
              
              <el-menu
                :default-active="activeSetting"
                @select="handleSettingSelect"
              >
                <el-menu-item index="general">
                  <el-icon><Setting /></el-icon>
                  <span>常规设置</span>
                </el-menu-item>
                <el-menu-item index="editor">
                  <el-icon><Edit /></el-icon>
                  <span>编辑器设置</span>
                </el-menu-item>
                <el-menu-item index="connection">
                  <el-icon><Connection /></el-icon>
                  <span>连接设置</span>
                </el-menu-item>
                <el-menu-item index="query">
                  <el-icon><Search /></el-icon>
                  <span>查询设置</span>
                </el-menu-item>
                <el-menu-item index="export">
                  <el-icon><Download /></el-icon>
                  <span>导出设置</span>
                </el-menu-item>
                <el-menu-item index="about">
                  <el-icon><InfoFilled /></el-icon>
                  <span>关于</span>
                </el-menu-item>
              </el-menu>
            </el-card>
          </el-col>
          
          <el-col :span="16">
            <!-- 设置内容 -->
            <el-card>
              <template #header>
                <span>{{ getSettingTitle() }}</span>
              </template>
              
              <!-- 常规设置 -->
              <div v-if="activeSetting === 'general'">
                <el-form :model="generalSettings" label-width="120px">
                  <el-form-item label="语言">
                    <el-select v-model="generalSettings.language" style="width: 200px;">
                      <el-option label="简体中文" value="zh-CN" />
                      <el-option label="English" value="en-US" />
                    </el-select>
                  </el-form-item>
                  
                  <el-form-item label="主题">
                    <el-radio-group v-model="generalSettings.theme">
                      <el-radio label="light">浅色主题</el-radio>
                      <el-radio label="dark">深色主题</el-radio>
                      <el-radio label="auto">跟随系统</el-radio>
                    </el-radio-group>
                  </el-form-item>
                  
                  <el-form-item label="自动保存">
                    <el-switch v-model="generalSettings.autoSave" />
                  </el-form-item>
                  
                  <el-form-item label="自动检查更新">
                    <el-switch v-model="generalSettings.autoUpdate" />
                  </el-form-item>
                </el-form>
              </div>
              
              <!-- 编辑器设置 -->
              <div v-if="activeSetting === 'editor'">
                <el-form :model="editorSettings" label-width="120px">
                  <el-form-item label="字体大小">
                    <el-input-number 
                      v-model="editorSettings.fontSize" 
                      :min="10" 
                      :max="24"
                      style="width: 200px;"
                    />
                  </el-form-item>
                  
                  <el-form-item label="字体族">
                    <el-select v-model="editorSettings.fontFamily" style="width: 200px;">
                      <el-option label="Monaco" value="Monaco" />
                      <el-option label="Consolas" value="Consolas" />
                      <el-option label="Courier New" value="Courier New" />
                      <el-option label="Source Code Pro" value="Source Code Pro" />
                    </el-select>
                  </el-form-item>
                  
                  <el-form-item label="显示行号">
                    <el-switch v-model="editorSettings.showLineNumbers" />
                  </el-form-item>
                  
                  <el-form-item label="显示小地图">
                    <el-switch v-model="editorSettings.showMinimap" />
                  </el-form-item>
                  
                  <el-form-item label="自动换行">
                    <el-switch v-model="editorSettings.wordWrap" />
                  </el-form-item>
                  
                  <el-form-item label="制表符大小">
                    <el-input-number 
                      v-model="editorSettings.tabSize" 
                      :min="2" 
                      :max="8"
                      style="width: 200px;"
                    />
                  </el-form-item>
                </el-form>
              </div>
              
              <!-- 连接设置 -->
              <div v-if="activeSetting === 'connection'">
                <el-form :model="connectionSettings" label-width="120px">
                  <el-form-item label="默认超时时间">
                    <el-input-number 
                      v-model="connectionSettings.defaultTimeout" 
                      :min="1000" 
                      :max="60000"
                      :step="1000"
                      style="width: 200px;"
                    />
                    <span style="margin-left: 8px; color: #909399;">毫秒</span>
                  </el-form-item>
                  
                  <el-form-item label="连接池大小">
                    <el-input-number 
                      v-model="connectionSettings.poolSize" 
                      :min="1" 
                      :max="20"
                      style="width: 200px;"
                    />
                  </el-form-item>
                  
                  <el-form-item label="自动重连">
                    <el-switch v-model="connectionSettings.autoReconnect" />
                  </el-form-item>
                  
                  <el-form-item label="重连间隔">
                    <el-input-number 
                      v-model="connectionSettings.reconnectInterval" 
                      :min="1000" 
                      :max="30000"
                      :step="1000"
                      style="width: 200px;"
                      :disabled="!connectionSettings.autoReconnect"
                    />
                    <span style="margin-left: 8px; color: #909399;">毫秒</span>
                  </el-form-item>
                </el-form>
              </div>
              
              <!-- 查询设置 -->
              <div v-if="activeSetting === 'query'">
                <el-form :model="querySettings" label-width="120px">
                  <el-form-item label="查询超时时间">
                    <el-input-number 
                      v-model="querySettings.timeout" 
                      :min="5000" 
                      :max="300000"
                      :step="5000"
                      style="width: 200px;"
                    />
                    <span style="margin-left: 8px; color: #909399;">毫秒</span>
                  </el-form-item>
                  
                  <el-form-item label="最大结果数量">
                    <el-input-number 
                      v-model="querySettings.maxResults" 
                      :min="100" 
                      :max="100000"
                      :step="100"
                      style="width: 200px;"
                    />
                  </el-form-item>
                  
                  <el-form-item label="保存查询历史">
                    <el-switch v-model="querySettings.saveHistory" />
                  </el-form-item>
                  
                  <el-form-item label="历史记录数量">
                    <el-input-number 
                      v-model="querySettings.historyLimit" 
                      :min="10" 
                      :max="1000"
                      :step="10"
                      style="width: 200px;"
                      :disabled="!querySettings.saveHistory"
                    />
                  </el-form-item>
                  
                  <el-form-item label="自动执行">
                    <el-switch v-model="querySettings.autoExecute" />
                  </el-form-item>
                </el-form>
              </div>
              
              <!-- 导出设置 -->
              <div v-if="activeSetting === 'export'">
                <el-form :model="exportSettings" label-width="120px">
                  <el-form-item label="默认导出格式">
                    <el-select v-model="exportSettings.defaultFormat" style="width: 200px;">
                      <el-option label="CSV" value="csv" />
                      <el-option label="JSON" value="json" />
                      <el-option label="Excel" value="excel" />
                    </el-select>
                  </el-form-item>
                  
                  <el-form-item label="包含列名">
                    <el-switch v-model="exportSettings.includeHeaders" />
                  </el-form-item>
                  
                  <el-form-item label="时间格式">
                    <el-select v-model="exportSettings.timeFormat" style="width: 200px;">
                      <el-option label="ISO 8601" value="iso" />
                      <el-option label="Unix 时间戳" value="unix" />
                      <el-option label="本地时间" value="local" />
                    </el-select>
                  </el-form-item>
                  
                  <el-form-item label="数字精度">
                    <el-input-number 
                      v-model="exportSettings.numberPrecision" 
                      :min="0" 
                      :max="10"
                      style="width: 200px;"
                    />
                  </el-form-item>
                </el-form>
              </div>
              
              <!-- 关于 -->
              <div v-if="activeSetting === 'about'">
                <el-descriptions :column="1" border>
                  <el-descriptions-item label="应用名称">
                    InfluxDB Studio
                  </el-descriptions-item>
                  <el-descriptions-item label="版本">
                    {{ appVersion }}
                  </el-descriptions-item>
                  <el-descriptions-item label="构建时间">
                    {{ buildTime }}
                  </el-descriptions-item>
                  <el-descriptions-item label="技术栈">
                    Vue 3 + TypeScript + Tauri
                  </el-descriptions-item>
                  <el-descriptions-item label="许可证">
                    MIT License
                  </el-descriptions-item>
                </el-descriptions>
                
                <div style="margin-top: 20px;">
                  <el-button type="primary" @click="checkUpdate">
                    检查更新
                  </el-button>
                  <el-button @click="openLogs">
                    查看日志
                  </el-button>
                </div>
              </div>
              
              <!-- 保存按钮 -->
              <div style="margin-top: 30px; text-align: center;">
                <el-button type="primary" @click="saveSettings">
                  保存设置
                </el-button>
                <el-button @click="resetSettings">
                  重置设置
                </el-button>
              </div>
            </el-card>
          </el-col>
        </el-row>
      </el-main>
    </el-container>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue'
import { ElMessage } from 'element-plus'
import { 
  Setting, 
  Edit, 
  Connection, 
  Search, 
  Download, 
  InfoFilled 
} from '@element-plus/icons-vue'

// 响应式数据
const activeSetting = ref('general')

// 应用信息
const appVersion = ref('1.0.0')
const buildTime = ref('2024-01-01')

// 设置数据
const generalSettings = reactive({
  language: 'zh-CN',
  theme: 'light',
  autoSave: true,
  autoUpdate: true
})

const editorSettings = reactive({
  fontSize: 14,
  fontFamily: 'Monaco',
  showLineNumbers: true,
  showMinimap: false,
  wordWrap: true,
  tabSize: 2
})

const connectionSettings = reactive({
  defaultTimeout: 5000,
  poolSize: 5,
  autoReconnect: true,
  reconnectInterval: 5000
})

const querySettings = reactive({
  timeout: 30000,
  maxResults: 10000,
  saveHistory: true,
  historyLimit: 100,
  autoExecute: false
})

const exportSettings = reactive({
  defaultFormat: 'csv',
  includeHeaders: true,
  timeFormat: 'iso',
  numberPrecision: 2
})

// 方法
const handleSettingSelect = (key: string) => {
  activeSetting.value = key
}

const getSettingTitle = () => {
  const titles: Record<string, string> = {
    general: '常规设置',
    editor: '编辑器设置',
    connection: '连接设置',
    query: '查询设置',
    export: '导出设置',
    about: '关于'
  }
  return titles[activeSetting.value] || '设置'
}

const saveSettings = () => {
  // TODO: 保存设置到本地存储
  const settings = {
    general: generalSettings,
    editor: editorSettings,
    connection: connectionSettings,
    query: querySettings,
    export: exportSettings
  }
  
  try {
    localStorage.setItem('influxdb-settings', JSON.stringify(settings))
    ElMessage.success('设置已保存')
  } catch (error) {
    ElMessage.error('保存设置失败')
  }
}

const resetSettings = () => {
  // TODO: 重置为默认设置
  ElMessage.info('设置已重置为默认值')
}

const checkUpdate = () => {
  // TODO: 检查应用更新
  ElMessage.info('检查更新功能开发中...')
}

const openLogs = () => {
  // TODO: 打开日志文件
  ElMessage.info('查看日志功能开发中...')
}

// 加载设置
const loadSettings = () => {
  try {
    const saved = localStorage.getItem('influxdb-settings')
    if (saved) {
      const settings = JSON.parse(saved)
      Object.assign(generalSettings, settings.general || {})
      Object.assign(editorSettings, settings.editor || {})
      Object.assign(connectionSettings, settings.connection || {})
      Object.assign(querySettings, settings.query || {})
      Object.assign(exportSettings, settings.export || {})
    }
  } catch (error) {
    console.error('加载设置失败:', error)
  }
}

// 初始化时加载设置
loadSettings()
</script>

<style scoped>
.settings {
  height: 100vh;
  background-color: #f5f5f5;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  background-color: white;
  border-bottom: 1px solid #e4e7ed;
  padding: 0 20px;
}

.header h2 {
  margin: 0;
  color: #303133;
}

.el-menu {
  border-right: none;
}

.el-menu-item {
  display: flex;
  align-items: center;
  gap: 8px;
}
</style> 