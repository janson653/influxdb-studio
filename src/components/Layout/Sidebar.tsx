import React from 'react';
import { Layout, Menu } from 'antd';
import { ApiOutlined, ShareAltOutlined } from '@ant-design/icons';

const { Sider } = Layout;

const Sidebar: React.FC = () => {
  return (
    <Sider width={200} theme="dark">
      <div style={{ height: '32px', margin: '16px', background: 'rgba(255, 255, 255, 0.2)' }}>
        {/* Logo Placeholder */}
      </div>
      <Menu theme="dark" mode="inline" defaultSelectedKeys={['1']}>
        <Menu.Item key="1" icon={<ApiOutlined />}>
          Connections
        </Menu.Item>
        <Menu.Item key="2" icon={<ShareAltOutlined />}>
          Another Item
        </Menu.Item>
      </Menu>
    </Sider>
  );
};

export default Sidebar; 