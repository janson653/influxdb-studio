#!/usr/bin/env node

import fs from 'fs';
import path from 'path';

const newVersion = process.argv[2];

if (!newVersion) {
  console.error('请提供版本号，例如: node scripts/sync-version.js 0.3.0');
  process.exit(1);
}

// 验证版本号格式
const versionRegex = /^\d+\.\d+\.\d+$/;
if (!versionRegex.test(newVersion)) {
  console.error('版本号格式错误，请使用 x.y.z 格式');
  process.exit(1);
}

const files = [
  'package.json',
  'src-tauri/Cargo.toml',
  'src-tauri/tauri.conf.json'
];

files.forEach(file => {
  if (!fs.existsSync(file)) {
    console.warn(`文件不存在: ${file}`);
    return;
  }

  const content = fs.readFileSync(file, 'utf8');
  let updated = false;

  if (file === 'package.json') {
    const packageJson = JSON.parse(content);
    if (packageJson.version !== newVersion) {
      packageJson.version = newVersion;
      fs.writeFileSync(file, JSON.stringify(packageJson, null, 2) + '\n');
      updated = true;
    }
  } else if (file === 'src-tauri/Cargo.toml') {
    const updatedContent = content.replace(/^version = ".*"$/m, `version = "${newVersion}"`);
    if (updatedContent !== content) {
      fs.writeFileSync(file, updatedContent);
      updated = true;
    }
  } else if (file === 'src-tauri/tauri.conf.json') {
    const tauriConfig = JSON.parse(content);
    if (tauriConfig.version !== newVersion) {
      tauriConfig.version = newVersion;
      fs.writeFileSync(file, JSON.stringify(tauriConfig, null, 2) + '\n');
      updated = true;
    }
  }

  if (updated) {
    console.log(`✅ 已更新 ${file} 版本为 ${newVersion}`);
  } else {
    console.log(`ℹ️  ${file} 版本已经是 ${newVersion}`);
  }
});

console.log('\n🎉 版本同步完成！'); 