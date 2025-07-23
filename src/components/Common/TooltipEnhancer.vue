<template>
  <div 
    class="tooltip-enhancer"
    @mouseenter="showTooltip"
    @mouseleave="hideTooltip"
    @focus="showTooltip"
    @blur="hideTooltip"
  >
    <slot />
    
    <!-- 自定义工具提示 -->
    <div 
      v-if="isVisible"
      ref="tooltipRef"
      class="enhanced-tooltip"
      :class="[
        `tooltip-${placement}`,
        `tooltip-${theme}`,
        { 'tooltip-large': size === 'large' }
      ]"
      :style="tooltipStyle"
    >
      <div class="tooltip-content">
        <div v-if="title" class="tooltip-title">{{ title }}</div>
        <div class="tooltip-body">
          <slot name="content">
            {{ content }}
          </slot>
        </div>
        <div v-if="shortcut" class="tooltip-shortcut">
          <kbd>{{ shortcut }}</kbd>
        </div>
      </div>
      
      <!-- 工具提示箭头 -->
      <div class="tooltip-arrow"></div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, nextTick } from 'vue'

interface Props {
  content?: string
  title?: string
  shortcut?: string
  placement?: 'top' | 'bottom' | 'left' | 'right' | 'top-start' | 'top-end' | 'bottom-start' | 'bottom-end' | 'left-start' | 'left-end' | 'right-start' | 'right-end'
  theme?: 'dark' | 'light' | 'geek'
  size?: 'small' | 'medium' | 'large'
  delay?: number
  maxWidth?: number
  showArrow?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  content: '',
  title: '',
  shortcut: '',
  placement: 'top',
  theme: 'geek',
  size: 'medium',
  delay: 200,
  maxWidth: 300,
  showArrow: true
})

const isVisible = ref(false)
const tooltipRef = ref<HTMLElement>()
const timeout = ref<number>()

// 计算工具提示样式
const tooltipStyle = computed(() => {
  return {
    maxWidth: `${props.maxWidth}px`,
    zIndex: 9999
  }
})

// 显示工具提示
const showTooltip = () => {
  if (timeout.value) {
    clearTimeout(timeout.value)
  }
  
  timeout.value = window.setTimeout(() => {
    isVisible.value = true
    nextTick(() => {
      positionTooltip()
    })
  }, props.delay)
}

// 隐藏工具提示
const hideTooltip = () => {
  if (timeout.value) {
    clearTimeout(timeout.value)
  }
  isVisible.value = false
}

// 定位工具提示
const positionTooltip = () => {
  if (!tooltipRef.value) return
  
  const tooltip = tooltipRef.value
  const trigger = tooltip.parentElement
  if (!trigger) return
  
  const tooltipRect = tooltip.getBoundingClientRect()
  const triggerRect = trigger.getBoundingClientRect()
  
  let top = 0
  let left = 0
  
  switch (props.placement) {
    case 'top':
      top = triggerRect.top - tooltipRect.height - 8
      left = triggerRect.left + (triggerRect.width - tooltipRect.width) / 2
      break
    case 'bottom':
      top = triggerRect.bottom + 8
      left = triggerRect.left + (triggerRect.width - tooltipRect.width) / 2
      break
    case 'left':
      top = triggerRect.top + (triggerRect.height - tooltipRect.height) / 2
      left = triggerRect.left - tooltipRect.width - 8
      break
    case 'right':
      top = triggerRect.top + (triggerRect.height - tooltipRect.height) / 2
      left = triggerRect.right + 8
      break
    case 'top-start':
      top = triggerRect.top - tooltipRect.height - 8
      left = triggerRect.left
      break
    case 'top-end':
      top = triggerRect.top - tooltipRect.height - 8
      left = triggerRect.right - tooltipRect.width
      break
    case 'bottom-start':
      top = triggerRect.bottom + 8
      left = triggerRect.left
      break
    case 'bottom-end':
      top = triggerRect.bottom + 8
      left = triggerRect.right - tooltipRect.width
      break
    case 'left-start':
      top = triggerRect.top
      left = triggerRect.left - tooltipRect.width - 8
      break
    case 'left-end':
      top = triggerRect.bottom - tooltipRect.height
      left = triggerRect.left - tooltipRect.width - 8
      break
    case 'right-start':
      top = triggerRect.top
      left = triggerRect.right + 8
      break
    case 'right-end':
      top = triggerRect.bottom - tooltipRect.height
      left = triggerRect.right + 8
      break
  }
  
  // 确保工具提示不超出视窗
  const viewportWidth = window.innerWidth
  const viewportHeight = window.innerHeight
  
  if (left < 0) left = 8
  if (left + tooltipRect.width > viewportWidth) left = viewportWidth - tooltipRect.width - 8
  if (top < 0) top = 8
  if (top + tooltipRect.height > viewportHeight) top = viewportHeight - tooltipRect.height - 8
  
  tooltip.style.top = `${top}px`
  tooltip.style.left = `${left}px`
}

// 监听窗口大小变化
const handleResize = () => {
  if (isVisible.value) {
    positionTooltip()
  }
}

onMounted(() => {
  window.addEventListener('resize', handleResize)
})

onUnmounted(() => {
  window.removeEventListener('resize', handleResize)
  if (timeout.value) {
    clearTimeout(timeout.value)
  }
})
</script>

