<template>
  <div class="tabbed-interface">
    <div class="tab-bar">
      <div class="tab-list">
        <button
          v-for="tab in tabs"
          :key="tab.id"
          class="tab-button"
          :class="{ 'active': activeTab === tab.id, 'closable': tab.closable }"
          @click="selectTab(tab.id)"
          @contextmenu.prevent="showTabContextMenu($event, tab)"
        >
          <span class="tab-icon" v-if="tab.icon">
            <component :is="tab.icon" />
          </span>
          <span class="tab-title">{{ tab.title }}</span>
          <button
            v-if="tab.closable"
            class="tab-close"
            @click.stop="closeTab(tab.id)"
            :title="`Close ${tab.title}`"
          >
            <svg width="12" height="12" viewBox="0 0 12 12" fill="currentColor">
              <path d="M9.5 3.5L8.5 2.5 6 5 3.5 2.5 2.5 3.5 5 6 2.5 8.5 3.5 9.5 6 7 8.5 9.5 9.5 8.5 7 6z"/>
            </svg>
          </button>
        </button>
      </div>
      
      <div class="tab-actions" v-if="$slots.actions">
        <slot name="actions" />
      </div>
    </div>
    
    <div class="tab-content">
      <div
        v-for="tab in tabs"
        :key="tab.id"
        v-show="activeTab === tab.id"
        class="tab-panel"
        :class="`tab-panel-${tab.id}`"
      >
        <slot :name="tab.id" :tab="tab" />
      </div>
    </div>
    
    <!-- Context menu for tabs -->
    <div
      v-if="contextMenu.visible"
      class="tab-context-menu"
      :style="{ left: contextMenu.x + 'px', top: contextMenu.y + 'px' }"
      @click="hideContextMenu"
    >
      <button
        v-if="contextMenu.tab?.closable"
        class="context-menu-item"
        @click="closeTab(contextMenu.tab.id)"
      >
        Close Tab
      </button>
      <button
        class="context-menu-item"
        @click="closeOtherTabs(contextMenu.tab?.id)"
      >
        Close Other Tabs
      </button>
      <button
        class="context-menu-item"
        @click="closeAllTabs"
      >
        Close All Tabs
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'

export interface Tab {
  id: string
  title: string
  icon?: any
  closable?: boolean
  data?: any
}

interface Props {
  tabs: Tab[]
  activeTab?: string
  persistActiveTab?: boolean
  storageKey?: string
}

const props = withDefaults(defineProps<Props>(), {
  activeTab: '',
  persistActiveTab: true,
  storageKey: 'active-tab'
})

const emit = defineEmits<{
  'tab-select': [tabId: string]
  'tab-close': [tabId: string]
  'tab-close-others': [tabId: string]
  'tab-close-all': []
}>()

// Context menu state
const contextMenu = ref({
  visible: false,
  x: 0,
  y: 0,
  tab: null as Tab | null
})

// Get initial active tab
const getInitialActiveTab = () => {
  if (props.persistActiveTab && props.storageKey) {
    const stored = localStorage.getItem(props.storageKey)
    if (stored && props.tabs.some(tab => tab.id === stored)) {
      return stored
    }
  }
  return props.activeTab || (props.tabs.length > 0 ? props.tabs[0].id : '')
}

const activeTab = ref(getInitialActiveTab())

const selectTab = (tabId: string) => {
  activeTab.value = tabId
  emit('tab-select', tabId)
  
  if (props.persistActiveTab && props.storageKey) {
    localStorage.setItem(props.storageKey, tabId)
  }
}

const closeTab = (tabId: string) => {
  emit('tab-close', tabId)
  
  // If closing the active tab, select another one
  if (activeTab.value === tabId) {
    const currentIndex = props.tabs.findIndex(tab => tab.id === tabId)
    const nextTab = props.tabs[currentIndex + 1] || props.tabs[currentIndex - 1]
    if (nextTab) {
      selectTab(nextTab.id)
    }
  }
}

const closeOtherTabs = (keepTabId?: string) => {
  emit('tab-close-others', keepTabId || activeTab.value)
}

const closeAllTabs = () => {
  emit('tab-close-all')
}

const showTabContextMenu = (event: MouseEvent, tab: Tab) => {
  contextMenu.value = {
    visible: true,
    x: event.clientX,
    y: event.clientY,
    tab
  }
}

const hideContextMenu = () => {
  contextMenu.value.visible = false
}

// Handle clicks outside context menu
const handleClickOutside = (event: MouseEvent) => {
  if (contextMenu.value.visible) {
    const menu = document.querySelector('.tab-context-menu')
    if (menu && !menu.contains(event.target as Node)) {
      hideContextMenu()
    }
  }
}

onMounted(() => {
  document.addEventListener('click', handleClickOutside)
  
  // Set initial active tab if none is set
  if (!activeTab.value && props.tabs.length > 0) {
    selectTab(props.tabs[0].id)
  }
})

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside)
})
</script>

<style scoped>
.tabbed-interface {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--geek-bg-primary);
}

.tab-bar {
  display: flex;
  align-items: center;
  background: var(--geek-bg-secondary);
  border-bottom: 1px solid var(--geek-border);
  min-height: 36px;
}

.tab-list {
  display: flex;
  flex: 1;
  overflow-x: auto;
  scrollbar-width: none;
  -ms-overflow-style: none;
}

.tab-list::-webkit-scrollbar {
  display: none;
}

.tab-button {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 12px;
  background: none;
  border: none;
  color: var(--geek-text-secondary);
  cursor: pointer;
  font-size: 13px;
  white-space: nowrap;
  border-right: 1px solid var(--geek-border);
  transition: all 0.2s ease;
  position: relative;
}

.tab-button:hover {
  background: var(--geek-bg-hover);
  color: var(--geek-text-primary);
}

.tab-button.active {
  background: var(--geek-bg-primary);
  color: var(--geek-text-primary);
  border-bottom: 2px solid var(--geek-accent-primary);
}

.tab-button.active::after {
  content: '';
  position: absolute;
  bottom: -1px;
  left: 0;
  right: 0;
  height: 1px;
  background: var(--geek-bg-primary);
}

.tab-icon {
  display: flex;
  align-items: center;
  font-size: 14px;
}

.tab-title {
  max-width: 150px;
  overflow: hidden;
  text-overflow: ellipsis;
}

.tab-close {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 16px;
  height: 16px;
  background: none;
  border: none;
  color: var(--geek-text-secondary);
  cursor: pointer;
  border-radius: 2px;
  transition: all 0.2s ease;
  opacity: 0;
}

.tab-button:hover .tab-close,
.tab-button.active .tab-close {
  opacity: 1;
}

.tab-close:hover {
  background: var(--geek-bg-hover);
  color: var(--geek-text-primary);
}

.tab-actions {
  display: flex;
  align-items: center;
  padding: 0 8px;
  border-left: 1px solid var(--geek-border);
}

.tab-content {
  flex: 1;
  overflow: hidden;
  position: relative;
}

.tab-panel {
  height: 100%;
  overflow: auto;
}

.tab-context-menu {
  position: fixed;
  background: var(--geek-bg-secondary);
  border: 1px solid var(--geek-border);
  border-radius: 4px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
  z-index: 1000;
  min-width: 150px;
  padding: 4px 0;
}

.context-menu-item {
  display: block;
  width: 100%;
  padding: 8px 12px;
  background: none;
  border: none;
  color: var(--geek-text-primary);
  cursor: pointer;
  font-size: 13px;
  text-align: left;
  transition: background-color 0.2s ease;
}

.context-menu-item:hover {
  background: var(--geek-bg-hover);
}
</style>