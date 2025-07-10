import React from 'react';
import { Layout } from 'antd';
import Sidebar from './Sidebar';

const { Content } = Layout;

const MainLayout: React.FC = () => {
  return (
    <Layout style={{ minHeight: '100vh' }}>
      <Sidebar />
      <Layout>
        <Content style={{ margin: '16px' }}>
          {/* 主内容区域将在这里渲染 */}
        </Content>
      </Layout>
    </Layout>
  );
};

export default MainLayout; 