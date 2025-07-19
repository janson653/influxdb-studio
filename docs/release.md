# 发布指南

## 概述

InfluxDB Studio 使用 GitHub Actions 进行自动化构建和发布，支持 Windows、macOS 和 Linux 三个主要平台。

## 支持的平台

- **Windows**: x86_64-pc-windows-msvc (.exe, .msi)
- **macOS**: universal-apple-darwin (.dmg, .app)
- **Linux**: x86_64-unknown-linux-gnu (.deb, .rpm, .AppImage, .flatpak)

## 发布流程

### 1. 准备发布

```bash
# 确保代码已提交并推送
git add .
git commit -m "准备发布 v0.1.0"
git push origin main
```

### 2. 创建发布版本

```bash
# 创建并推送标签
git tag v0.1.0
git push origin v0.1.0
```

### 3. 自动构建

推送标签后，GitHub Actions 会自动：
- 在 Ubuntu、macOS 和 Windows 上构建应用
- 生成所有平台的安装包
- 创建 GitHub Release
- 上传构建产物

### 4. 手动触发构建

如果需要手动触发构建：
1. 访问 GitHub 仓库的 Actions 页面
2. 选择 "Build and Release" 工作流
3. 点击 "Run workflow"
4. 选择分支并运行

## 构建产物

### Windows
- `influxdb-studio_0.1.0_x64_en-US.msi` - MSI 安装包
- `influxdb-studio_0.1.0_x64-setup.exe` - 安装程序

### macOS
- `influxdb-studio_0.1.0_universal.dmg` - DMG 镜像
- `influxdb-studio.app.tar.gz` - 应用包

### Linux
- `influxdb-studio_0.1.0_amd64.deb` - Debian 包
- `influxdb-studio-0.1.0-1.x86_64.rpm` - RPM 包
- `influxdb-studio_0.1.0_amd64.AppImage` - AppImage
- `influxdb-studio.flatpak` - Flatpak 包

## GitHub Actions 工作流

### 主要工作流

#### 1. `build.yml` - 主要构建工作流

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

#### 2. `flatpak.yml` - 专用 Flatpak 构建工作流

**触发条件**：
- 推送标签（`v*`）
- 推送到 `main` 分支的 PR
- 手动触发

**功能**：
- 多版本 GNOME Platform 并行构建
- 自动选择最佳版本作为主要发布
- 完整的包测试和验证

**矩阵构建**：
- GNOME Platform 版本：47
- 每个版本独立构建和测试
- 选择最新稳定版本作为主要发布

#### 3. `test.yml` - 测试工作流

**触发条件**：
- 推送到 `main` 或 `develop` 分支
- 推送到 `main` 分支的 PR

**功能**：
- 前端构建测试
- Rust 代码检查（check、clippy、test）
- Flatpak 配置验证（PR 时）

### 智能构建特性

#### 环境检测
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

#### 备用构建方案
当 `appstream-compose` 不可用时：
1. 备份原始配置文件
2. 修改配置避免使用 `appstream-compose`
3. 执行构建
4. 恢复原始配置

#### 多版本支持
```yaml
# 智能安装 GNOME Platform 运行时
flatpak install flathub org.gnome.Platform//47 org.gnome.Sdk//47 -y
```

## 代码签名配置

### Windows 签名

```bash
# 在 GitHub Secrets 中添加：
WINDOWS_CERTIFICATE: <base64 encoded certificate>
WINDOWS_CERTIFICATE_PASSWORD: <certificate password>
```

### macOS 签名

```bash
# 在 GitHub Secrets 中添加：
APPLE_CERTIFICATE: <base64 encoded certificate>
APPLE_CERTIFICATE_PASSWORD: <certificate password>
APPLE_SIGNING_IDENTITY: <signing identity>
APPLE_ID: <apple id>
APPLE_PASSWORD: <app-specific password>
```

## 自动更新配置

### 更新器设置

1. 在 `tauri.conf.json` 中配置更新器：
```json
{
  "updater": {
    "active": true,
    "endpoints": [
      "https://github.com/your-username/influxdb-ui/releases/latest/download/latest.json"
    ]
  }
}
```

2. 在 GitHub Secrets 中添加更新器密钥：
```bash
TAURI_PRIVATE_KEY: <private key>
TAURI_KEY_PASSWORD: <key password>
```

## 版本管理

### 语义化版本

推荐使用语义化版本：
- `v1.0.0` - 主要版本（重大功能更新）
- `v1.1.0` - 次要版本（新功能添加）
- `v1.1.1` - 补丁版本（Bug 修复）

### 版本号更新

1. 更新 `package.json` 中的版本号
2. 更新 `src-tauri/Cargo.toml` 中的版本号
3. 更新 `src-tauri/tauri.conf.json` 中的版本号
4. 创建版本标签

## 发布检查清单

发布前确认：
- [ ] 代码已测试
- [ ] 版本号已更新
- [ ] 更新日志已写好
- [ ] 所有平台构建成功
- [ ] 安装包可以正常安装
- [ ] 应用可以正常运行
- [ ] 代码签名配置正确
- [ ] 自动更新功能正常

## 故障排除

### 构建失败

1. **检查 Actions 日志**
   - 查看详细的错误信息
   - 确认构建环境配置

2. **验证依赖**
   - 确保所有依赖都已正确安装
   - 验证 Rust 和 Node.js 版本兼容性

3. **平台特定问题**
   - **Windows**: 确保 Visual Studio Build Tools 已安装
   - **macOS**: 确保 Xcode Command Line Tools 已安装
   - **Linux**: 确保所有系统依赖都已安装

### 权限问题

确保 GitHub Token 有足够的权限：
- `contents: write` - 创建发布
- `packages: write` - 上传构建产物

### 常见错误

#### appstream-compose 缺失
- 自动检测并使用备用方案
- 提供详细的错误信息

#### GNOME Platform 版本不兼容
- 自动尝试多个版本
- 矩阵构建确保兼容性

#### 构建失败
- 详细的错误日志
- 构建状态报告

### 调试信息

```yaml
# 版本信息输出
flatpak-builder --version
flatpak --version
appstream-compose --version || echo "⚠️  appstream-compose 不可用"
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