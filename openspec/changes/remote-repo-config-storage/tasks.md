## 1. 配置存储核心

- [x] 1.1 确认 Config::default_path() 使用 dirs crate正确获取路径
- [x] 1.2 确认配置目录自动创建逻辑 (save 方法中的 create_dir_all)
- [x] 1.3 验证 TOML 序列化/反序列化正常工作

## 2. 配置验证

- [x] 2.1 实现 Origin URL 格式验证（支持 https://, git@, 本地路径）
- [x] 2.2 实现 Origin 名称唯一性检查
- [x] 2.3 添加验证失败时的错误提示

## 3. 默认配置

- [x] 3.1 确认 with_defaults() 包含 GitLab 内部源
- [x] 3.2 验证首次运行自动创建默认配置

## 4. 测试验证

- [x] 4.1 手动测试 add-origin 命令
- [x] 4.2 手动测试 list-origins 命令
- [x] 4.3 验证配置文件正确写入 `~/.yc-skills/config.toml`