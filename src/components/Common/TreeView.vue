<template>
  <div 
    class="geek-tree-view geek-scrollbar"
    @contextmenu.prevent="handleContextMenu($event, null)"
  >
    <div v-if="loading" class="geek-tree-view__loading">
      <el-icon class="is-loading"><Loading /></el-icon>
      <span>Loading...</span>
    </div>
    
    <div v-else-if="!nodes || nodes.length === 0" class="geek-tree-view__empty">
      <el-empty :description="emptyText" :image-size="60" />
    </div>
    
    <div v-else class="geek-tree-view__content">
      <tree-node
        v-for="node in nodes"
        :key="(node as any).id || (node as any).name"
        :node="node as any"
        :level="0"
        :expanded-nodes="expandedNodes"
        @toggle="toggleNode"
        @node-click="handleNodeClick"
        @node-dblclick="handleNodeDoubleClick"
        @context-menu="handleContextMenu"
      />
    </div>
    
    <!-- Context Menu -->
    <div 
      v-show="showContextMenu"
      class="geek-tree-view__context-menu"
      :style="contextMenuStyle"
    >
      <div 
        v-for="(item, index) in contextMenuItems" 
        :key="index"
        class="geek-tree-view__context-menu-item"
        :class="{ 'geek-tree-view__context-menu-item--disabled': item.disabled }"
        @click="handleContextMenuAction(item)"
      >
        <el-icon v-if="item.icon" class="geek-tree-view__context-menu-icon">
          <component :is="item.icon" />
        </el-icon>
        <span>{{ item.label }}</span>
      </div>
      
      <div 
        v-if="contextMenuItems.length === 0"
        class="geek-tree-view__context-menu-item geek-tree-view__context-menu-item--disabled"
      >
        <span>No actions available</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onBeforeUnmount } from 'vue';
import { Loading } from '@element-plus/icons-vue';
import TreeNode from './TreeViewNode.vue';

// Define props
const props = defineProps({
  nodes: {
    type: Array,
    default: () => []
  },
  loading: {
    type: Boolean,
    default: false
  },
  emptyText: {
    type: String,
    default: 'No data'
  },
  nodeKey: {
    type: String,
    default: 'id'
  },
  defaultExpandAll: {
    type: Boolean,
    default: false
  },
  defaultExpandedKeys: {
    type: Array,
    default: () => []
  }
});

// Define emits
const emit = defineEmits([
  'node-click', 
  'node-dblclick', 
  'node-expand', 
  'node-collapse',
  'context-menu-action'
]);

// State
const expandedNodes = ref<Set<string | number>>(new Set());
const showContextMenu = ref(false);
const contextMenuPosition = ref({ x: 0, y: 0 });
const contextMenuNode = ref<any>(null);
const contextMenuItems = ref<any[]>([]);

// Computed
const contextMenuStyle = computed(() => {
  return {
    left: `${contextMenuPosition.value.x}px`,
    top: `${contextMenuPosition.value.y}px`
  };
});

// Methods
const toggleNode = (node: any) => {
  const nodeId = node[props.nodeKey] || node.name;
  
  if (expandedNodes.value.has(nodeId)) {
    expandedNodes.value.delete(nodeId);
    emit('node-collapse', node);
  } else {
    expandedNodes.value.add(nodeId);
    emit('node-expand', node);
  }
};

const handleNodeClick = (node: any) => {
  emit('node-click', node);
};

const handleNodeDoubleClick = (node: any) => {
  emit('node-dblclick', node);
};

const handleContextMenu = (event: MouseEvent, node: any) => {
  event.preventDefault();
  event.stopPropagation();
  
  // Set context menu position
  contextMenuPosition.value = {
    x: event.clientX,
    y: event.clientY
  };
  
  // Set context menu node
  contextMenuNode.value = node;
  
  // Generate context menu items based on node type
  generateContextMenuItems(node);
  
  // Show context menu
  showContextMenu.value = true;
};

