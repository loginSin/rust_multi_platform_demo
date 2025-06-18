#!/usr/bin/env bash

# bash scripts/build-linux.sh

#| 架构      | Rust Target Triple              | 输出库格式 | 说明                 |
#| ------- | ------------------------------- | ----- | ------------------ |
#| x86\_64 | `x86_64-unknown-linux-gnu`      | `.a`  | 常见 64 位 Linux      |
#| aarch64 | `aarch64-unknown-linux-gnu`     | `.a`  | 64 位 ARM (如树莓派 4+) |
#| armv7   | `armv7-unknown-linux-gnueabihf` | `.a`  | 32 位 ARM (如树莓派 3)  |
#| i686    | `i686-unknown-linux-gnu`        | `.a`  | 32 位 x86           |
#| riscv64 | `riscv64gc-unknown-linux-gnu`   | `.a`  | RISC-V 平台（新兴架构）    |

# 安装 x86_64-unknown-linux-gnu
# brew tap messense/macos-cross-toolchains
# brew install x86_64-unknown-linux-gnu
# x86_64-unknown-linux-gnu-gcc --version
# export CC_x86_64_unknown_linux_gnu=x86_64-unknown-linux-gnu-gcc

# 构建 x86_64 架构，参考 android 需要指明 CC_x86_64-unknown-linux-gnu
mkdir -p output/linux/x86_64-unknown-linux-gnu
cargo build -p my_lib --release --target x86_64-unknown-linux-gnu
cp -f target/x86_64-unknown-linux-gnu/release/libmy_lib.a output/linux/x86_64-unknown-linux-gnu


# 复制头文件
mkdir -p output/linux/includes
cp -f examples/c_demo/ffi_client.h output/linux/includes