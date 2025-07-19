import { createRouter, createWebHashHistory } from 'vue-router'
import type { RouteRecordRaw } from 'vue-router'

// 路由配置
const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'Layout',
    component: () => import('../components/Layout/MainLayout.vue'),
    redirect: '/home',
    children: [
      {
        path: 'home',
        name: 'Home',
        component: () => import('../views/Home.vue'),
        meta: { title: '首页' }
      },
      {
        path: 'connection',
        name: 'ConnectionManager',
        component: () => import('../views/ConnectionManager.vue'),
        meta: { title: '连接管理' }
      },
      {
        path: 'database',
        name: 'DatabaseExplorer',
        component: () => import('../views/DatabaseExplorer.vue'),
        meta: { title: '数据库浏览器' }
      },
      {
        path: 'query',
        name: 'QueryEditor',
        component: () => import('../views/QueryEditor.vue'),
        meta: { title: '查询编辑器' }
      },
      {
        path: 'settings',
        name: 'Settings',
        component: () => import('../views/Settings.vue'),
        meta: { title: '设置' }
      },
      {
        path: 'monaco-test',
        name: 'MonacoTest',
        component: () => import('../views/MonacoTest.vue'),
        meta: { title: 'Monaco Editor 测试' }
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
  }
  next()
})

// 路由错误处理
router.onError((error) => {
  console.error('路由错误:', error)
})

// 处理路由导航失败
router.afterEach((_to, _from, failure) => {
  if (failure) {
    console.error('路由导航失败:', failure)
  }
})

export default router 