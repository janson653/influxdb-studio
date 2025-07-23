#!/bin/bash

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🚀 开始运行测试套件...${NC}"

# 检查是否在正确的目录
if [ ! -f "package.json" ]; then
    echo -e "${RED}❌ 错误: 请在项目根目录运行此脚本${NC}"
    exit 1
fi

# 安装依赖（如果需要）
if [ ! -d "node_modules" ]; then
    echo -e "${YELLOW}📦 安装前端依赖...${NC}"
    npm install
fi

# 运行前端测试
echo -e "${BLUE}🧪 运行前端测试...${NC}"
npm run test:run

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ 前端测试通过${NC}"
else
    echo -e "${RED}❌ 前端测试失败${NC}"
    FRONTEND_TESTS_FAILED=true
fi

# 运行前端测试覆盖率
echo -e "${BLUE}📊 生成前端测试覆盖率报告...${NC}"
npm run test:coverage

# 运行后端测试
echo -e "${BLUE}🔧 运行后端测试...${NC}"
cd src-tauri
cargo test

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ 后端测试通过${NC}"
else
    echo -e "${RED}❌ 后端测试失败${NC}"
    BACKEND_TESTS_FAILED=true
fi

cd ..

# 总结
echo -e "${BLUE}📋 测试结果总结:${NC}"

if [ "$FRONTEND_TESTS_FAILED" = true ] || [ "$BACKEND_TESTS_FAILED" = true ]; then
    echo -e "${RED}❌ 部分测试失败${NC}"
    exit 1
else
    echo -e "${GREEN}🎉 所有测试通过!${NC}"
fi

echo -e "${BLUE}✨ 测试完成${NC}" 