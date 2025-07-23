<template>
  <div class="main-layout">
    <!-- Left Sidebar -->
    <ResizablePanel
      :initial-width="sidebarWidth"
      :min-width="200"
      :max-width="500"
      :collapsed="sidebarCollapsed"
      direction="right"
      @resize="onSidebarResize"
    >
      <CollapsibleSidebar
        :title="props.sidebarTitle"
        :initial-collapsed="sidebarCollapsed"
        storage-key="main-sidebar-collapsed"
        @collapse="onSidebarCollapse"
      >
        <template #title>
          <slot name="sidebar-title">{{ props.sidebarTitle }}</slot>
        </template>
        
        <slot name="sidebar" />
        
        <template #footer>
          <slot name="sidebar-footer" />
        </template>
      </CollapsibleSidebar>
    </ResizablePanel>

    <!-- Main Content Area -->
    <div class="main-content">
      <!-- Top Content Area -->
      <div class="content-top" :style="{ height: `calc(100% - ${bottomPanelHeight}px)` }">
        <TabbedInterface
          :tabs="props.mainTabs"
          :active-tab="props.activeMainTab"
          storage-key="main-tabs-active"
          @tab-select="onMainTabSelect"
          @tab-close="onMainTabClose"
          @tab-close-others="onMainTabCloseOthers"
          @tab-close-all="onMainTabCloseAll"
        >
          <template #actions>
            <slot name="main-tab-actions" />
          </template>
          
          <template v-for="tab in props.mainTabs" :key="tab.id" #[tab.id]>
            <slot :name="`main-tab-${tab.id}`" :tab="tab" />
          </template>
        </TabbedInterface>
      </div>

      <!-- Bottom Panel (Resizable) -->
      <div class="bottom-panel-container" v-if="!bottomPanelCollapsed">
        <div class="bottom-panel-resize-handle" @mousedown="startBottomResize" />
        <div class="bottom-panel" :style="{ height: `${bottomPanelHeight}px` }">
          <TabbedInterface
            :tabs="props.bottomTabs"
            :active-tab="props.activeBottomTab"
            storage-key="bottom-tabs-active"
            @tab-select="onBottomTabSelect"
            @tab-close="onBottomTabClose"
            @tab-close-others="onBottomTabCloseOthers"
            @tab-close-all="onBottomTabCloseAll"
          >
            <template #actions>
              <button
                class="panel-toggle-button"
                @click="toggleBottomPanel"
                title="Collapse bottom panel"
              >
                <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
                  <path d="M8 4l4 4H4l4-4z"/>
                </svg>
              </button>
            </template>
            
            <template v-for="tab in props.bottomTabs" :key="tab.id" #[tab.id]>
              <slot :name="`bottom-tab-${tab.id}`" :tab="tab" />
            </template>
          </TabbedInterface>
        </div>
      </div>

      <!-- Bottom Panel Collapsed State -->
      <div v-else class="bottom-panel-collapsed">
        <button
          class="panel-expand-button"
          @click="toggleBottomPanel"
          title="Expand bottom panel"
        >
          <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
            <path d="M4 8l4-4 4 4H4z"/>
          </svg>
          <span>{{ props.bottomTabs.length }} panel{{ props.bottomTabs.length !== 1 ? 's' : '' }}</span>
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import ResizablePanel from './ResizablePanel.vue'
import CollapsibleSidebar from './CollapsibleSidebar.vue'
import TabbedInterface, { type Tab } from './TabbedInterface.vue'

const props = withDefaults(defineProps<{
  sidebarTitle?: string
  mainTabs?: Tab[]
  bottomTabs?: Tab[]
  activeMainTab?: string
  activeBottomTab?: string
}>(), {
  sidebarTitle: 'Explorer',
  mainTabs: () => [],
  bottomTabs: () => [],
  activeMainTab: '',
  activeBottomTab: ''
})

const emit = defineEmits<{
  'sidebar-resize': [width: number]
  'sidebar-collapse': [collapsed: boolean]
  'main-tab-select': [tabId: string]
  'main-tab-close': [tabId: string]
  'main-tab-close-others': [tabId: string]
  'main-tab-close-all': []
  'bottom-tab-select': [tabId: string]
  'bottom-tab-close': [tabId: string]
  'bottom-tab-close-others': [tabId: string]
  'bottom-tab-close-all': []
  'bottom-panel-toggle': [collapsed: boolean]
}>()

// Sidebar state
const sidebarWidth = ref(300)
const sidebarCollapsed = ref(false)

// Bottom panel state
const bottomPanelHeight = ref(200)
const bottomPanelCollapsed = ref(false)
const isResizingBottom = ref(false)
const startY = ref(0)
const startHeight = ref(0)

