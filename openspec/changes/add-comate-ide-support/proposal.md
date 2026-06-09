## Why

all-skills CLI 工具当前支持 Trae IDE 的安装目录 `.trae/skills`，需要扩展支持 Comate IDE，使用 `.comate/skills` 目录结构，保持与其他 IDE 安装路径的一致性。

## What Changes

- 为 `install` 命令添加 `--comate` 参数选项
- 安装路径：`<current-dir>/.comate/skills/<skill-name>/`
- 与现有 `--trae`、`--clion` 选项保持一致的使用体验

## Capabilities

### New Capabilities

- `comate-ide-support`: 支持 Comate IDE 的安装目录 `.comate/skills`

### Modified Capabilities

- `skill-install`: 修改 install 命令，添加 `--comate` 参数选项，安装到 `.comate/skills` 目录

## Impact

- 修改 `src/commands/install.rs` 中的 Install 结构体
- 添加 `--comate` CLI 参数
- 修改 `get_install_dir()` 方法支持 `.comate/skills` 路径