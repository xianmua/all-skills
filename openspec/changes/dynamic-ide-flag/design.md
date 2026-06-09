## Context

当前 `install` 命令使用硬编码参数：
```rust
#[arg(long)] pub trae: bool,
#[arg(long)] pub clion: bool,
#[arg(long)] pub comate: bool,
```

每次新增 IDE 都需要修改代码。

## Goals / Non-Goals

**Goals:**
- 支持动态 `--xxx` 参数
- 自动映射到 `.xxx/skills` 目录
- 与 `--dir` 参数互斥

**Non-Goals:**
- 不改变默认安装目录（`.agent/skills`）

## Decisions

### 使用 Clap 的 `raw` 或自定义解析

**方案 1:** 使用 `--ide <name>` 参数（显式）
```bash
yc-skills install fmt --ide trae
```

**方案 2:** 使用动态 `--xxx` 格式（用户期望）
```bash
yc-skills install fmt --trae
```

**决定:** 采用方案 2，使用 Clap 的 `raw(true)` 和自定义解析

**实现思路:**
```rust
// 捕获所有 --xxx 参数
#[arg(long, raw = true)]
pub ide: Option<String>,

// 在 execute 中解析参数名
```

**替代方案:** 使用 `value_parser` 动态验证参数名

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| 与 `--help` 等内置参数冲突 | 排除内置参数名 |
| 参数验证复杂 | 在代码中明确排除列表 |
