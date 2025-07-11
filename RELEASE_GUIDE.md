# 发布指南

## 🚀 使用 GitHub Actions 构建跨平台版本

### 支持的平台

- **Windows**: x86_64-pc-windows-msvc (.exe, .msi)
- **macOS**: universal-apple-darwin (.dmg, .app)
- **Linux**: x86_64-unknown-linux-gnu (.deb, .rpm, .AppImage)

### 发布流程

#### 1. 创建发布版本

```bash
# 1. 确保代码已提交并推送
git add .
git commit -m "准备发布 v0.1.0"
git push origin main

# 2. 创建并推送标签
git tag v0.1.0
git push origin v0.1.0
```

#### 2. 自动构建

推送标签后，GitHub Actions 会自动：
- 在 Ubuntu、macOS 和 Windows 上构建应用
- 生成所有平台的安装包
- 创建 GitHub Release
- 上传构建产物

#### 3. 手动触发构建

如果需要手动触发构建：
1. 访问 GitHub 仓库的 Actions 页面
2. 选择 "Build and Release" 工作流
3. 点击 "Run workflow"
4. 选择分支并运行

### 构建产物

每个平台会生成以下文件：

#### Windows
- `influxdb-studio_0.1.0_x64_en-US.msi` - MSI 安装包
- `influxdb-studio_0.1.0_x64-setup.exe` - 安装程序

#### macOS
- `influxdb-studio_0.1.0_universal.dmg` - DMG 镜像
- `influxdb-studio.app.tar.gz` - 应用包

#### Linux
- `influxdb-studio_0.1.0_amd64.deb` - Debian 包
- `influxdb-studio-0.1.0-1.x86_64.rpm` - RPM 包
- `influxdb-studio_0.1.0_amd64.AppImage` - AppImage

### 配置签名（可选）

为了让用户信任您的应用，建议配置代码签名：

#### Windows 签名
```bash
# 在 GitHub Secrets 中添加：
WINDOWS_CERTIFICATE: <base64 encoded certificate>
WINDOWS_CERTIFICATE_PASSWORD: <certificate password>
```

#### macOS 签名
```bash
# 在 GitHub Secrets 中添加：
APPLE_CERTIFICATE: <base64 encoded certificate>
APPLE_CERTIFICATE_PASSWORD: <certificate password>
APPLE_SIGNING_IDENTITY: <signing identity>
APPLE_ID: <apple id>
APPLE_PASSWORD: <app-specific password>
```

### 更新器配置

如果启用了自动更新功能：

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

### 故障排除

#### 构建失败
1. 检查 Actions 日志
2. 确保所有依赖都已正确安装
3. 验证 Rust 和 Node.js 版本兼容性

#### 权限问题
确保 GitHub Token 有足够的权限：
- `contents: write` - 创建发布
- `packages: write` - 上传构建产物

#### 平台特定问题
- **Windows**: 确保 Visual Studio Build Tools 已安装
- **macOS**: 确保 Xcode Command Line Tools 已安装
- **Linux**: 确保所有系统依赖都已安装

### 版本管理

推荐使用语义化版本：
- `v1.0.0` - 主要版本
- `v1.1.0` - 次要版本
- `v1.1.1` - 补丁版本

### 发布检查清单

发布前确认：
- [ ] 代码已测试
- [ ] 版本号已更新
- [ ] 更新日志已写好
- [ ] 所有平台构建成功
- [ ] 安装包可以正常安装
- [ ] 应用可以正常运行

---

## 🎉 恭喜！

您的应用现在可以自动构建并发布到所有主要平台了！ 