const generateContextMenuItems = (node: any) => {
  const items: any[] = [];
  
  if (!node) {
    // Root level context menu
    items.push({
      label: 'Refresh All',
      icon: 'Refresh',
      action: 'refresh-all'
    });
    
    // Add more root level actions if needed
    
  } else {
    // Node specific context menu
    items.push({
      label: 'Refresh',
      icon: 'Refresh',
      action: 'refresh',
      data: node
    });
    
    if (node.type === 'database') {
      items.push({
        label: 'Create Measurement',
        icon: 'Plus',
        action: 'create-measurement',
        data: node
      });
      
      items.push({
        label: 'Delete Database',
        icon: 'Delete',
        action: 'delete-database',
        data: node
      });
    }
    
    if (node.type === 'measurement') {
      items.push({
        label: 'Query Data',
        icon: 'Document',
        action: 'query-data',
        data: node
      });
      
      items.push({
        label: 'Delete Measurement',
        icon: 'Delete',
        action: 'delete-measurement',
        data: node
      });
    }
  }
  
  contextMenuItems.value = items;
};

const handleContextMenuAction = (item: any) => {
  if (item.disabled) return;
  
  emit('context-menu-action', {
    action: item.action,
    node: contextMenuNode.value,
    data: item.data
  });
  
  hideContextMenu();
};

const hideContextMenu = () => {
  showContextMenu.value = false;
};

const handleClickOutside = () => {
  hideContextMenu();
};

// Lifecycle hooks
onMounted(() => {
  // Initialize expanded nodes
  if (props.defaultExpandAll) {
    const initExpandedNodes = (nodes: any[]) => {
      nodes.forEach(node => {
        const nodeId = node[props.nodeKey] || node.name;
        expandedNodes.value.add(nodeId);
        
        if (node.children && node.children.length > 0) {
          initExpandedNodes(node.children);
        }
      });
    };
    
    initExpandedNodes(props.nodes);
  } else if (props.defaultExpandedKeys.length > 0) {
    props.defaultExpandedKeys.forEach(key => {
      expandedNodes.value.add(key as string);
    });
  }
  
  // Add click outside listener
  document.addEventListener('click', handleClickOutside);
});

onBeforeUnmount(() => {
  // Remove click outside listener
  document.removeEventListener('click', handleClickOutside);
});
</script>

<style>
.geek-tree-view {
  position: relative;
  height: 100%;
  overflow-y: auto;
  font-family: var(--geek-font-sans);
  color: var(--geek-text-primary);
}

.geek-tree-view__loading,
.geek-tree-view__empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: var(--geek-spacing-xl);
  color: var(--geek-text-secondary);
}

.geek-tree-view__loading .el-icon {
  font-size: 24px;
  margin-bottom: var(--geek-spacing-sm);
}

.geek-tree-view__content {
  padding: var(--geek-spacing-xs) 0;
}

.geek-tree-view__context-menu {
  position: fixed;
  min-width: 160px;
  background-color: var(--geek-bg-secondary);
  border: 1px solid var(--geek-border-color);
  border-radius: 3px;
  box-shadow: var(--geek-shadow);
  z-index: 1000;
  padding: var(--geek-spacing-xs) 0;
}

.geek-tree-view__context-menu-item {
  display: flex;
  align-items: center;
  padding: var(--geek-spacing-sm) var(--geek-spacing-md);
  cursor: pointer;
  transition: background-color var(--geek-transition-fast);
}

.geek-tree-view__context-menu-item:hover {
  background-color: var(--geek-bg-highlight);
}

.geek-tree-view__context-menu-item--disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.geek-tree-view__context-menu-item--disabled:hover {
  background-color: transparent;
}

.geek-tree-view__context-menu-icon {
  margin-right: var(--geek-spacing-sm);
  font-size: var(--geek-font-size-sm);
}
</style>