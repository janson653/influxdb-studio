# 测试文档

## 概述

本项目包含完整的测试套件，涵盖前端和后端代码。测试采用分层策略，确保代码质量和功能可靠性。

## 测试架构

### 前端测试 (Vue 3 + TypeScript)
- **测试框架**: Vitest
- **组件测试**: Vue Test Utils
- **覆盖率**: V8 Coverage
- **环境**: jsdom

### 后端测试 (Rust)
- **测试框架**: Rust 内置测试
- **Mock**: mockall
- **异步测试**: tokio

## 运行测试

### 快速开始

```bash
# 运行所有测试
./scripts/run-tests.sh

# 或分别运行
npm run test:run        # 前端测试
npm run test:coverage   # 前端测试覆盖率
cd src-tauri && cargo test  # 后端测试
```

### 前端测试命令

```bash
# 开发模式运行测试
npm run test

# 运行测试并生成覆盖率报告
npm run test:coverage

# 使用UI界面运行测试
npm run test:ui

# 单次运行测试
npm run test:run
```

### 后端测试命令

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_list_databases_success

# 运行测试并显示输出
cargo test -- --nocapture

# 运行测试并生成覆盖率报告
cargo tarpaulin
```

## 测试策略

### 单元测试

#### 前端单元测试
- **Store测试**: 测试Pinia store的状态管理和操作
- **组件测试**: 测试Vue组件的渲染和交互
- **工具函数测试**: 测试纯函数和工具方法

#### 后端单元测试
- **命令测试**: 测试Tauri命令的实现
- **数据库操作测试**: 测试InfluxDB操作
- **错误处理测试**: 测试异常情况处理

### 集成测试
- **API集成**: 测试前后端通信
- **数据流测试**: 测试完整的数据操作流程

### 端到端测试
- **用户操作流程**: 测试完整的用户操作场景
- **错误恢复**: 测试错误情况下的恢复机制

## 测试文件结构

```
src/
├── tests/
│   ├── setup.ts                    # 测试环境设置
│   ├── stores/                     # Store测试
│   │   ├── databaseStore.test.ts
│   │   └── measurementStore.test.ts
│   └── components/                 # 组件测试
│       ├── DatabaseList.test.ts
│       └── CreateDatabaseDialog.test.ts

src-tauri/
├── tests/                          # 后端测试
│   ├── database_commands_tests.rs
│   └── measurement_commands_tests.rs
└── src/
    └── commands/                   # 命令实现
```

## 测试最佳实践

### 前端测试

1. **组件测试**
   ```typescript
   // 测试组件渲染
   it('should render correctly', () => {
     const wrapper = mount(Component)
     expect(wrapper.find('.class').exists()).toBe(true)
   })

   // 测试用户交互
   it('should handle user interaction', async () => {
     const wrapper = mount(Component)
     await wrapper.find('button').trigger('click')
     expect(wrapper.emitted('event')).toBeTruthy()
   })
   ```

2. **Store测试**
   ```typescript
   // 测试状态变化
   it('should update state', async () => {
     const store = useStore()
     await store.action()
     expect(store.state).toEqual(expectedValue)
   })
   ```

### 后端测试

1. **Mock测试**
   ```rust
   #[tokio::test]
   async fn test_function() {
       let mut mock = MockClient::new();
       mock.expect_method()
           .times(1)
           .returning(|| Ok(result));
       
       let result = function(&mock).await;
       assert!(result.is_ok());
   }
   ```

2. **错误处理测试**
   ```rust
   #[test]
   fn test_error_handling() {
       let result = function_that_may_fail();
       match result {
           Ok(_) => panic!("Expected error"),
           Err(e) => assert_eq!(e.to_string(), "expected error message")
       }
   }
   ```

## 覆盖率要求

- **前端覆盖率**: 目标 > 80%
- **后端覆盖率**: 目标 > 90%
- **关键路径**: 100% 覆盖

## 持续集成

测试在以下情况下自动运行：
- 代码提交
- Pull Request
- 发布前检查

## 故障排除

### 常见问题

1. **测试环境问题**
   ```bash
   # 清理并重新安装依赖
   rm -rf node_modules package-lock.json
   npm install
   ```

2. **Mock问题**
   ```typescript
   // 确保正确导入mock
   vi.mock('@/stores/store', () => ({
     useStore: vi.fn()
   }))
   ```

3. **异步测试问题**
   ```typescript
   // 使用await等待异步操作
   await wrapper.vm.$nextTick()
   ```

### 调试技巧

1. **前端测试调试**
   ```bash
   # 使用UI模式调试
   npm run test:ui
   ```

2. **后端测试调试**
   ```bash
   # 显示详细输出
   cargo test -- --nocapture
   ```

## 测试数据

### 测试数据库
- 使用独立的测试数据库
- 每次测试后清理数据
- 使用固定的测试数据集

### Mock数据
- 预定义的测试数据
- 可配置的mock响应
- 覆盖各种边界情况

## 性能测试

### 前端性能
- 组件渲染性能
- 内存泄漏检测
- 用户交互响应时间

### 后端性能
- API响应时间
- 数据库查询性能
- 内存使用情况

## 安全测试

- 输入验证测试
- SQL注入防护
- 权限控制测试
- 数据加密验证

## 维护

### 测试维护
- 定期更新测试依赖
- 重构时同步更新测试
- 监控测试覆盖率变化

### 文档维护
- 及时更新测试文档
- 记录测试策略变更
- 维护测试最佳实践 