<style scoped>
.tooltip-enhancer {
  position: relative;
  display: inline-block;
}

.enhanced-tooltip {
  position: fixed;
  background: var(--ide-bg-secondary);
  border: 1px solid var(--ide-border);
  border-radius: 6px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
  padding: 12px;
  font-size: 13px;
  line-height: 1.4;
  color: var(--ide-text-primary);
  pointer-events: none;
  z-index: 9999;
  max-width: 300px;
  word-wrap: break-word;
}

.tooltip-content {
  position: relative;
}

.tooltip-title {
  font-weight: 600;
  margin-bottom: 6px;
  color: white;
  font-size: 14px;
}

.tooltip-body {
  margin-bottom: 8px;
}

.tooltip-shortcut {
  margin-top: 8px;
  padding-top: 8px;
  border-top: 1px solid var(--ide-border);
}

.tooltip-shortcut kbd {
  background: var(--ide-bg-tertiary);
  border: 1px solid var(--ide-border);
  border-radius: 3px;
  padding: 2px 6px;
  font-family: var(--ide-font-mono);
  font-size: 11px;
  color: var(--ide-accent-orange);
}

.tooltip-arrow {
  position: absolute;
  width: 0;
  height: 0;
  border: 6px solid transparent;
}

/* 箭头位置 */
.tooltip-top .tooltip-arrow {
  bottom: -6px;
  left: 50%;
  transform: translateX(-50%);
  border-top-color: var(--ide-bg-secondary);
  border-bottom: none;
}

.tooltip-bottom .tooltip-arrow {
  top: -6px;
  left: 50%;
  transform: translateX(-50%);
  border-bottom-color: var(--ide-bg-secondary);
  border-top: none;
}

.tooltip-left .tooltip-arrow {
  right: -6px;
  top: 50%;
  transform: translateY(-50%);
  border-left-color: var(--ide-bg-secondary);
  border-right: none;
}

.tooltip-right .tooltip-arrow {
  left: -6px;
  top: 50%;
  transform: translateY(-50%);
  border-right-color: var(--ide-bg-secondary);
  border-left: none;
}

/* 主题样式 */
.tooltip-dark {
  background: #2b2b2b;
  border-color: #555;
  color: #a9b7c6;
}

.tooltip-light {
  background: #ffffff;
  border-color: #e1e1e1;
  color: #333333;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.tooltip-geek {
  background: var(--ide-bg-secondary);
  border-color: var(--ide-border);
  color: var(--ide-text-primary);
}

/* 尺寸样式 */
.tooltip-small {
  padding: 8px;
  font-size: 12px;
}

.tooltip-medium {
  padding: 12px;
  font-size: 13px;
}

.tooltip-large {
  padding: 16px;
  font-size: 14px;
  max-width: 400px;
}

/* 动画效果 */
.enhanced-tooltip {
  animation: tooltipFadeIn 0.2s ease-out;
}

@keyframes tooltipFadeIn {
  from {
    opacity: 0;
    transform: scale(0.9);
  }
  to {
    opacity: 1;
    transform: scale(1);
  }
}

/* 特殊样式 */
.tooltip-info {
  border-left: 4px solid var(--ide-accent-blue);
}

.tooltip-success {
  border-left: 4px solid var(--ide-accent-green);
}

.tooltip-warning {
  border-left: 4px solid var(--ide-accent-orange);
}

.tooltip-error {
  border-left: 4px solid var(--ide-accent-red);
}

/* 响应式设计 */
@media (max-width: 768px) {
  .enhanced-tooltip {
    max-width: 250px;
    font-size: 12px;
    padding: 10px;
  }
  
  .tooltip-large {
    max-width: 300px;
  }
}

/* 高对比度模式 */
@media (prefers-contrast: high) {
  .enhanced-tooltip {
    border-width: 2px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.5);
  }
}

/* 减少动画模式 */
@media (prefers-reduced-motion: reduce) {
  .enhanced-tooltip {
    animation: none;
  }
}

/* 工具提示内容样式 */
.tooltip-content code {
  background: var(--ide-bg-tertiary);
  padding: 2px 4px;
  border-radius: 3px;
  font-family: var(--ide-font-mono);
  font-size: 12px;
  color: var(--ide-accent-orange);
}

.tooltip-content pre {
  background: var(--ide-bg-tertiary);
  padding: 8px;
  border-radius: 4px;
  font-family: var(--ide-font-mono);
  font-size: 11px;
  overflow-x: auto;
  margin: 8px 0;
}

.tooltip-content ul {
  margin: 8px 0;
  padding-left: 16px;
}

.tooltip-content li {
  margin: 4px 0;
}

/* 工具提示链接样式 */
.tooltip-content a {
  color: var(--ide-accent-primary);
  text-decoration: none;
}

.tooltip-content a:hover {
  text-decoration: underline;
}

/* 工具提示表格样式 */
.tooltip-content table {
  width: 100%;
  border-collapse: collapse;
  margin: 8px 0;
  font-size: 12px;
}

.tooltip-content th,
.tooltip-content td {
  padding: 4px 8px;
  border: 1px solid var(--ide-border);
  text-align: left;
}

.tooltip-content th {
  background: var(--ide-bg-tertiary);
  font-weight: 600;
}
</style> 