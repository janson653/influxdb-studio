<template>
  <div class="animation-enhancer">
    <!-- 淡入淡出动画 -->
    <transition name="fade" mode="out-in">
      <slot v-if="visible" />
    </transition>
    
    <!-- 滑动动画 -->
    <transition name="slide" mode="out-in">
      <slot v-if="visible" />
    </transition>
    
    <!-- 缩放动画 -->
    <transition name="scale" mode="out-in">
      <slot v-if="visible" />
    </transition>
    
    <!-- 旋转动画 -->
    <transition name="rotate" mode="out-in">
      <slot v-if="visible" />
    </transition>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'

interface Props {
  type?: 'fade' | 'slide' | 'scale' | 'rotate' | 'bounce' | 'shake'
  delay?: number
  duration?: number
  autoStart?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  type: 'fade',
  delay: 0,
  duration: 300,
  autoStart: true
})

const visible = ref(false)

onMounted(() => {
  if (props.autoStart) {
    setTimeout(() => {
      visible.value = true
    }, props.delay)
  }
})

// 暴露方法给父组件
defineExpose({
  show: () => {
    visible.value = true
  },
  hide: () => {
    visible.value = false
  },
  toggle: () => {
    visible.value = !visible.value
  }
})
</script>

<style scoped>
/* 淡入淡出动画 */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

/* 滑动动画 */
.slide-enter-active,
.slide-leave-active {
  transition: all 0.3s ease;
}

.slide-enter-from {
  transform: translateX(-100%);
  opacity: 0;
}

.slide-leave-to {
  transform: translateX(100%);
  opacity: 0;
}

/* 缩放动画 */
.scale-enter-active,
.scale-leave-active {
  transition: all 0.3s ease;
}

.scale-enter-from {
  transform: scale(0.8);
  opacity: 0;
}

.scale-leave-to {
  transform: scale(1.2);
  opacity: 0;
}

/* 旋转动画 */
.rotate-enter-active,
.rotate-leave-active {
  transition: all 0.3s ease;
}

.rotate-enter-from {
  transform: rotate(-180deg) scale(0.8);
  opacity: 0;
}

.rotate-leave-to {
  transform: rotate(180deg) scale(1.2);
  opacity: 0;
}

/* 弹跳动画 */
@keyframes bounce {
  0%, 20%, 53%, 80%, 100% {
    transform: translate3d(0, 0, 0);
  }
  40%, 43% {
    transform: translate3d(0, -30px, 0);
  }
  70% {
    transform: translate3d(0, -15px, 0);
  }
  90% {
    transform: translate3d(0, -4px, 0);
  }
}

.bounce-enter-active {
  animation: bounce 0.6s ease;
}

/* 抖动动画 */
@keyframes shake {
  0%, 100% {
    transform: translateX(0);
  }
  10%, 30%, 50%, 70%, 90% {
    transform: translateX(-10px);
  }
  20%, 40%, 60%, 80% {
    transform: translateX(10px);
  }
}

.shake-enter-active {
  animation: shake 0.6s ease;
}

/* 脉冲动画 */
@keyframes pulse {
  0% {
    transform: scale(1);
  }
  50% {
    transform: scale(1.05);
  }
  100% {
    transform: scale(1);
  }
}

.pulse-enter-active {
  animation: pulse 0.6s ease;
}

/* 闪烁动画 */
@keyframes flash {
  0%, 50%, 100% {
    opacity: 1;
  }
  25%, 75% {
    opacity: 0;
  }
}

.flash-enter-active {
  animation: flash 0.6s ease;
}

/* 摇摆动画 */
@keyframes swing {
  20% {
    transform: rotate(15deg);
  }
  40% {
    transform: rotate(-10deg);
  }
  60% {
    transform: rotate(5deg);
  }
  80% {
    transform: rotate(-5deg);
  }
  100% {
    transform: rotate(0deg);
  }
}

.swing-enter-active {
  animation: swing 0.6s ease;
}

/* 淡入动画 */
@keyframes fadeIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}

.fadeIn-enter-active {
  animation: fadeIn 0.6s ease;
}

