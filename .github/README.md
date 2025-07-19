# GitHub Actions 工作流说明

本项目包含多个 GitHub Actions 工作流，用于自动化构建、测试和发布。

## 工作流概览

### 1. `build.yml` - 主要构建工作流

**触发条件**：
- 推送标签（`v*`）
- 推送到 `main` 分支的 PR
- 手动触发

**功能**：
- 多平台构建（macOS、Ubuntu、Windows）
- 自动发布到 GitHub Releases
- 智能 Flatpak 构建（仅 Ubuntu）

**优化特性**：
- 🚀 智能检测 `appstream-compose` 可用性
- 🔧 自动选择最佳 GNOME Platform 版本
- 📦 备用构建方案处理依赖缺失
- 📊 详细的构建状态报告

### 2. `flatpak.yml` - 专用 Flatpak 构建工作流

**触发条件**：
- 推送标签（`v*`）
- 推送到 `main` 分支的 PR
- 手动触发

**功能**：
- 多版本 GNOME Platform 并行构建
- 自动选择最佳版本作为主要发布
- 完整的包测试和验证

**矩阵构建**：
- GNOME Platform 版本：47, 46, 45, 44
- 每个版本独立构建和测试
- 选择最新稳定版本作为主要发布

### 3. `test.yml` - 测试工作流

**触发条件**：
- 推送到 `main` 或 `develop` 分支
- 推送到 `main` 分支的 PR

**功能**：
- 前端构建测试
- Rust 代码检查（check、clippy、test）
- Flatpak 配置验证（PR 时）

## 智能构建特性

### 🔍 环境检测
```yaml
# 检查 appstream-compose 可用性
if command -v appstream-compose &> /dev/null; then
  echo "✅ 使用标准构建流程"
  BUILD_METHOD="standard"
else
  echo "⚠️  使用备用构建流程"
  BUILD_METHOD="fallback"
fi
```

### 🔧 备用构建方案
当 `appstream-compose` 不可用时：
1. 备份原始配置文件
2. 修改配置避免使用 `appstream-compose`
3. 执行构建
4. 恢复原始配置

### 📦 多版本支持
```yaml
# 智能安装 GNOME Platform 运行时
flatpak install flathub org.gnome.Platform//47 org.gnome.Sdk//47 -y || \
flatpak install flathub org.gnome.Platform//46 org.gnome.Sdk//46 -y || \
flatpak install flathub org.gnome.Platform//45 org.gnome.Sdk//45 -y || \
flatpak install flathub org.gnome.Platform//44 org.gnome.Sdk//44 -y
```

## 构建产物

### 标准构建
- **macOS**: `.dmg` 文件
- **Windows**: `.msi` 文件
- **Linux**: `.AppImage` 文件

### Flatpak 构建
- **多版本**: `influxdb-studio-gnome{version}.flatpak`
- **最佳版本**: `influxdb-studio.flatpak`（自动选择）

## 错误处理

### 常见问题解决

1. **appstream-compose 缺失**
   - 自动检测并使用备用方案
   - 提供详细的错误信息

2. **GNOME Platform 版本不兼容**
   - 自动尝试多个版本
   - 矩阵构建确保兼容性

3. **构建失败**
   - 详细的错误日志
   - 构建状态报告

### 调试信息
```yaml
# 版本信息输出
flatpak-builder --version
flatpak --version
appstream-compose --version || echo "⚠️  appstream-compose 不可用"
```

## 使用建议

### 开发阶段
- 使用 `test.yml` 进行快速验证
- PR 时自动测试 Flatpak 配置

### 发布阶段
- 推送标签触发完整构建
- 自动发布到 GitHub Releases
- 多平台和多版本支持

### 手动触发
```bash
# 手动触发构建
gh workflow run build.yml

# 手动触发 Flatpak 构建
gh workflow run flatpak.yml

# 手动触发测试
gh workflow run test.yml
```

## 性能优化

### 缓存策略
- Node.js 依赖缓存
- Rust 构建缓存
- 工作空间优化

### 并行构建
- 多平台并行构建
- 多版本 GNOME Platform 并行构建
- 独立的测试和构建步骤

## 监控和维护

### 构建状态
- 实时构建状态报告
- 详细的错误信息
- 包大小和构建时间统计

### 版本管理
- 自动版本标签
- 多版本兼容性测试
- 最佳版本选择

## 故障排除

### 构建失败
1. 检查构建日志中的详细错误信息
2. 验证依赖安装是否正确
3. 确认配置文件语法

### Flatpak 问题
1. 检查 `appstream-compose` 可用性
2. 验证 GNOME Platform 版本兼容性
3. 查看备用构建方案日志

### 发布问题
1. 确认 GitHub Token 权限
2. 检查 Release 创建权限
3. 验证文件上传路径 