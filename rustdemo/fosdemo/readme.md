#

必须使用特定版本才能编译通过，所以自建项目时，Cargo.lock 文件锁定版本。

- rust = nightly-2021-03-01
- bootloader = "<=0.9.18"
- x86_64 = "<0.14.0"


```bash
# 安装指定版本
rustup toolchain install nightly-2021-03-01

#
rustup default nightly-2021-03-01

rustup component add rust-src
rustup component add llvm-tools-preview
```
