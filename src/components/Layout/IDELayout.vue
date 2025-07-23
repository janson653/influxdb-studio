<template>
  <div class="ide-layout">
    <!-- 顶部工具栏 -->
    <div class="top-toolbar">
      <div class="project-selector">
        <div class="project-icon">DP</div>
        <span>demoProject</span>
        <span>▼</span>
      </div>
      
      <div class="tests-dropdown">
        <span>tests</span>
        <span>▼</span>
      </div>
      
      <div class="connection-status">
        <span class="status-connected">
          <span class="status-dot connected"></span>
          guest.public
        </span>
        <span>▼</span>
      </div>
    </div>

    <!-- 主工具栏 -->
    <div class="main-toolbar">
      <div class="toolbar-tabs">
        <button 
          class="toolbar-tab" 
          :class="{ active: activeToolbarTab === 'console' }"
          @click="setActiveToolbarTab('console')"
        >
          console
        </button>
        <button 
          class="toolbar-tab" 
          :class="{ active: activeToolbarTab === 'actor' }"
          @click="setActiveToolbarTab('actor')"
        >
          actor
        </button>
      </div>
      
      <div class="toolbar-actions">
        <button class="toolbar-btn primary">
          <span class="icon icon-play"></span>
        </button>
        <button class="toolbar-btn">
          <span class="icon icon-history"></span>
        </button>
        <button class="toolbar-btn">
          <span class="icon icon-settings"></span>
        </button>
        <div class="toolbar-divider"></div>
        <select class="toolbar-select">
          <option>Tx: Auto</option>
        </select>
        <select class="toolbar-select">
          <option>Playground</option>
        </select>
      </div>
    </div>

    <!-- 主容器 -->
    <div class="main-container">
      <!-- 左侧数据库浏览器 -->
      <div class="database-explorer">
        <div class="explorer-header">
          <span>Database Explorer</span>
        </div>
        <div class="explorer-tree">
          <slot name="database-tree">
            <!-- 默认的数据库树结构 -->
            <div 
              class="tree-item" 
              :class="{ expanded: expandedTreeItems.has('postgresql') }"
              @click="toggleTreeItem('postgresql')"
            >
              <span class="tree-expand" :class="{ expanded: expandedTreeItems.has('postgresql') }">▶</span>
              <span class="tree-icon icon-database"></span>
              <span>PostgreSQL</span>
              <span class="tree-count">(2 of 5)</span>
            </div>
            <div class="tree-children" v-show="expandedTreeItems.has('postgresql')">
              <div 
                class="tree-item" 
                :class="{ expanded: expandedTreeItems.has('guest') }"
                @click="toggleTreeItem('guest')"
              >
                <span class="tree-expand" :class="{ expanded: expandedTreeItems.has('guest') }">▶</span>
                <span class="tree-icon icon-user"></span>
                <span>guest</span>
              </div>
              <div class="tree-children" v-show="expandedTreeItems.has('guest')">
                <div 
                  class="tree-item" 
                  :class="{ expanded: expandedTreeItems.has('public') }"
                  @click="toggleTreeItem('public')"
                >
                  <span class="tree-expand" :class="{ expanded: expandedTreeItems.has('public') }">▶</span>
                  <span class="tree-icon icon-schema"></span>
                  <span>public</span>
                </div>
                <div class="tree-children" v-show="expandedTreeItems.has('public')">
                  <div class="tree-item">
                    <span class="tree-icon icon-table"></span>
                    <span>tables 51</span>
                  </div>
                  <div 
                    class="tree-item" 
                    :class="{ expanded: expandedTreeItems.has('views') }"
                    @click="toggleTreeItem('views')"
                  >
                    <span class="tree-expand" :class="{ expanded: expandedTreeItems.has('views') }">▶</span>
                    <span class="tree-icon icon-view"></span>
                    <span>views 8</span>
                  </div>
                  <div class="tree-children" v-show="expandedTreeItems.has('views')">
                    <div class="tree-item">
                      <span class="tree-icon icon-file"></span>
                      <span>actor_info</span>
                    </div>
                    <div class="tree-item">
                      <span class="tree-icon icon-file"></span>
                      <span>customer_list</span>
                    </div>
                    <div class="tree-item">
                      <span class="tree-icon icon-file"></span>
                      <span>film_list</span>
                    </div>
                    <div class="tree-item">
                      <span class="tree-icon icon-file"></span>
                      <span>films_actors</span>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </slot>
        </div>
      </div>

      <!-- 中央内容区域 -->
      <div class="content-area">
        <slot name="content">
          <!-- 默认内容区域 -->
          <div class="sql-editor-container">
            <div class="editor-content">
              <div class="editor-lines">
                <div class="line-number">1</div>
                <div class="line-number">2</div>
                <div class="line-number">3</div>
                <div class="line-number">4</div>
                <div class="line-number">5</div>
                <div class="line-number">6</div>
                <div class="line-number">7</div>
                <div class="line-number">8</div>
                <div class="line-number">9</div>
              </div>
              <div class="editor-code">
                <div class="code-line">
                  <span class="sql-keyword">SELECT</span> 
                  <span class="sql-field">f.title</span>, 
                  <span class="sql-field">c.name</span>, 
                  <span class="sql-field">a.first_name</span>, 
                  <span class="sql-field">a.last_name</span>
                </div>
                <div class="code-line">
                  <span class="sql-keyword">FROM</span> 
                  <span class="sql-table">actor</span>
                </div>
                <div class="code-line">
                  <span class="sql-keyword">JOIN</span> 
                  <span class="sql-table">film_actor</span> 
                  <span class="sql-alias">fa</span> 
                  <span class="sql-keyword">ON</span> 
                  <span class="sql-table">actor</span>.<span class="sql-field">actor_id</span> = 
                  <span class="sql-alias">a</span>.<span class="sql-field">actor_id</span>
                </div>
                <div class="code-line">
                  <span class="sql-keyword">JOIN</span> 
                  <span class="sql-table">film</span> 
                  <span class="sql-alias">f</span> 
                  <span class="sql-keyword">ON</span> 
                  <span class="sql-alias">fa</span>.<span class="sql-field">film_id</span> = 
                  <span class="sql-alias">f</span>.<span class="sql-field">film_id</span>
                </div>
                <div class="code-line">
                  <span class="sql-keyword">JOIN</span> 
                  <span class="sql-table">film_category</span> 
                  <span class="sql-alias">fc</span> 
                  <span class="sql-keyword">ON</span> 
                  <span class="sql-alias">f</span>.<span class="sql-field">film_id</span> = 
                  <span class="sql-alias">fc</span>.<span class="sql-field">film_id</span>
                </div>
                <div class="code-line">
                  <span class="sql-keyword">JOIN</span> 
                  <span class="sql-table">category</span> 
                  <span class="sql-alias">c</span> 
                  <span class="sql-keyword">ON</span> 
                  <span class="sql-alias">c</span>.<span class="sql-field">category_id</span> = 
                  <span class="sql-alias">fc</span>.<span class="sql-field">category_id</span>
                </div>
                <div class="code-line">
                  <span class="sql-keyword">ORDER BY</span> 
                  <span class="sql-alias">f</span>.<span class="sql-field">title</span>;
                </div>
              </div>
            </div>
          </div>

          <!-- 结果区域 -->
          <div class="results-container">
            <div class="results-tabs">
              <button class="results-tab">
                Output
              </button>
              <button class="results-tab active">
                <span class="icon icon-table"></span>
                Result 1
              </button>
            </div>
            
            <div class="results-toolbar">
              <div class="results-info">1-500 of 501+</div>
              <div class="results-actions">
                <button class="results-btn">⟲</button>
                <button class="results-btn">📄</button>
              </div>
            </div>
            
            <div class="results-table">
              <slot name="results-table">
                <!-- 默认结果表格 -->
                <table>
                  <thead>
                    <tr>
                      <th>
                        title
                        <span class="sort-icon">↕</span>
                      </th>
                      <th>
                        name
                        <span class="sort-icon">↕</span>
                      </th>
                      <th>
                        first_name
                        <span class="sort-icon">↕</span>
                      </th>
                      <th>
                        last_name
                        <span class="sort-icon">↕</span>
                      </th>
                    </tr>
                  </thead>
                  <tbody>
                    <tr>
                      <td>ACADEMY DINOSAUR</td>
                      <td>Documentary</td>
                      <td>ROCK</td>
                      <td>DUKAKIS</td>
                    </tr>
                    <tr>
                      <td>ACADEMY DINOSAUR</td>
                      <td>Documentary</td>
                      <td>MARY</td>
                      <td>KEITEL</td>
                    </tr>
                    <tr>
                      <td>ACADEMY DINOSAUR</td>
                      <td>Documentary</td>
                      <td>JOHNNY</td>
                      <td>CAGE</td>
                    </tr>
                    <tr>
                      <td>ACADEMY DINOSAUR</td>
                      <td>Documentary</td>
                      <td>PENELOPE</td>
                      <td>GUINESS</td>
                    </tr>
                    <tr>
                      <td>ACADEMY DINOSAUR</td>
                      <td>Documentary</td>
                      <td>SANDRA</td>
                      <td>KILMER</td>
                    </tr>
                  </tbody>
                </table>
              </slot>
            </div>
          </div>
        </slot>
      </div>
    </div>

    <!-- 底部状态栏 -->
    <div class="status-bar">
      <div class="status-item">
        <span>Database Consoles > PostgreSQL > console</span>
      </div>
      <div class="status-item">
        <span>10:1</span>
      </div>
      <div class="status-item">
        <span>LF</span>
      </div>
      <div class="status-item">
        <span>UTF-8</span>
      </div>
      <div class="status-item">
        <span>4 spaces</span>
      </div>
      <div class="status-item">
        <span>tests</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useConnectionStore } from '../../stores/connectionStore'
