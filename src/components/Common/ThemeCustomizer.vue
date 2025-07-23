<template>
  <div class="theme-customizer">
    <div class="customizer-header">
      <h3>主题定制</h3>
      <el-button size="small" @click="resetToDefault">重置为默认</el-button>
    </div>

    <div class="customizer-content">
      <el-tabs v-model="activeTab" class="customizer-tabs">
        <el-tab-pane label="颜色" name="colors">
          <div class="color-section">
            <h4>主色调</h4>
            <div class="color-grid">
              <div class="color-item">
                <label>主背景色</label>
                <el-color-picker 
                  v-model="themeColors.bgPrimary" 
                  @change="updateTheme"
                  show-alpha
                />
                <span class="color-value">{{ themeColors.bgPrimary }}</span>
              </div>
              <div class="color-item">
                <label>次背景色</label>
                <el-color-picker 
                  v-model="themeColors.bgSecondary" 
                  @change="updateTheme"
                  show-alpha
                />
                <span class="color-value">{{ themeColors.bgSecondary }}</span>
              </div>
              <div class="color-item">
                <label>主文本色</label>
                <el-color-picker 
                  v-model="themeColors.textPrimary" 
                  @change="updateTheme"
                />
                <span class="color-value">{{ themeColors.textPrimary }}</span>
              </div>
              <div class="color-item">
                <label>次文本色</label>
                <el-color-picker 
                  v-model="themeColors.textSecondary" 
                  @change="updateTheme"
                />
                <span class="color-value">{{ themeColors.textSecondary }}</span>
              </div>
            </div>
          </div>

          <div class="color-section">
            <h4>强调色</h4>
            <div class="color-grid">
              <div class="color-item">
                <label>主强调色</label>
                <el-color-picker 
                  v-model="themeColors.accentPrimary" 
                  @change="updateTheme"
                />
                <span class="color-value">{{ themeColors.accentPrimary }}</span>
              </div>
              <div class="color-item">
                <label>成功色</label>
                <el-color-picker 
                  v-model="themeColors.accentGreen" 
                  @change="updateTheme"
                />
                <span class="color-value">{{ themeColors.accentGreen }}</span>
              </div>
              <div class="color-item">
                <label>警告色</label>
                <el-color-picker 
                  v-model="themeColors.accentOrange" 
                  @change="updateTheme"
                />
                <span class="color-value">{{ themeColors.accentOrange }}</span>
              </div>
              <div class="color-item">
                <label>错误色</label>
                <el-color-picker 
                  v-model="themeColors.accentRed" 
                  @change="updateTheme"
                />
                <span class="color-value">{{ themeColors.accentRed }}</span>
              </div>
            </div>
          </div>

          <div class="color-section">
            <h4>边框色</h4>
            <div class="color-grid">
              <div class="color-item">
                <label>主边框色</label>
                <el-color-picker 
                  v-model="themeColors.border" 
                  @change="updateTheme"
                />
                <span class="color-value">{{ themeColors.border }}</span>
              </div>
              <div class="color-item">
                <label>浅边框色</label>
                <el-color-picker 
                  v-model="themeColors.borderLight" 
                  @change="updateTheme"
                />
                <span class="color-value">{{ themeColors.borderLight }}</span>
              </div>
            </div>
          </div>
        </el-tab-pane>

        <el-tab-pane label="字体" name="fonts">
          <div class="font-section">
            <h4>字体设置</h4>
            <div class="font-grid">
              <div class="font-item">
                <label>等宽字体</label>
                <el-select v-model="themeFonts.mono" @change="updateTheme">
                  <el-option label="JetBrains Mono" value="'JetBrains Mono', 'Fira Code', 'Consolas', monospace" />
                  <el-option label="Fira Code" value="'Fira Code', 'Consolas', monospace" />
                  <el-option label="Consolas" value="'Consolas', monospace" />
                  <el-option label="Monaco" value="'Monaco', 'Menlo', monospace" />
                  <el-option label="Source Code Pro" value="'Source Code Pro', monospace" />
                </el-select>
              </div>
              <div class="font-item">
                <label>无衬线字体</label>
                <el-select v-model="themeFonts.sans" @change="updateTheme">
                  <el-option label="系统默认" value="-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif" />
                  <el-option label="Segoe UI" value="'Segoe UI', sans-serif" />
                  <el-option label="Roboto" value="'Roboto', sans-serif" />
                  <el-option label="Open Sans" value="'Open Sans', sans-serif" />
                  <el-option label="Inter" value="'Inter', sans-serif" />
                </el-select>
              </div>
            </div>
          </div>

          <div class="font-section">
            <h4>字体大小</h4>
            <div class="font-grid">
              <div class="font-item">
                <label>小字体</label>
                <el-input-number 
                  v-model="themeFonts.sizeXs" 
                  :min="8" 
                  :max="16" 
                  @change="updateTheme"
                />
                <span class="unit">px</span>
              </div>
              <div class="font-item">
                <label>标准字体</label>
                <el-input-number 
                  v-model="themeFonts.sizeMd" 
                  :min="12" 
                  :max="20" 
                  @change="updateTheme"
                />
                <span class="unit">px</span>
              </div>
              <div class="font-item">
                <label>大字体</label>
                <el-input-number 
                  v-model="themeFonts.sizeLg" 
                  :min="14" 
                  :max="24" 
                  @change="updateTheme"
                />
                <span class="unit">px</span>
              </div>
            </div>
          </div>
        </el-tab-pane>

        <el-tab-pane label="间距" name="spacing">
          <div class="spacing-section">
            <h4>间距设置</h4>
            <div class="spacing-grid">
              <div class="spacing-item">
                <label>小间距</label>
                <el-input-number 
                  v-model="themeSpacing.xs" 
                  :min="2" 
                  :max="12" 
                  @change="updateTheme"
                />
                <span class="unit">px</span>
              </div>
              <div class="spacing-item">
                <label>标准间距</label>
                <el-input-number 
                  v-model="themeSpacing.md" 
                  :min="8" 
                  :max="24" 
                  @change="updateTheme"
                />
                <span class="unit">px</span>
              </div>
              <div class="spacing-item">
                <label>大间距</label>
                <el-input-number 
                  v-model="themeSpacing.lg" 
                  :min="12" 
                  :max="32" 
                  @change="updateTheme"
                />
                <span class="unit">px</span>
              </div>
            </div>
          </div>

          <div class="spacing-section">
            <h4>圆角设置</h4>
            <div class="spacing-grid">
              <div class="spacing-item">
                <label>小圆角</label>
                <el-input-number 
                  v-model="themeSpacing.borderRadiusSmall" 
                  :min="0" 
                  :max="12" 
                  @change="updateTheme"
                />
                <span class="unit">px</span>
              </div>
              <div class="spacing-item">
                <label>标准圆角</label>
                <el-input-number 
                  v-model="themeSpacing.borderRadius" 
                  :min="0" 
                  :max="16" 
                  @change="updateTheme"
                />
                <span class="unit">px</span>
              </div>
            </div>
          </div>
        </el-tab-pane>

        <el-tab-pane label="预设" name="presets">
          <div class="preset-section">
            <h4>主题预设</h4>
            <div class="preset-grid">
              <div 
                v-for="preset in themePresets" 
                :key="preset.name"
                class="preset-item"
                :class="{ active: currentPreset === preset.name }"
                @click="applyPreset(preset)"
              >
                <div class="preset-preview" :style="getPresetPreviewStyle(preset)">
                  <div class="preview-header"></div>
                  <div class="preview-content">
                    <div class="preview-sidebar"></div>
                    <div class="preview-main"></div>
                  </div>
                </div>
                <div class="preset-name">{{ preset.label }}</div>
              </div>
            </div>
          </div>
        </el-tab-pane>
      </el-tabs>
    </div>

    <div class="customizer-footer">
      <el-button @click="saveTheme" type="primary">保存主题</el-button>
      <el-button @click="exportTheme">导出主题</el-button>
      <el-button @click="importTheme">导入主题</el-button>
    </div>

    <!-- 文件输入用于导入主题 -->
    <input 
      ref="fileInput" 
      type="file" 
      accept=".json" 
      style="display: none" 
      @change="handleFileImport"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { ElMessage } from 'element-plus'
