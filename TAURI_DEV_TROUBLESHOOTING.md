# Tauri 开发环境故障排除

## 问题描述

启动 Tauri 开发环境时遇到符号链接循环错误：

```
Error: ELOOP: too many symbolic links encountered, stat '/home/janson/codebase/influxdb-studio/flatpak/build/var/run/udev/watch/1'
```

## 根本原因

**问题根源**：Flatpak 构建过程中创建的临时文件系统包含大量符号链接，这些链接形成了循环引用，导致 Vite 的文件监视器无法正常工作。

**具体原因**：
1. Flatpak 构建目录 (`flatpak/build/`) 包含复杂的文件系统结构
2. 其中包含大量的符号链接，特别是 `var/run/udev/watch/` 目录
3. Vite 的文件监视器尝试监视这些目录时遇到符号链接循环
4. 导致 `ELOOP` 错误，开发服务器启动失败

## 解决方案

### 方案一：清理 Flatpak 构建目录（推荐）

```bash
# 清理 Flatpak 构建相关目录
rm -rf flatpak/build
rm -rf flatpak/repo
rm -rf flatpak/.flatpak-builder

# 清理 Vite 缓存
rm -rf node_modules/.vite
```

### 方案二：使用修复脚本

```bash
# 运行自动修复脚本
./scripts/fix-tauri-dev.sh
```

### 方案三：优化 Vite 配置

在 `vite.config.ts` 中添加文件监视器忽略规则：

```typescript
server: {
  port: 1420,
  strictPort: true,
  watch: {
    ignored: [
      "**/src-tauri/**",
      "**/flatpak/build/**",      // 忽略 Flatpak 构建目录
      "**/flatpak/repo/**",       // 忽略 Flatpak 仓库目录
      "**/flatpak/.flatpak-builder/**", // 忽略 Flatpak 构建缓存
      "**/node_modules/**"
    ],
  },
},
```

### 方案四：使用改进的启动脚本

```bash
# 使用改进的启动脚本（自动清理）
./scripts/start-tauri-improved.sh
```

## 预防措施

### 1. 更新 .gitignore

添加以下规则到 `.gitignore`：

```gitignore
# Flatpak build artifacts
flatpak/build/
flatpak/repo/
flatpak/.flatpak-builder/
flatpak/*.flatpak
```

### 2. 开发工作流建议

- **开发前**：清理 Flatpak 构建目录
- **开发中**：避免同时运行 Flatpak 构建和 Tauri 开发
- **开发后**：定期清理构建缓存

### 3. 自动化脚本

创建自动化脚本来管理开发环境：

```bash
#!/bin/bash
# 开发环境准备脚本

# 清理 Flatpak 构建目录
rm -rf flatpak/build flatpak/repo flatpak/.flatpak-builder

# 清理缓存
rm -rf node_modules/.vite

# 启动开发环境
pnpm tauri dev
```

## 错误诊断

### 检查符号链接

```bash
# 查找项目中的符号链接
find . -type l -not -path "./node_modules/*" -not -path "./.git/*"

# 检查 Flatpak 构建目录的符号链接
ls -la flatpak/build/var/run/udev/watch/
```

### 检查端口占用

```bash
# 检查端口 1420 是否被占用
lsof -ti:1420

# 释放端口
lsof -ti:1420 | xargs kill -9
```

### 检查 Vite 配置

```bash
# 验证 Vite 配置语法
npx vite --config vite.config.ts --mode development
```

## 常见问题

### Q1: 为什么会出现符号链接循环？

**A**: Flatpak 构建过程中会创建一个完整的文件系统沙盒，包含 `/var/run/udev/watch/` 等系统目录，这些目录包含大量的符号链接用于设备监控。

### Q2: 如何避免这个问题？

**A**: 
1. 在开发前清理 Flatpak 构建目录
2. 在 Vite 配置中忽略 Flatpak 相关目录
3. 使用自动化脚本管理开发环境

### Q3: 是否会影响生产构建？

**A**: 不会。这个问题只影响开发环境的文件监视器，生产构建不受影响。

### Q4: 如何验证修复是否成功？

**A**: 
1. 清理 Flatpak 构建目录
2. 启动 Tauri 开发环境
3. 检查是否还有 `ELOOP` 错误

## 最佳实践

### 开发环境管理

1. **分离构建环境**：开发时避免同时运行 Flatpak 构建
2. **定期清理**：定期清理构建缓存和临时文件
3. **使用脚本**：使用自动化脚本管理开发环境

### 配置优化

1. **Vite 配置**：正确配置文件监视器忽略规则
2. **Git 忽略**：将构建产物添加到 `.gitignore`
3. **环境变量**：正确设置开发环境变量

### 故障排除流程

1. **识别问题**：查看错误日志，确定问题类型
2. **清理环境**：清理相关的构建目录和缓存
3. **验证修复**：重新启动开发环境
4. **预防措施**：实施预防措施避免再次发生

## 相关文件

- `scripts/fix-tauri-dev.sh` - 自动修复脚本
- `scripts/start-tauri-improved.sh` - 改进的启动脚本
- `vite.config.ts` - Vite 配置文件
- `.gitignore` - Git 忽略规则 