import { useDatabaseStore } from '../../stores/databaseStore'

// Stores
const connectionStore = useConnectionStore()
const databaseStore = useDatabaseStore()

// 响应式状态
const activeToolbarTab = ref('console')
const expandedTreeItems = ref(new Set(['postgresql', 'guest', 'public', 'views']))
const showConnectionManager = ref(false)
const showSettings = ref(false)

// 方法
const toggleTreeItem = (itemId: string) => {
  if (expandedTreeItems.value.has(itemId)) {
    expandedTreeItems.value.delete(itemId)
  } else {
    expandedTreeItems.value.add(itemId)
  }
}

const setActiveToolbarTab = (tabId: string) => {
  activeToolbarTab.value = tabId
}

const refreshDatabases = async () => {
  if (connectionStore.isConnected) {
    try {
      await databaseStore.fetchDatabases()
    } catch (error) {
      console.error('刷新数据库失败:', error)
    }
  }
}

// 生命周期
onMounted(() => {
  // 初始化
})

// 暴露方法给父组件
defineExpose({
  activeToolbarTab,
  expandedTreeItems,
  toggleTreeItem,
  setActiveToolbarTab
})
</script>

<style scoped>
/* IDE布局样式 */
.ide-layout {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  background-color: var(--ide-bg-primary);
  color: var(--ide-text-primary);
  height: 100vh;
  overflow: hidden;
}

