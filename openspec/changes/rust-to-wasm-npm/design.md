## Context

`yc-skills` 是一个 Rust 编写的 CLI 工具，用于管理 skill 包。当前的安装方式需要用户具备 Rust 工具链（cargo）。为了扩大用户群体，特别是前端和 Node.js 开发者生态，需要将其打包为 npm 包发布。

## Goals / Non-Goals

**Goals:**
- 将核心逻辑编译为 WASM 模块，支持 Node.js 环境运行
- 生成符合 npm 规范的包（ESM + CJS + TypeScript 类型）
- 保持 API 向后兼容，已有 CLI 用户无感知
- 支持浏览器环境运行（扩展使用场景）

**Non-Goals:**
- 不重构 Rust 代码逻辑，只做 WASM 适配
- 不移除原有的 cargo 构建方式
- 不提供浏览器专用的 UI组件

## Decisions

### 1. 使用 wasm-bindgen + wasm-pack 方案

**选择**: wasm-bindgen 生态
**原因**:
- wasm-bindgen 提供 Rust 和 JavaScript 之间的高效绑定
- wasm-pack 封装了编译、测试、打包的完整流程
- 支持 `--target nodejs` 直接生成 Node.js 可用模块

**替代方案考虑**:
- `wasm-bindgen` + `wasm-pack` ✅ (选择)
- `wasmer` / `wasmtime` (需要运行时，不适合 npm 发布)
- 纯手写 WASM 接口 (工作量大，不推荐)

### 2. 双格式输出 (ESM + CJS)

**选择**: 同时输出 ESM 和 CJS
**原因**:
- ESM 支持现代打包工具（Vite、esbuild、Rollup）
- CJS 保证 Node.js 兼容性和旧项目支持
- wasm-pack 默认支持多格式输出

### 3. 包结构

```
yc-skills-wasm/
├── dist/
│   ├── index.js        # CJS
│   ├── index.mjs       # ESM
│   └── index.d.ts      # TypeScript 类型
├── package.json
└── README.md
```

## Risks / Trade-offs

- **[风险]** WASM 二进制文件体积可能较大
  - **缓解**: 使用 `wasm-pack build --release --optimize-size`，开启 LTO 和 wasm-opt

- **[风险]** wasm-bindgen 依赖版本冲突
  - **缓解**: 锁定 wasm-bindgen 版本，避免主版本升级

- **[风险]** npm 发布需要组织命名空间
  - **缓解**: 使用 `@yc/skills-wasm` 或类似 scope名称

- **[权衡]** 首次 wasm-pack build 需要安装 wasm32目标平台
  - **缓解**: 在 README 中说明，或使用预编译的 CI 流程

## Migration Plan

1. 创建 `wasm-pack` 构建配置
2. 添加 npm scripts (build:wasm, publish:npm)
3. 配置 CI/CD流程自动发布
4. 发布首个版本到 npm
5. 更新文档说明新的使用方式