// Load persisted state
onMounted(() => {
  const savedSidebarWidth = localStorage.getItem('main-layout-sidebar-width')
  if (savedSidebarWidth) {
    sidebarWidth.value = parseInt(savedSidebarWidth)
  }
  
  const savedBottomHeight = localStorage.getItem('main-layout-bottom-height')
  if (savedBottomHeight) {
    bottomPanelHeight.value = parseInt(savedBottomHeight)
  }
  
  const savedBottomCollapsed = localStorage.getItem('main-layout-bottom-collapsed')
  if (savedBottomCollapsed) {
    bottomPanelCollapsed.value = JSON.parse(savedBottomCollapsed)
  }
})

// Sidebar handlers
const onSidebarResize = (width: number) => {
  sidebarWidth.value = width
  localStorage.setItem('main-layout-sidebar-width', width.toString())
  emit('sidebar-resize', width)
}

const onSidebarCollapse = (collapsed: boolean) => {
  sidebarCollapsed.value = collapsed
  emit('sidebar-collapse', collapsed)
}

// Main tab handlers
const onMainTabSelect = (tabId: string) => {
  emit('main-tab-select', tabId)
}

const onMainTabClose = (tabId: string) => {
  emit('main-tab-close', tabId)
}

const onMainTabCloseOthers = (tabId: string) => {
  emit('main-tab-close-others', tabId)
}

const onMainTabCloseAll = () => {
  emit('main-tab-close-all')
}

// Bottom tab handlers
const onBottomTabSelect = (tabId: string) => {
  emit('bottom-tab-select', tabId)
}

const onBottomTabClose = (tabId: string) => {
  emit('bottom-tab-close', tabId)
}

const onBottomTabCloseOthers = (tabId: string) => {
  emit('bottom-tab-close-others', tabId)
}

const onBottomTabCloseAll = () => {
  emit('bottom-tab-close-all')
}

// Bottom panel resize handlers
const startBottomResize = (event: MouseEvent) => {
  isResizingBottom.value = true
  startY.value = event.clientY
  startHeight.value = bottomPanelHeight.value
  
  document.addEventListener('mousemove', handleBottomResize)
  document.addEventListener('mouseup', stopBottomResize)
  document.body.style.cursor = 'row-resize'
  document.body.style.userSelect = 'none'
}

const handleBottomResize = (event: MouseEvent) => {
  if (!isResizingBottom.value) return
  
  const deltaY = startY.value - event.clientY
  const newHeight = Math.max(100, Math.min(600, startHeight.value + deltaY))
  
  bottomPanelHeight.value = newHeight
  localStorage.setItem('main-layout-bottom-height', newHeight.toString())
}

const stopBottomResize = () => {
  isResizingBottom.value = false
  document.removeEventListener('mousemove', handleBottomResize)
  document.removeEventListener('mouseup', stopBottomResize)
  document.body.style.cursor = ''
  document.body.style.userSelect = ''
}

const toggleBottomPanel = () => {
  bottomPanelCollapsed.value = !bottomPanelCollapsed.value
  localStorage.setItem('main-layout-bottom-collapsed', JSON.stringify(bottomPanelCollapsed.value))
  emit('bottom-panel-toggle', bottomPanelCollapsed.value)
}

onUnmounted(() => {
  if (isResizingBottom.value) {
    stopBottomResize()
  }
})
</script>

<style scoped>
.main-layout {
  display: flex;
  height: 100vh;
  background: var(--geek-bg-primary);
  color: var(--geek-text-primary);
}

.main-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.content-top {
  flex: 1;
  overflow: hidden;
}

.bottom-panel-container {
  position: relative;
  border-top: 1px solid var(--geek-border);
}

.bottom-panel-resize-handle {
  position: absolute;
  top: -2px;
  left: 0;
  right: 0;
  height: 4px;
  cursor: row-resize;
  background: transparent;
  z-index: 10;
  transition: background-color 0.2s ease;
}

.bottom-panel-resize-handle:hover {
  background-color: var(--geek-accent-primary);
}

.bottom-panel {
  background: var(--geek-bg-secondary);
}

.bottom-panel-collapsed {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 32px;
  background: var(--geek-bg-secondary);
  border-top: 1px solid var(--geek-border);
}

.panel-toggle-button,
.panel-expand-button {
  display: flex;
  align-items: center;
  gap: 4px;
  background: none;
  border: none;
  color: var(--geek-text-secondary);
  cursor: pointer;
  padding: 4px 8px;
  border-radius: 4px;
  font-size: 12px;
  transition: all 0.2s ease;
}

.panel-toggle-button:hover,
.panel-expand-button:hover {
  background: var(--geek-bg-hover);
  color: var(--geek-text-primary);
}

.panel-expand-button {
  font-size: 11px;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}
</style>