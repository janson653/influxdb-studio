import { createApp } from 'vue'
import { createPinia } from 'pinia'
import { VueQueryPlugin } from '@tanstack/vue-query'
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import App from './App.vue'
import router from './router'

// 创建 Vue 应用实例
const app = createApp(App)

// 配置 Pinia 状态管理
const pinia = createPinia()
app.use(pinia)

// 配置 Vue Query
app.use(VueQueryPlugin)

// 配置 Element Plus UI 组件库
app.use(ElementPlus)

// 配置路由
app.use(router)

// 挂载应用
app.mount('#app') 