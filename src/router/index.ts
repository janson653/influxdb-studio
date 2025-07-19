import { createRouter, createWebHashHistory } from 'vue-router'
import type { RouteRecordRaw } from 'vue-router'
import MainLayout from '../components/Layout/MainLayout.vue'

// 路由配置
const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'Layout',
    component: MainLayout,
    // 子路由现在用于在 MainLayout 的 <router-view> 中显示非核心页面
    children: [
      {
        path: 'settings',
        name: 'Settings',
        component: () => import('../views/Settings.vue'),
        meta: { title: '设置' }
      },
      // 主页，由 MainLayout 内部管理
      {
        path: '',
        name: 'Home',
        component: { template: '<div></div>' } 
      }
    ]
  }
]

// 创建路由实例 - 使用 hash 模式以兼容 Tauri
const router = createRouter({
  history: createWebHashHistory(),
  routes
})

// 路由守卫
router.beforeEach((to, _from, next) => {
  // 设置页面标题
  if (to.meta.title) {
    document.title = `${to.meta.title} - InfluxDB Studio`
  } else {
    document.title = 'InfluxDB Studio'
  }
  next()
})

// 路由错误处理
router.onError((error) => {
  console.error('路由错误:', error)
})

export default router 