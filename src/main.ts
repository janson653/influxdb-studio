import { createApp } from 'vue'
import { createPinia } from 'pinia'
import { VueQueryPlugin } from '@tanstack/vue-query'
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import './styles/ide-theme.css' // 引入IDE主题样式
import App from './App.vue'
import router from './router'
import { useThemeStore } from './stores/themeStore'

// 创建 Vue 应用实例
const app = createApp(App)

// 配置 Pinia 状态管理
const pinia = createPinia()
app.use(pinia)

// 初始化主题
const themeStore = useThemeStore()
themeStore.initializeTheme()


// 配置 Vue Query
app.use(VueQueryPlugin)

// 配置 Element Plus UI 组件库
app.use(ElementPlus)

// 配置路由
app.use(router)

// 全局错误处理
app.config.errorHandler = (err, instance, info) => {
  console.error('全局错误:', err)
  console.error('组件实例:', instance)
  console.error('错误信息:', info)
}

// 挂载应用
app.mount('#app') 