## Why

当前企业内部缺乏统一的 skill 包管理工具。开发者需要手动下载、配置和管理各种 skill（如 openspec、code-review 等），流程繁琐且难以维护。需要一个类似 npm/pip 的 CLI 工具来简化 skill 的安装、更新和卸载流程。

## What Changes

- 创建 `all-skills` CLI 工具（Rust 实现），提供跨平台的 skill 包管理能力
- 支持从企业 GitLab/GitHub/Gitee 等仓库查询和安装 skill
- 支持按 IDE 类型（如 `--trae`）安装到对应目录结构
- 支持 skill 的卸载和更新操作
- 支持添加多个 git 仓库来源

## Capabilities

### New Capabilities

- `skill-registry`: 支持配置多个 git 仓库来源（GitLab/GitHub/Gitee），支持仓库的增删改查
- `skill-install`: 从配置的仓库查询并下载 skill，支持按 IDE 类型安装到正确目录
- `skill-uninstall`: 卸载已安装的 skill，清理相关文件
- `skill-update`: 检查并更新已安装的 skill 到最新版本
- `skill-search`: 在配置的仓库中搜索可用的 skill

## Impact

- 新增 Rust 项目：`all-skills`
- 新增 CLI 命令：`install`、`uninstall`、`update`、`search`、`add-origin`、`list-origins`、`remove-origin`
- 配置文件：`~/.all-skills/config.toml`（存储仓库来源配置）
- 安装目录：`.trae/skills/`（参考 openspec 目录结构）