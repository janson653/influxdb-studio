#!/usr/bin/env node

import fs from 'fs';

const files = [
  { path: 'package.json', key: 'version' },
  { path: 'src-tauri/Cargo.toml', key: 'version' },
  { path: 'src-tauri/tauri.conf.json', key: 'version' }
];

const versions = {};

files.forEach(({ path: filePath, key }) => {
  if (!fs.existsSync(filePath)) {
    console.warn(`⚠️  文件不存在: ${filePath}`);
    return;
  }

  const content = fs.readFileSync(filePath, 'utf8');
  let version;

  if (filePath === 'package.json') {
    const packageJson = JSON.parse(content);
    version = packageJson[key];
  } else if (filePath === 'src-tauri/Cargo.toml') {
    const match = content.match(/^version = "([^"]+)"/m);
    version = match ? match[1] : null;
  } else if (filePath === 'src-tauri/tauri.conf.json') {
    const tauriConfig = JSON.parse(content);
    version = tauriConfig[key];
  }

  if (version) {
    versions[filePath] = version;
    console.log(`📄 ${filePath}: ${version}`);
  } else {
    console.error(`❌ ${filePath}: 无法获取版本号`);
  }
});

const uniqueVersions = [...new Set(Object.values(versions))];

if (uniqueVersions.length === 1) {
  console.log(`\n✅ 所有文件版本一致: ${uniqueVersions[0]}`);
} else {
  console.log(`\n❌ 版本不一致:`);
  uniqueVersions.forEach(version => {
    const filesWithVersion = Object.entries(versions)
      .filter(([_, v]) => v === version)
      .map(([file]) => file);
    console.log(`   ${version}: ${filesWithVersion.join(', ')}`);
  });
  process.exit(1);
} 