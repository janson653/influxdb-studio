<template>
  <div class="resizable-panel" :class="{ 'is-collapsed': collapsed }">
    <div class="panel-content" :style="{ width: collapsed ? '0' : `${width}px` }">
      <slot />
    </div>
    <div
      v-if="resizable && !collapsed"
      class="resize-handle"
      :class="`resize-handle-${direction}`"
      @mousedown="startResize"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'

interface Props {
  initialWidth?: number
  minWidth?: number
  maxWidth?: number
  resizable?: boolean
  collapsed?: boolean
  direction?: 'left' | 'right' | 'top' | 'bottom'
}

const props = withDefaults(defineProps<Props>(), {
  initialWidth: 350,
  minWidth: 250,
  maxWidth: 800,
  resizable: true,
  collapsed: false,
  direction: 'right'
})

const emit = defineEmits<{
  resize: [width: number]
  collapse: [collapsed: boolean]
}>()

const width = ref(props.initialWidth)
const isResizing = ref(false)
const startX = ref(0)
const startWidth = ref(0)

const actualWidth = computed(() => props.collapsed ? 0 : width.value)

const startResize = (event: MouseEvent) => {
  isResizing.value = true
  startX.value = event.clientX
  startWidth.value = width.value
  
  document.addEventListener('mousemove', handleResize)
  document.addEventListener('mouseup', stopResize)
  document.body.style.cursor = 'col-resize'
  document.body.style.userSelect = 'none'
}

const handleResize = (event: MouseEvent) => {
  if (!isResizing.value) return
  
  const deltaX = event.clientX - startX.value
  let newWidth = startWidth.value
  
  if (props.direction === 'right') {
    newWidth = startWidth.value + deltaX
  } else if (props.direction === 'left') {
    newWidth = startWidth.value - deltaX
  }
  
  newWidth = Math.max(props.minWidth, Math.min(props.maxWidth, newWidth))
  width.value = newWidth
  emit('resize', newWidth)
}

const stopResize = () => {
  isResizing.value = false
  document.removeEventListener('mousemove', handleResize)
  document.removeEventListener('mouseup', stopResize)
  document.body.style.cursor = ''
  document.body.style.userSelect = ''
}

onMounted(() => {
  emit('resize', actualWidth.value)
})

onUnmounted(() => {
  if (isResizing.value) {
    stopResize()
  }
})
</script>

<style scoped>
.resizable-panel {
  position: relative;
  display: flex;
  height: 100%;
  transition: all 0.2s ease;
}

.panel-content {
  overflow: hidden;
  transition: width 0.2s ease;
  background: var(--ide-bg-secondary);
  border-right: 1px solid var(--ide-border);
}

.is-collapsed .panel-content {
  width: 0 !important;
}

.resize-handle {
  position: absolute;
  background: transparent;
  z-index: 10;
  transition: background-color 0.2s ease;
}

.resize-handle-right {
  right: -2px;
  top: 0;
  width: 4px;
  height: 100%;
  cursor: col-resize;
}

.resize-handle-left {
  left: -2px;
  top: 0;
  width: 4px;
  height: 100%;
  cursor: col-resize;
}

.resize-handle-top {
  top: -2px;
  left: 0;
  width: 100%;
  height: 4px;
  cursor: row-resize;
}

.resize-handle-bottom {
  bottom: -2px;
  left: 0;
  width: 100%;
  height: 4px;
  cursor: row-resize;
}

.resize-handle:hover {
  background-color: var(--ide-accent-primary);
}

.resize-handle:active {
  background-color: var(--ide-accent-primary);
}
</style>