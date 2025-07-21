// 数据库显示问题调试脚本
// 在浏览器控制台中运行此脚本来诊断问题

console.log('=== 数据库显示问题诊断 ===');

// 1. 检查连接状态
function checkConnectionStatus() {
  console.log('1. 检查连接状态:');
  
  // 获取连接store
  const connectionStore = window.__VUE_APP__.config.globalProperties.$pinia._s.get('connection');
  
  if (!connectionStore) {
    console.error('❌ 无法获取连接store');
    return false;
  }
  
  console.log('✅ 连接store存在');
  
  const activeConnection = connectionStore.activeConnectionConfig;
  const connectionStatus = connectionStore.connectionStatus;
  const activeConnectionId = connectionStore.activeConnection;
  
  console.log('活跃连接ID:', activeConnectionId);
  console.log('活跃连接配置:', activeConnection);
  console.log('连接状态:', connectionStatus);
  
  if (!activeConnection) {
    console.error('❌ 没有活跃连接配置');
    return false;
  }
  
  if (!activeConnectionId) {
    console.error('❌ 没有活跃连接ID');
    return false;
  }
  
  const status = connectionStatus[activeConnectionId];
  console.log('当前连接状态:', status);
  
  if (!status) {
    console.error('❌ 没有连接状态信息');
    return false;
  }
  
  if (status.status !== 'connected') {
    console.error(`❌ 连接状态不是 'connected'，当前状态: ${status.status}`);
    return false;
  }
  
  if (!status.backendConnectionId) {
    console.error('❌ 没有后端连接ID');
    return false;
  }
  
  console.log('✅ 连接状态正常');
  return true;
}

// 2. 检查数据库树数据
function checkDatabaseTreeData() {
  console.log('\n2. 检查数据库树数据:');
  
  // 获取MainLayout组件实例
  const mainLayout = document.querySelector('.main-layout');
  if (!mainLayout) {
    console.error('❌ 无法找到MainLayout组件');
    return false;
  }
  
  // 尝试获取Vue组件实例
  const vueInstance = mainLayout.__vueParentComponent;
  if (!vueInstance) {
    console.error('❌ 无法获取Vue组件实例');
    return false;
  }
  
  const databaseTreeData = vueInstance.ctx.databaseTreeData;
  const isLoading = vueInstance.ctx.isLoading;
  
  console.log('数据库树数据:', databaseTreeData);
  console.log('是否正在加载:', isLoading);
  
  if (!databaseTreeData || databaseTreeData.length === 0) {
    console.error('❌ 数据库树数据为空');
    return false;
  }
  
  console.log('✅ 数据库树数据存在');
  return true;
}

// 3. 手动触发数据库加载
async function triggerDatabaseLoad() {
  console.log('\n3. 手动触发数据库加载:');
  
  try {
    const connectionStore = window.__VUE_APP__.config.globalProperties.$pinia._s.get('connection');
    const activeConnectionId = connectionStore.activeConnection;
    const status = connectionStore.connectionStatus[activeConnectionId];
    
    if (!status?.backendConnectionId) {
      console.error('❌ 没有有效的后端连接ID');
      return false;
    }
    
    console.log('正在调用 get_databases...');
    const { invoke } = await import('@tauri-apps/api/core');
    const response = await invoke('get_databases', { connectionId: status.backendConnectionId });
    
    console.log('get_databases 响应:', response);
    
    if (response.success) {
      console.log('✅ 数据库查询成功');
      console.log('数据库列表:', response.data);
      return true;
    } else {
      console.error('❌ 数据库查询失败:', response.error);
      return false;
    }
  } catch (error) {
    console.error('❌ 调用 get_databases 时出错:', error);
    return false;
  }
}

// 4. 检查DOM元素
function checkDOMElements() {
  console.log('\n4. 检查DOM元素:');
  
  const sidebar = document.querySelector('.sidebar');
  const databaseExplorer = document.querySelector('.database-explorer');
  const explorerTree = document.querySelector('.explorer-tree');
  const emptyState = document.querySelector('.empty-state');
  const tree = document.querySelector('.el-tree');
  
  console.log('侧边栏:', !!sidebar);
  console.log('数据库浏览器:', !!databaseExplorer);
  console.log('浏览器树容器:', !!explorerTree);
  console.log('空状态显示:', !!emptyState);
  console.log('树组件:', !!tree);
  
  if (emptyState && emptyState.style.display !== 'none') {
    console.log('⚠️ 显示空状态，可能没有连接或数据');
  }
  
  if (tree) {
    const treeNodes = tree.querySelectorAll('.el-tree-node');
    console.log('树节点数量:', treeNodes.length);
  }
}

// 执行诊断
async function runDiagnosis() {
  console.log('开始诊断...\n');
  
  const connectionOk = checkConnectionStatus();
  const treeDataOk = checkDatabaseTreeData();
  const domOk = checkDOMElements();
  
  if (connectionOk) {
    await triggerDatabaseLoad();
  }
  
  console.log('\n=== 诊断完成 ===');
  console.log('建议:');
  if (!connectionOk) {
    console.log('- 检查连接配置和连接状态');
  }
  if (!treeDataOk) {
    console.log('- 检查数据库数据加载');
  }
  if (!domOk) {
    console.log('- 检查DOM元素渲染');
  }
}

// 运行诊断
runDiagnosis(); 