/* 滑入动画 */
@keyframes slideIn {
  from {
    transform: translateY(-100%);
    opacity: 0;
  }
  to {
    transform: translateY(0);
    opacity: 1;
  }
}

.slideIn-enter-active {
  animation: slideIn 0.6s ease;
}

/* 缩放进入动画 */
@keyframes zoomIn {
  from {
    transform: scale(0.3);
    opacity: 0;
  }
  50% {
    opacity: 1;
  }
  to {
    transform: scale(1);
  }
}

.zoomIn-enter-active {
  animation: zoomIn 0.6s ease;
}

/* 翻转动画 */
@keyframes flip {
  from {
    transform: perspective(400px) rotateY(0);
  }
  40% {
    transform: perspective(400px) rotateY(-10deg);
  }
  50% {
    transform: perspective(400px) rotateY(10deg);
  }
  60% {
    transform: perspective(400px) rotateY(-10deg);
  }
  to {
    transform: perspective(400px) rotateY(0);
  }
}

.flip-enter-active {
  animation: flip 0.6s ease;
}

/* 弹性动画 */
@keyframes elastic {
  0% {
    transform: scale(0);
  }
  50% {
    transform: scale(1.1);
  }
  100% {
    transform: scale(1);
  }
}

.elastic-enter-active {
  animation: elastic 0.6s ease;
}

/* 回弹动画 */
@keyframes backIn {
  0% {
    transform: scale(0.3) translateX(-200px);
    opacity: 0;
  }
  50% {
    transform: scale(1.05) translateX(0);
    opacity: 0.8;
  }
  70% {
    transform: scale(0.9);
  }
  100% {
    transform: scale(1) translateX(0);
    opacity: 1;
  }
}

.backIn-enter-active {
  animation: backIn 0.6s ease;
}

/* 自定义动画类 */
.animation-enhancer {
  position: relative;
}

/* 悬停效果 */
.hover-lift {
  transition: transform 0.2s ease, box-shadow 0.2s ease;
}

.hover-lift:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.hover-scale {
  transition: transform 0.2s ease;
}

.hover-scale:hover {
  transform: scale(1.05);
}

.hover-rotate {
  transition: transform 0.2s ease;
}

.hover-rotate:hover {
  transform: rotate(5deg);
}

.hover-glow {
  transition: box-shadow 0.2s ease;
}

.hover-glow:hover {
  box-shadow: 0 0 20px rgba(104, 151, 187, 0.5);
}

/* 加载动画 */
@keyframes loading {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
}

.loading-spinner {
  animation: loading 1s linear infinite;
}

/* 进度条动画 */
@keyframes progress {
  0% {
    width: 0%;
  }
  100% {
    width: 100%;
  }
}

.progress-bar {
  animation: progress 2s ease-in-out;
}

/* 打字机效果 */
@keyframes typewriter {
  from {
    width: 0;
  }
  to {
    width: 100%;
  }
}

.typewriter {
  overflow: hidden;
  white-space: nowrap;
  animation: typewriter 2s steps(40, end);
}

/* 呼吸效果 */
@keyframes breathe {
  0%, 100% {
    transform: scale(1);
  }
  50% {
    transform: scale(1.05);
  }
}

.breathe {
  animation: breathe 2s ease-in-out infinite;
}

/* 波浪效果 */
@keyframes wave {
  0%, 100% {
    transform: translateY(0);
  }
  50% {
    transform: translateY(-10px);
  }
}

.wave {
  animation: wave 1s ease-in-out infinite;
}

/* 闪烁边框效果 */
@keyframes borderBlink {
  0%, 100% {
    border-color: transparent;
  }
  50% {
    border-color: var(--geek-accent-primary);
  }
}

.border-blink {
  animation: borderBlink 1s ease-in-out infinite;
}

/* 渐变背景动画 */
@keyframes gradientShift {
  0% {
    background-position: 0% 50%;
  }
  50% {
    background-position: 100% 50%;
  }
  100% {
    background-position: 0% 50%;
  }
}

.gradient-bg {
  background: linear-gradient(-45deg, #ee7752, #e73c7e, #23a6d5, #23d5ab);
  background-size: 400% 400%;
  animation: gradientShift 3s ease infinite;
}
</style> 