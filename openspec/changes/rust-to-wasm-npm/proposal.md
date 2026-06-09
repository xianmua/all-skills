## Why

当前 `all-skills` 是纯 Rust CLI 工具，只能通过 cargo编译安装。将其转换为 WebAssembly (WASM) 模块并发布到 npm，可以让 JavaScript/TypeScript开发者无需安装 Rust 工具链即可使用该工具，同时支持在浏览器环境中运行。

## What Changes

- 添加 `wasm-bindgen` 依赖，将核心逻辑编译为 WASM 模块
- 创建 npm兼容的 JavaScript/TypeScript wrapper
- 配置 wasm-pack 构建流程，生成 ESM 和 CJS 双格式
- 发布到 npm，支持 `@your-scope/all-skills` 或类似命名空间
- 保留原有 CLI 功能作为可选功能

## Capabilities

### New Capabilities

- `wasm-compilation`: 将 Rust 核心逻辑编译为 WASM 模块，支持 Node.js 和浏览器环境
- `npm-package`: 创建符合 npm 规范的包结构，包含 ESM/CJS 双格式和 TypeScript 类型定义
- `js-wrapper`: 提供 JavaScript/TypeScript API 封装，简化集成

### Modified Capabilities

- (无)

## Impact

- **新增依赖**: `wasm-bindgen`, `wasm-pack`
- **构建系统**: 添加 wasm-pack 构建流程
- **发布目标**: crates.io (原有) + npm (新增)
- **影响范围**: CLI 入口点、WASM 编译配置、包发布配置