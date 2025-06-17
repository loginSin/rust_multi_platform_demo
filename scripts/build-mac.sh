#!/usr/bin/env bash

# bash scripts/build-mac.sh

# 编译 arm64 架构： M 芯片
mkdir -p output/mac/aarch64-apple-darwin
cargo build --release --target aarch64-apple-darwin
cp -f target/aarch64-apple-darwin/release/libmy_lib.a output/mac/aarch64-apple-darwin

# 编译 x86 架构：intel 芯片
mkdir -p output/mac/x86_64-apple-darwin
cargo build --release --target x86_64-apple-darwin
cp -f target/x86_64-apple-darwin/release/libmy_lib.a output/mac/x86_64-apple-darwin

# 复制头文件
mkdir -p output/mac/includes
cp -f examples/c_demo/ffi_client.h output/mac/includes