// import { useThemeStore } from '../../stores/themeStore'

// const themeStore = useThemeStore()
const activeTab = ref('colors')
const currentPreset = ref('geek')
const fileInput = ref<HTMLInputElement>()

// 主题颜色
const themeColors = reactive({
  bgPrimary: '#2B2B2B',
  bgSecondary: '#3C3F41',
  textPrimary: '#A9B7C6',
  textSecondary: '#808080',
  accentPrimary: '#6897BB',
  accentGreen: '#6A8759',
  accentOrange: '#CC7832',
  accentRed: '#BC3F3C',
  border: '#555555',
  borderLight: '#666666'
})

// 主题字体
const themeFonts = reactive({
  mono: "'JetBrains Mono', 'Fira Code', 'Consolas', monospace",
  sans: "-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif",
  sizeXs: 11,
  sizeMd: 14,
  sizeLg: 16
})

// 主题间距
const themeSpacing = reactive({
  xs: 4,
  md: 12,
  lg: 16,
  borderRadius: 3,
  borderRadiusSmall: 2
})

// 主题预设
const themePresets = [
  {
    name: 'geek',
    label: 'Geek 主题',
    colors: {
      bgPrimary: '#2B2B2B',
      bgSecondary: '#3C3F41',
      textPrimary: '#A9B7C6',
      textSecondary: '#808080',
      accentPrimary: '#6897BB',
      accentGreen: '#6A8759',
      accentOrange: '#CC7832',
      accentRed: '#BC3F3C',
      border: '#555555',
      borderLight: '#666666'
    }
  },
  {
    name: 'dark',
    label: '深色主题',
    colors: {
      bgPrimary: '#1E1E1E',
      bgSecondary: '#252526',
      textPrimary: '#CCCCCC',
      textSecondary: '#969696',
      accentPrimary: '#007ACC',
      accentGreen: '#4EC9B0',
      accentOrange: '#CE9178',
      accentRed: '#F44747',
      border: '#3E3E42',
      borderLight: '#5A5A5A'
    }
  },
  {
    name: 'light',
    label: '浅色主题',
    colors: {
      bgPrimary: '#FFFFFF',
      bgSecondary: '#F3F3F3',
      textPrimary: '#333333',
      textSecondary: '#666666',
      accentPrimary: '#007ACC',
      accentGreen: '#28A745',
      accentOrange: '#FFC107',
      accentRed: '#DC3545',
      border: '#E1E1E1',
      borderLight: '#CCCCCC'
    }
  }
]

