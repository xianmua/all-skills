## 1. 项目配置

- [x] 1.1 清理 WASM 相关配置
- [x] 1.2 创建 package.json (npm CLI 包配置)
- [x] 1.3 创建 bin/all-skills.js (跨平台 wrapper)

## 2. 构建与发布

- [ ] 2.1 本地构建测试 (cargo build --release)
- [ ] 2.2 GitHub Actions 构建多平台发布 (可选)
- [ ] 2.3登录 npm 账号 (npm login)
- [ ] 2.4 发布到 npm (npm publish --access public)

## 3.验证

- [ ] 3.1 测试 npx @yc/skills-wasm --version
- [ ] 3.2 测试 npm install 后 all-skills 命令可用