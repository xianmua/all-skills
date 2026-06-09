## 1. 修改 Install 命令

- [ ] 1.1 在 `Install` 结构体中添加 `--comate` 参数（与 `--trae`、`--clion` 互斥）
- [ ] 1.2 在 `get_install_dir()` 方法中添加 `.comate/skills` 分支判断
- [ ] 1.3 更新 `--help` 帮助文本确保清晰显示所有选项

## 2. 测试验证

- [ ] 2.1 测试 `install fmt --comate` 安装到正确目录
- [ ] 2.2 测试 `install fmt --comate --trae` 显示冲突错误
- [ ] 2.3 测试默认安装仍为 `--trae`

## 3. 更新文档

- [ ] 3.1 更新 README.md 添加 `--comate` 说明
- [ ] 3.2 更新中文帮助文本