/* 顶部工具栏 */
.top-toolbar {
  height: 30px;
  background-color: var(--ide-bg-secondary);
  border-bottom: 1px solid var(--ide-border);
  display: flex;
  align-items: center;
  padding: 0 10px;
  gap: 15px;
}

.project-selector {
  display: flex;
  align-items: center;
  gap: 5px;
  padding: 3px 8px;
  background-color: var(--ide-bg-tertiary);
  border-radius: 3px;
  cursor: pointer;
  transition: all 0.2s ease;
  border: 1px solid var(--ide-border);
}

.project-selector:hover {
  background-color: var(--ide-bg-hover);
}

.project-icon {
  width: 14px;
  height: 14px;
  background-color: var(--ide-accent-primary);
  border-radius: 2px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  font-size: 9px;
  font-weight: bold;
}

.tests-dropdown {
  display: flex;
  align-items: center;
  gap: 5px;
  padding: 3px 8px;
  background-color: var(--ide-bg-tertiary);
  border-radius: 3px;
  cursor: pointer;
  transition: all 0.2s ease;
  border: 1px solid var(--ide-border);
  font-size: 12px;
}

.tests-dropdown:hover {
  background-color: var(--ide-bg-hover);
}

.connection-status {
  display: flex;
  align-items: center;
  gap: 5px;
  font-size: 12px;
  margin-left: auto;
}

.status-connected {
  color: var(--ide-accent-success);
  display: flex;
  align-items: center;
  gap: 5px;
}

.status-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
}

.status-dot.connected {
  background-color: var(--ide-accent-success);
}

