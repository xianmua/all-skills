## Context

当前 openspec 等 skill 工具没有统一的包管理机制，用户需要手动克隆仓库、复制文件。`yc-skills` 将成为一个类似 npm/yarn 的 CLI 工具，用于管理 skill 包的安装、更新和卸载。

参考 openspec 的目录结构：`.trae/skills/<skill-name>/`，每个 skill 下有 `skill.yaml` 或 `manifest.json` 描述 skill 元信息。

## Goals / Non-Goals

**Goals:**
- 提供 `install`、`uninstall`、`update`、`search` 等核心命令
- 支持多个 git 仓库来源配置
- 自动识别 IDE 类型（如 `--trae`）安装到对应目录
- 跨平台支持（Windows、macOS、Linux）

**Non-Goals:**
- 不实现 skill 的运行时执行
- 不实现依赖解析（skill 之间的依赖关系）
- 不实现私有认证（未来可扩展）

## Decisions

### 1. 使用 Rust 实现 CLI 工具

**决定：** 采用 Rust + Clap 框架实现

**理由：**
- 静态编译，单二进制分发，无需运行时依赖
- 性能优秀，适合频繁的 IO 操作（git clone、文件操作）
- 跨平台支持好

**替代方案：**
- Node.js/TypeScript：需要 Node 环境，分发不便
- Python：启动慢，依赖管理复杂

### 2. 配置文件存储在 `~/.yc-skills/config.toml`

**决定：** 使用 TOML 格式存储配置

**理由：**
- 人类可读，易于手动编辑
- Rust 有成熟的 `toml` crate 支持
- 简单场景无需复杂格式

**配置结构：**
```toml
[origins]
gitlab = "https://gitlab.internal.company.com/skills"
github = "https://github.com/company/skills"
gitee = "https://gitee.com/company/skills"

[defaults]
ide = "trae"  # 默认 IDE 类型
```

### 3. Skill 目录结构

**决定：** 遵循 openspec 约定，skill 安装到 `.trae/skills/<skill-name>/`

**理由：**
- 与现有工具保持一致
- 用户体验统一

**每个 skill 包含：**
```
.trae/skills/<skill-name>/
├── skill.yaml           # skill 元信息（名称、版本、描述）
├── README.md            # 文档
├── src/                  # 源代码（如果有）
├── configs/              # 配置文件
└── ...                   # 其他资源
```

### 4. Git 仓库查询策略

**决定：** 通过 git ls-remote 或 API 获取仓库文件列表

**理由：**
- 无需完整的 git clone，节省带宽
- 可快速检查 skill 是否存在

**流程：**
1. 遍历配置的 origins（按优先级）
2. 对每个 origin 执行 `git ls-remote` 获取 refs
3. 检查是否存在 `<skill-name>/skill.yaml` 或 `<skill-name>/manifest.json`
4. 存在则下载

### 5. CLI 命令设计

```
yc-skills install <name> [--trae|--clion]   # 安装 skill
yc-skills uninstall <name>                    # 卸载 skill
yc-skills update <name>                       # 更新 skill
yc-skills search <keyword>                    # 搜索 skill
yc-skills list                               # 列出已安装的 skill
yc-skills add-origin <url> [--name <name>]   # 添加仓库来源
yc-skills list-origins                       # 列出配置的来源
yc-skills remove-origin <name>               # 移除仓库来源
```

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| Git 仓库网络不通 | 添加 `--offline` 模式，使用本地缓存 |
| 仓库结构不统一 | 在 skill.yaml 中定义标准字段，支持向后兼容 |
| 权限问题 | 配置文件和安装目录使用 `dirs` crate 获取标准路径 |
| 多平台路径差异 | 使用 `path` crate 处理跨平台路径 |

## Open Questions

1. 是否需要实现 skill 的版本锁定文件（如 `package-lock.json`）？
2. 是否需要支持离线安装（预先下载的 tarball）？
3. 企业内网 GitLab 是否需要认证？