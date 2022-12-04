# 

## 

```bash
# 打印 rust 编译器目录
rustc --print sysroot

# 切换成 未稳定工具链
rustup default nightly

# 切换回标准版本
rustup default stable

# 使用 nightly 运行
cargo +nightly run
```

### 安装工具库

```bash
# 安装二进制处理工具
cargo install cargo-binutils

# 安装启动镜像工具包
cargo install bootimage

# 获取 rust 编译器源码
rustup component add rust-src

# 获取 rust 编译器源码 指定 nightly 版本
rustup component add rust-src --toolchain nightly-x86_64-pc-windows-msvc

# 获取 llvm 工具预览版（之后正式发布可能时 llvm-tools）
rustup component add llvm-tools-preview
```
