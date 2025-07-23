<template>
  <div class="geek-tree-node">
    <div 
      class="geek-tree-node__content"
      :class="{ 'geek-tree-node__content--selected': isSelected }"
      :style="{ paddingLeft: `${level * 16}px` }"
      @click="handleClick"
      @dblclick="handleDoubleClick"
      @contextmenu.prevent="handleContextMenu"
    >
      <div 
        v-if="hasChildren"
        class="geek-tree-node__expand-icon"
        @click.stop="handleToggle"
      >
        <el-icon>
          <component :is="isExpanded ? 'ArrowDown' : 'ArrowRight'" />
        </el-icon>
      </div>
      <div v-else class="geek-tree-node__expand-placeholder"></div>
      
      <div class="geek-tree-node__icon">
        <el-icon>
          <Folder v-if="node.type === 'database'" />
          <Document v-else-if="node.type === 'measurement'" />
          <Document v-else />
        </el-icon>
      </div>
      
      <div class="geek-tree-node__label">
        {{ node.name }}
        <span v-if="node.count !== undefined" class="geek-tree-node__count">
          ({{ node.count }})
        </span>
      </div>
    </div>
    
    <div v-if="isExpanded && hasChildren" class="geek-tree-node__children">
      <tree-node
        v-for="child in node.children"
        :key="child.id || child.name"
        :node="child"
        :level="level + 1"
        :expanded-nodes="expandedNodes"
        @toggle="handleChildToggle"
        @node-click="handleChildClick"
        @node-dblclick="handleChildDoubleClick"
        @context-menu="handleChildContextMenu"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import {
  Folder, Document
} from '@element-plus/icons-vue';

// Define props
const props = defineProps({
  node: {
    type: Object,
    required: true
  },
  level: {
    type: Number,
    default: 0
  },
  expandedNodes: {
    type: Object,
    required: true
  },
  selectedNode: {
    type: Object,
    default: null
  }
});

// Define emits
const emit = defineEmits([
  'toggle', 
  'node-click', 
  'node-dblclick', 
  'context-menu'
]);

// Computed
const nodeId = computed(() => {
  return props.node.id || props.node.name;
});

const isExpanded = computed(() => {
  return props.expandedNodes.has(nodeId.value);
});

const hasChildren = computed(() => {
  return props.node.children && props.node.children.length > 0;
});

const isSelected = computed(() => {
  return props.selectedNode === props.node;
});

// const nodeIcon = computed(() => {
//   if (!props.node.type) return 'Document';
  
//   switch (props.node.type) {
//     case 'database':
//       return isExpanded.value ? 'FolderOpened' : 'Folder';
//     case 'measurement':
//       return 'DataLine';
//     case 'tag':
//       return 'Collection';
//     case 'field':
//       return 'Document';
//     case 'system':
//       return 'Setting';
//     default:
//       return 'Document';
//   }
// });

// Methods
const handleToggle = () => {
  emit('toggle', props.node);
};

const handleClick = () => {
  emit('node-click', props.node);
};

const handleDoubleClick = () => {
  emit('node-dblclick', props.node);
};

const handleContextMenu = (event: MouseEvent) => {
  emit('context-menu', event, props.node);
};

const handleChildToggle = (node: any) => {
  emit('toggle', node);
};

const handleChildClick = (node: any) => {
  emit('node-click', node);
};

const handleChildDoubleClick = (node: any) => {
  emit('node-dblclick', node);
};

const handleChildContextMenu = (event: MouseEvent, node: any) => {
  emit('context-menu', event, node);
};
</script>

<style>
.geek-tree-node {
  font-size: var(--geek-font-size-sm);
}

.geek-tree-node__content {
  display: flex;
  align-items: center;
  height: 24px;
  padding-right: var(--geek-spacing-md);
  cursor: pointer;
  transition: background-color var(--geek-transition-fast);
}

.geek-tree-node__content:hover {
  background-color: var(--geek-bg-highlight);
}

.geek-tree-node__content--selected {
  background-color: var(--geek-accent-green);
  color: var(--geek-text-highlight);
}

.geek-tree-node__expand-icon,
.geek-tree-node__expand-placeholder {
  width: 16px;
  height: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  margin-right: var(--geek-spacing-xs);
}

.geek-tree-node__icon {
  margin-right: var(--geek-spacing-xs);
  display: flex;
  align-items: center;
  justify-content: center;
}

.geek-tree-node__label {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.geek-tree-node__count {
  color: var(--geek-text-secondary);
  font-size: var(--geek-font-size-xs);
  margin-left: var(--geek-spacing-xs);
}

.geek-tree-node__children {
  margin-left: var(--geek-spacing-md);
}
</style>