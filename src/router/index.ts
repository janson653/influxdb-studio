import { createRouter, createWebHistory } from 'vue-router'
import type { RouteRecordRaw } from 'vue-router'

// 路由配置
const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'Home',
    component: () => import('../views/Home.vue'),
    meta: { title: '首页' }
  },
  {
    path: '/connection',
    name: 'ConnectionManager',
    component: () => import('../views/ConnectionManager.vue'),
    meta: { title: '连接管理' }
  },
  {
    path: '/database',
    name: 'DatabaseExplorer',
    component: () => import('../views/DatabaseExplorer.vue'),
    meta: { title: '数据库浏览器' }
  },
  {
    path: '/query',
    name: 'QueryEditor',
    component: () => import('../views/QueryEditor.vue'),
    meta: { title: '查询编辑器' }
  },
  {
    path: '/settings',
    name: 'Settings',
    component: () => import('../views/Settings.vue'),
    meta: { title: '设置' }
  }
]

// 创建路由实例
const router = createRouter({
  history: createWebHistory(),
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

export default router 