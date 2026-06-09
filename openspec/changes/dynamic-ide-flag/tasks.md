## 1. 实现动态 --xxx 参数

- [x] 1.1 在 main.rs 中添加 extract_ide_args() 函数提取所有 --xxx 参数
- [x] 1.2 在 main.rs 中添加 clean_args() 函数清理自定义参数
- [x] 1.3 修改 parse_from() 使用清理后的参数
- [x] 1.4 将提取的 IDE 参数传递给 install 命令

## 2. 更新 Install 命令

- [x] 2.1 简化 Install 结构体，使用 --ide 参数接收动态值
- [x] 2.2 修改 get_install_dir() 支持动态 IDE 目录

## 3. 测试验证

- [x] 3.1 测试 `install fmt --trae` → `.trae/skills/fmt`
- [x] 3.2 测试 `install fmt --comate` → `.comate/skills/fmt`
- [x] 3.3 测试默认安装到 `.agent/skills`

## 4. 构建发布

- [x] 4.1 构建 release 版本
