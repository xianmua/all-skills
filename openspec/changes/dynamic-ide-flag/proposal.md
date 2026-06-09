## Why

当前 `--trae`、`--clion`、`--comate` 是硬编码的参数，每次新增 IDE 都需要修改代码。需要改为动态参数，支持任意 `--xxx` 格式，自动映射到 `.xxx/skills` 目录。

## What Changes

- 将固定的 `--trae`、`--clion`、`--comate` 参数改为动态 `--<ide-name>` 参数
- 支持 `all-skills install fmt --trae` → `.trae/skills/fmt`
- 支持 `all-skills install fmt --comate` → `.comate/skills/fmt`
- 支持未来任意 IDE，无需修改代码

## Capabilities

### New Capabilities

- `dynamic-ide-flag`: 动态 IDE 参数，支持任意 `--xxx` 格式

### Modified Capabilities

- `skill-install`: 改用动态参数替代硬编码的 IDE 选项

## Impact

- 修改 `src/commands/install.rs`
- 简化参数定义，去除硬编码枚举