/* 主工具栏 */
.main-toolbar {
  height: 35px;
  background-color: var(--ide-bg-secondary);
  border-bottom: 1px solid var(--ide-border);
  display: flex;
  align-items: center;
  padding: 0 10px;
}

.toolbar-tabs {
  display: flex;
  align-items: center;
  height: 100%;
}

.toolbar-tab {
  padding: 8px 15px;
  background-color: transparent;
  border: none;
  color: var(--ide-text-secondary);
  cursor: pointer;
  border-right: 1px solid var(--ide-border);
  font-size: 13px;
  height: 100%;
  display: flex;
  align-items: center;
  transition: all 0.2s ease;
}

.toolbar-tab.active {
  background-color: var(--ide-bg-primary);
  border-bottom: 2px solid var(--ide-accent-primary);
  color: var(--ide-text-primary);
}

.toolbar-tab:hover:not(.active) {
  background-color: var(--ide-bg-hover);
}

.toolbar-actions {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-left: auto;
}

.toolbar-btn {
  padding: 5px 8px;
  background-color: transparent;
  border: none;
  color: var(--ide-text-primary);
  cursor: pointer;
  border-radius: 3px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s ease;
  font-size: 12px;
}

.toolbar-btn:hover {
  background-color: var(--ide-bg-hover);
}

.toolbar-btn.primary {
  background-color: var(--ide-accent-primary);
  color: white;
}

.toolbar-btn.primary:hover {
  background-color: var(--ide-accent-secondary);
}

.toolbar-divider {
  width: 1px;
  height: 16px;
  background-color: var(--ide-border);
}

.toolbar-select {
  background-color: var(--ide-bg-tertiary);
  border: 1px solid var(--ide-border);
  color: var(--ide-text-primary);
  padding: 3px 6px;
  border-radius: 3px;
  font-size: 11px;
  cursor: pointer;
}

.toolbar-select:focus {
  outline: none;
  border-color: var(--ide-accent-primary);
}

/* 主容器 */
.main-container {
  display: flex;
  height: calc(100vh - 100px); /* 减去顶部工具栏、主工具栏和底部状态栏的高度 */
}

/* 左侧数据库浏览器 */
.database-explorer {
  width: 280px;
  background-color: var(--ide-bg-secondary);
  border-right: 1px solid var(--ide-border);
  display: flex;
  flex-direction: column;
}

.explorer-header {
  padding: 10px;
  border-bottom: 1px solid var(--ide-border);
  font-weight: bold;
  font-size: 13px;
  color: var(--ide-text-primary);
}

.explorer-tree {
  flex: 1;
  overflow-y: auto;
  padding: 5px;
}

.tree-item {
  padding: 3px 5px;
  cursor: pointer;
  border-radius: 3px;
  display: flex;
  align-items: center;
  gap: 5px;
  font-size: 13px;
  transition: background-color 0.2s ease;
}

.tree-item:hover {
  background-color: var(--ide-bg-hover);
}

.tree-expand {
  width: 12px;
  height: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 8px;
  color: var(--ide-text-secondary);
  transition: transform 0.2s ease;
}

.tree-expand.expanded {
  transform: rotate(90deg);
}

.tree-children {
  margin-left: 15px;
}

.tree-icon {
  width: 16px;
  height: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 2px;
}

.tree-count {
  font-size: 11px;
  color: var(--ide-text-secondary);
  margin-left: auto;
}

/* 中央内容区域 */
.content-area {
  flex: 1;
  display: flex;
  flex-direction: column;
}

/* SQL编辑器区域 */
.sql-editor-container {
  flex: 1;
  display: flex;
  background-color: var(--ide-bg-primary);
  position: relative;
}

.editor-content {
  display: flex;
  width: 100%;
  height: 100%;
}

.editor-lines {
  width: 50px;
  background-color: var(--ide-bg-secondary);
  border-right: 1px solid var(--ide-border);
  padding: 10px 5px;
  text-align: right;
  font-family: 'JetBrains Mono', 'Fira Code', monospace;
  font-size: 12px;
  color: var(--ide-text-secondary);
  user-select: none;
}

.line-number {
  height: 18px;
  line-height: 18px;
  padding-right: 8px;
}

.editor-code {
  flex: 1;
  padding: 10px;
  font-family: 'JetBrains Mono', 'Fira Code', monospace;
  font-size: 14px;
  line-height: 18px;
  color: var(--ide-text-primary);
  overflow-y: auto;
}