// 更新主题
const updateTheme = () => {
  const cssVars = {
    '--geek-bg-primary': themeColors.bgPrimary,
    '--geek-bg-secondary': themeColors.bgSecondary,
    '--geek-text-primary': themeColors.textPrimary,
    '--geek-text-secondary': themeColors.textSecondary,
    '--geek-accent-primary': themeColors.accentPrimary,
    '--geek-accent-green': themeColors.accentGreen,
    '--geek-accent-orange': themeColors.accentOrange,
    '--geek-border': themeColors.border,
    '--geek-border-light': themeColors.borderLight,
    '--geek-font-mono': themeFonts.mono,
    '--geek-font-sans': themeFonts.sans,
    '--geek-font-size-xs': `${themeFonts.sizeXs}px`,
    '--geek-font-size-md': `${themeFonts.sizeMd}px`,
    '--geek-font-size-lg': `${themeFonts.sizeLg}px`,
    '--geek-spacing-xs': `${themeSpacing.xs}px`,
    '--geek-spacing-md': `${themeSpacing.md}px`,
    '--geek-spacing-lg': `${themeSpacing.lg}px`,
    '--geek-border-radius': `${themeSpacing.borderRadius}px`,
    '--geek-border-radius-small': `${themeSpacing.borderRadiusSmall}px`
  }

  Object.entries(cssVars).forEach(([key, value]) => {
    document.documentElement.style.setProperty(key, value)
  })
}

// 应用预设
const applyPreset = (preset: any) => {
  currentPreset.value = preset.name
  Object.assign(themeColors, preset.colors)
  updateTheme()
  ElMessage.success(`已应用 ${preset.label}`)
}

// 获取预设预览样式
const getPresetPreviewStyle = (preset: any) => {
  return {
    '--preview-bg-primary': preset.colors.bgPrimary,
    '--preview-bg-secondary': preset.colors.bgSecondary,
    '--preview-border': preset.colors.border
  }
}

// 重置为默认
const resetToDefault = () => {
  applyPreset(themePresets[0])
  ElMessage.info('已重置为默认主题')
}

// 保存主题
const saveTheme = () => {
  const themeData = {
    colors: { ...themeColors },
    fonts: { ...themeFonts },
    spacing: { ...themeSpacing },
    preset: currentPreset.value
  }
  
  localStorage.setItem('custom-theme', JSON.stringify(themeData))
  ElMessage.success('主题已保存')
}

