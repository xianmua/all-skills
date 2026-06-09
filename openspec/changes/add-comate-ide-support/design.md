## Context

all-skills CLI 当前支持 `--trae` 和 `--clion` 两个 IDE 选项，用户需要在 Comate IDE 环境下使用相同的方式安装 skills。

现有代码结构：
- `install --trae` → `.trae/skills/<name>/`
- `install --clion` → `.clion/skills/<name>/`
- 需要新增 `install --comate` → `.comate/skills/<name>/`

## Goals / Non-Goals

**Goals:**
- 添加 `--comate` 参数支持
- 安装路径为 `<current-dir>/.comate/skills/<skill-name>/`
- 保持与现有 `--trae`、`--clion` 一致的使用体验

**Non-Goals:**
- 不修改 uninstall、update 等其他命令的逻辑（其他命令会自动识别目录）
- 不添加额外的配置项

## Decisions

### 1. 添加 `--comate` 参数

**决定：** 在 `Install` 结构体中添加 `#[arg(long)] comate: bool`

**理由：**
- 与现有参数风格一致
- Clap 支持自动生成冲突检查（conflicts_with）

**实现：**
```rust
/// 安装到 .comate/skills 目录 (Comate IDE)
#[arg(long, conflicts_with_all = ["dir", "trae", "clion"])]
pub comate: bool,
```

### 2. 修改安装目录逻辑

**决定：** 在 `get_install_dir()` 方法中添加对 `.comate` 的判断

**理由：**
- 保持现有代码风格一致
- 最小化代码改动

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| 用户同时指定多个 IDE 参数 | 使用 `conflicts_with_all` 确保互斥 |
| 未来需要支持更多 IDE | 考虑重构为枚举类型（后续迭代） |

## Open Questions

1. 是否需要更新默认 IDE 配置？
2. `.comate` 目录是否需要预先创建？（安装时会自动创建）