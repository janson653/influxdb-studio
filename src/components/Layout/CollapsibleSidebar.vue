<template>
  <div class="collapsible-sidebar" :class="{ 'is-collapsed': collapsed }">
    <div class="sidebar-header">
      <div class="sidebar-title" v-if="!collapsed">
        <slot name="title">{{ title }}</slot>
      </div>
      <button
        class="collapse-button"
        @click="toggleCollapse"
        :title="collapsed ? 'Expand sidebar' : 'Collapse sidebar'"
      >
        <svg
          class="collapse-icon"
          :class="{ 'rotated': collapsed }"
          width="16"
          height="16"
          viewBox="0 0 16 16"
          fill="currentColor"
        >
          <path d="M6 4l4 4-4 4V4z" />
        </svg>
      </button>
    </div>
    
    <div class="sidebar-content" v-if="!collapsed">
      <slot />
    </div>
    
    <div class="sidebar-footer" v-if="!collapsed && $slots.footer">
      <slot name="footer" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'

interface Props {
  title?: string
  initialCollapsed?: boolean
  persistState?: boolean
  storageKey?: string
}

const props = withDefaults(defineProps<Props>(), {
  title: '',
  initialCollapsed: false,
  persistState: true,
  storageKey: 'sidebar-collapsed'
})

const emit = defineEmits<{
  collapse: [collapsed: boolean]
}>()

// Initialize collapsed state from localStorage if persistence is enabled
const getInitialState = () => {
  if (props.persistState && props.storageKey) {
    const stored = localStorage.getItem(props.storageKey)
    return stored ? JSON.parse(stored) : props.initialCollapsed
  }
  return props.initialCollapsed
}

const collapsed = ref(getInitialState())

const toggleCollapse = () => {
  collapsed.value = !collapsed.value
  emit('collapse', collapsed.value)
}

// Persist state changes to localStorage
watch(collapsed, (newValue) => {
  if (props.persistState && props.storageKey) {
    localStorage.setItem(props.storageKey, JSON.stringify(newValue))
  }
}, { immediate: true })
</script>

<style scoped>
.collapsible-sidebar {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--geek-bg-secondary);
  border-right: 1px solid var(--geek-border);
  transition: all 0.2s ease;
  min-width: 40px;
}

.is-collapsed {
  width: 40px !important;
  min-width: 40px;
}

.sidebar-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 12px;
  border-bottom: 1px solid var(--geek-border);
  background: var(--geek-bg-tertiary);
  min-height: 40px;
}

.sidebar-title {
  font-size: 13px;
  font-weight: 600;
  color: var(--geek-text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  flex: 1;
}

.collapse-button {
  background: none;
  border: none;
  color: var(--geek-text-secondary);
  cursor: pointer;
  padding: 4px;
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s ease;
  flex-shrink: 0;
}

.collapse-button:hover {
  background: var(--geek-bg-hover);
  color: var(--geek-text-primary);
}

.collapse-icon {
  transition: transform 0.2s ease;
}

.collapse-icon.rotated {
  transform: rotate(180deg);
}

.sidebar-content {
  flex: 1;
  overflow: auto;
  padding: 8px 0;
}

.sidebar-footer {
  border-top: 1px solid var(--geek-border);
  padding: 8px 12px;
  background: var(--geek-bg-tertiary);
}

/* Custom scrollbar for sidebar content */
.sidebar-content::-webkit-scrollbar {
  width: 6px;
}

.sidebar-content::-webkit-scrollbar-track {
  background: var(--geek-bg-secondary);
}

.sidebar-content::-webkit-scrollbar-thumb {
  background: var(--geek-border);
  border-radius: 3px;
}

.sidebar-content::-webkit-scrollbar-thumb:hover {
  background: var(--geek-text-secondary);
}
</style>