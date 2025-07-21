# 故障排除指南

## 开发环境问题

### Tauri 开发环境启动失败

#### 问题描述
启动 Tauri 开发环境时遇到问题

#### 解决方案

**方案一：清理缓存（推荐）**
```bash
# 清理 Vite 缓存
rm -rf node_modules/.vite

# 重新启动开发环境
pnpm tauri dev
```

**方案二：使用修复脚本**
```bash
# 运行自动修复脚本
./scripts/fix-tauri-dev.sh
```

**方案三：优化 Vite 配置**
在 `vite.config.ts` 中添加文件监视器忽略规则：

```typescript
export default defineConfig({
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      ignored: [
        "**/src-tauri/**",
        "**/node_modules/**"
      ],
    },
  },
  // ... 其他配置
});
```

### 端口占用问题

#### 检查端口占用
```bash
# 检查端口 1420 是否被占用
lsof -ti:1420

# 释放端口
lsof -ti:1420 | xargs kill -9
```

#### 修改端口配置
如果端口 1420 被占用，可以在 `vite.config.ts` 中修改端口：

```typescript
export default defineConfig({
  server: {
    port: 3000, // 修改为其他可用端口
    strictPort: false, // 允许自动选择端口
  },
});
```

### 依赖安装失败

#### 前端依赖问题
```bash
# 清理缓存
pnpm store prune
rm -rf node_modules
pnpm install

# 如果使用 npm
npm cache clean --force
rm -rf node_modules package-lock.json
npm install
```

#### Rust 依赖问题
```bash
cd src-tauri
cargo clean
cargo build

# 更新 Rust 工具链
rustup update
```

#### 系统依赖问题（Linux）
```bash
# Ubuntu/Debian
sudo apt update
sudo apt install build-essential curl wget file git libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev

# CentOS/RHEL
sudo yum groupinstall "Development Tools"
sudo yum install curl wget file git openssl-devel gtk3-devel libappindicator-gtk3 librsvg2-devel
```

## 构建问题

### 构建失败

#### 检查构建日志
1. 查看 GitHub Actions 的详细日志
2. 确认构建环境配置
3. 验证依赖版本兼容性

#### 平台特定问题

**Windows 构建问题**
```bash
# 确保安装了 Visual Studio Build Tools
# 或者安装完整的 Visual Studio Community

# 检查 Rust 工具链
rustup target list --installed
rustup target add x86_64-pc-windows-msvc
```

**macOS 构建问题**
```bash
# 安装 Xcode Command Line Tools
xcode-select --install

# 检查 Rust 工具链
rustup target list --installed
rustup target add x86_64-apple-darwin
```

**Linux 构建问题**
```bash
# 安装必要的系统依赖
sudo apt update
sudo apt install build-essential curl wget file git libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev


```



## 运行时问题

### 应用启动失败

#### 检查错误日志
```bash
# 查看应用日志
# Windows: 查看事件查看器
# macOS: 查看控制台应用
# Linux: 查看系统日志
journalctl -f -u influxdb-studio
```

#### 权限问题
```bash
# Linux: 确保应用有执行权限
chmod +x influxdb-studio

# macOS: 处理安全限制
# 在系统偏好设置 > 安全性与隐私中允许应用运行
```

### 数据库连接问题

#### 连接超时
1. 检查网络连接
2. 验证 InfluxDB 服务是否运行
3. 确认防火墙设置
4. 检查连接配置（主机、端口、认证信息）

#### 认证失败
1. 验证用户名和密码
2. 检查 Token 是否有效
3. 确认用户权限
4. 检查 InfluxDB 版本兼容性

#### SSL/TLS 问题
```bash
# 测试 SSL 连接
openssl s_client -connect your-influxdb-host:8086

# 检查证书
openssl x509 -in certificate.pem -text -noout
```

## 性能问题

### 应用启动慢
1. 检查系统资源使用情况
2. 优化依赖加载
3. 使用生产构建版本
4. 检查杀毒软件是否影响

### 查询执行慢
1. 优化查询语句
2. 检查数据库索引
3. 分析查询计划
4. 考虑使用缓存

### 内存使用过高
1. 检查内存泄漏
2. 优化数据结构
3. 减少不必要的组件渲染
4. 使用内存分析工具

## 调试技巧

### 前端调试

#### Vue DevTools
1. 安装 Vue DevTools 浏览器扩展
2. 检查组件状态和属性
3. 监控状态变化
4. 分析性能问题

#### 浏览器开发者工具
```javascript
// 在控制台中调试
console.log('调试信息');
debugger; // 设置断点

// 监控网络请求
// 在 Network 标签页查看 API 调用
```

### 后端调试

#### Rust 调试
```rust
// 使用 println! 输出调试信息
println!("调试信息: {:?}", variable);

// 使用 dbg! 宏
dbg!(&variable);

// 使用日志
tracing::info!("调试信息: {:?}", variable);
```

#### 网络调试
```bash
# 使用 curl 测试 API
curl -X POST http://localhost:8086/query \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "q=SHOW DATABASES"

# 使用 Wireshark 分析网络流量
# 过滤 InfluxDB 相关流量
```

### 系统调试

#### 进程监控
```bash
# 监控应用进程
ps aux | grep influxdb-studio

# 监控系统资源
htop
iotop
```

#### 日志分析
```bash
# 查看系统日志
tail -f /var/log/syslog

# 查看应用日志
tail -f ~/.config/influxdb-studio/logs/app.log
```

## 常见错误代码

### 错误代码说明

| 错误代码 | 说明 | 解决方案 |
|---------|------|----------|
| ELOOP | 符号链接循环 | 清理缓存目录 |
| EADDRINUSE | 端口被占用 | 释放端口或修改配置 |
| EACCES | 权限不足 | 检查文件权限 |
| ECONNREFUSED | 连接被拒绝 | 检查服务是否运行 |
| ETIMEDOUT | 连接超时 | 检查网络和防火墙 |

### 错误处理最佳实践

1. **提供清晰的错误信息**
2. **记录详细的错误日志**
3. **实现优雅的错误恢复**
4. **提供用户友好的错误提示**

## 预防措施

### 开发环境管理

1. **定期清理构建缓存**
2. **使用版本管理工具**
3. **配置自动化脚本**
4. **监控系统资源**

### 代码质量

1. **编写单元测试**
2. **使用静态代码分析**
3. **遵循编码规范**
4. **定期代码审查**

### 部署准备

1. **测试所有平台构建**
2. **验证安装包功能**
3. **检查依赖兼容性**
4. **准备回滚方案**

## 获取帮助

### 官方资源
- [Tauri 官方文档](https://tauri.app/docs/)
- [Vue.js 官方文档](https://vuejs.org/guide/)
- [Rust 官方文档](https://doc.rust-lang.org/)
- [InfluxDB 官方文档](https://docs.influxdata.com/)

### 社区支持
- [GitHub Issues](https://github.com/your-username/influxdb-studio/issues)
- [Stack Overflow](https://stackoverflow.com/questions/tagged/tauri)
- [Discord 社区](https://discord.gg/tauri)

### 报告问题

报告问题时请包含：
1. 操作系统和版本
2. Node.js 和 Rust 版本
3. 详细的错误信息
4. 重现步骤
5. 相关日志文件 