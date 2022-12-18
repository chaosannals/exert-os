# kosdemo

```bash
# 运行
cargo run

# 构建
cargo build

# 运行测试
cargo test
# 运行单元测试
cargo test --lib
```

## 依赖

- rustc 1.67.0-nightly

```bash
# cargo build 扩展工具
cargo install cargo-xbuild

# 构造
cargo xbuild --target x86_64.json

# 通过 bootimage 构造
cargo bootimage
```

```bash
# Linux
cargo rustc -- -C link-arg=-nostartfiles
# Windows
cargo rustc -- -C link-args="/ENTRY:_start /SUBSYSTEM:console"
# macOS
cargo rustc -- -C link-args="-e __start -static -nostartfiles"
```

```bash
# qemu 直接运行
qemu-system-x86_64 -drive format=raw,file=target/x86_64/debug/bootimage-kosdemo.bin

# linux dd 命令写入 U 盘 sdX 改成 U 盘名
dd if=target/x86_64/debug/bootimage-kosdemo.bin of=/dev/sdX && sync
```