.code-line {
  height: 18px;
  line-height: 18px;
  white-space: pre;
}

.sql-keyword {
  color: var(--ide-accent-purple);
  font-weight: bold;
}

.sql-field {
  color: var(--ide-accent-success);
}

.sql-table {
  color: var(--ide-accent-orange);
}

.sql-alias {
  color: var(--ide-accent-info);
}

/* 结果区域 */
.results-container {
  height: 300px;
  background-color: var(--ide-bg-primary);
  border-top: 1px solid var(--ide-border);
  display: flex;
  flex-direction: column;
}

.results-tabs {
  height: 30px;
  background-color: var(--ide-bg-secondary);
  border-bottom: 1px solid var(--ide-border);
  display: flex;
  align-items: center;
}

.results-tab {
  padding: 5px 15px;
  background-color: transparent;
  border: none;
  color: var(--ide-text-secondary);
  cursor: pointer;
  border-right: 1px solid var(--ide-border);
  font-size: 12px;
  display: flex;
  align-items: center;
  gap: 5px;
  transition: all 0.2s ease;
}

.results-tab.active {
  background-color: var(--ide-bg-primary);
  color: var(--ide-text-primary);
}

.results-tab:hover:not(.active) {
  background-color: var(--ide-bg-hover);
}

.results-toolbar {
  height: 30px;
  background-color: var(--ide-bg-secondary);
  border-bottom: 1px solid var(--ide-border);
  display: flex;
  align-items: center;
  padding: 0 10px;
  gap: 10px;
}

.results-info {
  font-size: 12px;
  color: var(--ide-text-secondary);
}

.results-actions {
  display: flex;
  gap: 5px;
  margin-left: auto;
}

.results-btn {
  padding: 3px 8px;
  background-color: transparent;
  border: none;
  color: var(--ide-text-primary);
  cursor: pointer;
  border-radius: 3px;
  font-size: 12px;
  transition: background-color 0.2s ease;
}

.results-btn:hover {
  background-color: var(--ide-bg-hover);
}

.results-table {
  flex: 1;
  overflow: auto;
  background-color: var(--ide-bg-primary);
}

.results-table table {
  width: 100%;
  border-collapse: collapse;
  font-size: 12px;
}

.results-table th {
  background-color: var(--ide-bg-secondary);
  padding: 8px;
  text-align: left;
  border-bottom: 1px solid var(--ide-border);
  font-weight: normal;
  position: sticky;
  top: 0;
  color: var(--ide-text-primary);
}

.sort-icon {
  font-size: 10px;
  color: var(--ide-text-secondary);
  cursor: pointer;
}

.results-table td {
  padding: 6px 8px;
  border-bottom: 1px solid var(--ide-border-light);
  color: var(--ide-text-primary);
}

.results-table tr:hover {
  background-color: var(--ide-bg-hover);
}

/* 底部状态栏 */
.status-bar {
  height: 25px;
  background-color: var(--ide-bg-secondary);
  border-top: 1px solid var(--ide-border);
  display: flex;
  align-items: center;
  padding: 0 10px;
  font-size: 12px;
  color: var(--ide-text-secondary);
}

.status-item {
  margin-right: 20px;
  display: flex;
  align-items: center;
  gap: 5px;
}

.status-item:last-child {
  margin-left: auto;
  margin-right: 0;
}

/* 图标样式 */
.icon {
  width: 16px;
  height: 16px;
  display: inline-block;
  background-size: contain;
  background-repeat: no-repeat;
  background-position: center;
  border-radius: 2px;
}

.icon-database { background-color: var(--ide-accent-primary); }
.icon-play { background-color: var(--ide-accent-success); }
.icon-settings { background-color: var(--ide-accent-purple); }
.icon-refresh { background-color: var(--ide-accent-info); }
.icon-history { background-color: var(--ide-accent-info); }
.icon-table { background-color: var(--ide-accent-orange); }
.icon-view { background-color: var(--ide-accent-success); }
.icon-folder { background-color: var(--ide-accent-orange); }
.icon-file { background-color: var(--ide-accent-success); }
.icon-user { background-color: var(--ide-accent-primary); }
.icon-schema { background-color: var(--ide-accent-orange); }
</style> 