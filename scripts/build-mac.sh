#!/usr/bin/env bash

# bash scripts/build-mac.sh

# 编译 arm64 架构： M 芯片
mkdir -p output/aarch64-apple-darwin
cargo build --release --target aarch64-apple-darwin
cp -f target/aarch64-apple-darwin/release/libmy_lib.a output/aarch64-apple-darwin

# 编译 x86 架构：intel 芯片
mkdir -p output/x86_64-apple-darwin
cargo build --release --target x86_64-apple-darwin
cp -f target/x86_64-apple-darwin/release/libmy_lib.a output/x86_64-apple-darwin

mkdir -p output/includes
cp -f examples/c_demo/ffi_client.h output/includes