// 导出主题
const exportTheme = () => {
  const themeData = {
    colors: { ...themeColors },
    fonts: { ...themeFonts },
    spacing: { ...themeSpacing },
    preset: currentPreset.value
  }
  
  const blob = new Blob([JSON.stringify(themeData, null, 2)], { type: 'application/json' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = 'influxdb-studio-theme.json'
  a.click()
  URL.revokeObjectURL(url)
  
  ElMessage.success('主题已导出')
}

// 导入主题
const importTheme = () => {
  fileInput.value?.click()
}

// 处理文件导入
const handleFileImport = (event: Event) => {
  const file = (event.target as HTMLInputElement).files?.[0]
  if (!file) return
  
  const reader = new FileReader()
  reader.onload = (e) => {
    try {
      const themeData = JSON.parse(e.target?.result as string)
      
      if (themeData.colors) Object.assign(themeColors, themeData.colors)
      if (themeData.fonts) Object.assign(themeFonts, themeData.fonts)
      if (themeData.spacing) Object.assign(themeSpacing, themeData.spacing)
      if (themeData.preset) currentPreset.value = themeData.preset
      
      updateTheme()
      ElMessage.success('主题导入成功')
    } catch (error) {
      ElMessage.error('主题文件格式错误')
    }
  }
  reader.readAsText(file)
}

// 加载保存的主题
const loadSavedTheme = () => {
  const saved = localStorage.getItem('custom-theme')
  if (saved) {
    try {
      const themeData = JSON.parse(saved)
      
      if (themeData.colors) Object.assign(themeColors, themeData.colors)
      if (themeData.fonts) Object.assign(themeFonts, themeData.fonts)
      if (themeData.spacing) Object.assign(themeSpacing, themeData.spacing)
      if (themeData.preset) currentPreset.value = themeData.preset
      
      updateTheme()
    } catch (error) {
      console.error('加载保存的主题失败:', error)
    }
  }
}

onMounted(() => {
  loadSavedTheme()
})
</script>

<style scoped>
.theme-customizer {
  height: 100%;
  display: flex;
  flex-direction: column;
  background: var(--geek-bg-primary);
  color: var(--geek-text-primary);
}

.customizer-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px;
  border-bottom: 1px solid var(--geek-border);
  background: var(--geek-bg-secondary);
}

.customizer-header h3 {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
}

.customizer-content {
  flex: 1;
  overflow: hidden;
  padding: 20px;
}

.customizer-tabs {
  height: 100%;
}

.color-section,
.font-section,
.spacing-section,
.preset-section {
  margin-bottom: 24px;
}

.color-section h4,
.font-section h4,
.spacing-section h4,
.preset-section h4 {
  margin: 0 0 16px 0;
  font-size: 14px;
  font-weight: 600;
  color: var(--geek-text-primary);
  border-bottom: 1px solid var(--geek-border);
  padding-bottom: 8px;
}

.color-grid,
.font-grid,
.spacing-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 16px;
}

.color-item,
.font-item,
.spacing-item {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.color-item label,
.font-item label,
.spacing-item label {
  font-size: 12px;
  color: var(--geek-text-secondary);
  font-weight: 500;
}

.color-value {
  font-size: 11px;
  color: var(--geek-text-secondary);
  font-family: var(--geek-font-mono);
}

.unit {
  font-size: 12px;
  color: var(--geek-text-secondary);
  margin-left: 4px;
}

.preset-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
  gap: 16px;
}

.preset-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  padding: 12px;
  border: 2px solid var(--geek-border);
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.preset-item:hover {
  border-color: var(--geek-accent-primary);
}

.preset-item.active {
  border-color: var(--geek-accent-primary);
  background: var(--geek-bg-highlight);
}

.preset-preview {
  width: 80px;
  height: 60px;
  border-radius: 4px;
  overflow: hidden;
  background: var(--preview-bg-primary);
  border: 1px solid var(--preview-border);
}

.preview-header {
  height: 12px;
  background: var(--preview-bg-secondary);
  border-bottom: 1px solid var(--preview-border);
}

.preview-content {
  display: flex;
  height: calc(100% - 12px);
}

.preview-sidebar {
  width: 20px;
  background: var(--preview-bg-secondary);
  border-right: 1px solid var(--preview-border);
}

.preview-main {
  flex: 1;
  background: var(--preview-bg-primary);
}

.preset-name {
  font-size: 12px;
  text-align: center;
  color: var(--geek-text-primary);
}

.customizer-footer {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  padding: 16px 20px;
  border-top: 1px solid var(--geek-border);
  background: var(--geek-bg-secondary);
}

/* 自定义滚动条 */
.customizer-content::-webkit-scrollbar {
  width: 6px;
}

.customizer-content::-webkit-scrollbar-track {
  background: var(--geek-bg-primary);
}

.customizer-content::-webkit-scrollbar-thumb {
  background: var(--geek-border);
  border-radius: 3px;
}

.customizer-content::-webkit-scrollbar-thumb:hover {
  background: var(--geek-text-secondary);
}
</style> 