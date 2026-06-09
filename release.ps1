# all-skills 发布脚本

# Windows
echo "构建 Windows 版本..."
cargo build --release --target x86_64-pc-windows-msvc
copy target\release\all-skills.exe all-skills.exe

# Linux
echo "构建 Linux 版本..."
cargo build --release --target x86_64-unknown-linux-gnu

# macOS
echo "构建 macOS 版本..."
cargo build --release --target x86_64-apple-darwin

echo "构建完成！"
