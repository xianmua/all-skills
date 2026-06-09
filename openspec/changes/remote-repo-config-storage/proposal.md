## Why

当用户使用 `all-skills add-origin` 添加远程仓库时，需要将配置持久化保存。目前配置存储在用户系统目录 `~/.all-skills/config.toml` 中，但需要确保配置的读写逻辑清晰、可测试，并且支持多平台。

## What Changes

- 明确配置文件的存储路径规则（跨平台兼容）
- 实现配置的加载/保存逻辑
- 添加配置的验证机制（URL格式、必填字段等）
- 提供默认配置初始化功能
- 确保配置变更后能正确写回文件

## Capabilities

### New Capabilities

- `config-storage`: 配置的持久化存储，支持 TOML 格式读写
- `config-validation`: 配置验证，确保 origin URL 格式正确
- `config-defaults`: 默认配置生成，包含预置的 skills 源

### Modified Capabilities

- (无)

## Impact

- **受影响模块**: `src/models/config.rs`, `src/config/mod.rs`
- **新增依赖**: 无
- **配置文件**: `~/.all-skills/config.toml`
- **影响范围**: 所有需要读写配置的 CLI 命令（add-origin, list-origins, install 等）