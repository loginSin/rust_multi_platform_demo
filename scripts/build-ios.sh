#!/usr/bin/env bash

# bash scripts/build-ios.sh

# 编译真机 arm64 架构
mkdir -p output/ios/aarch64-apple-ios
cargo build --release --target aarch64-apple-ios
cp -f target/aarch64-apple-ios/release/libmy_lib.a output/ios/aarch64-apple-ios

# 编译模拟器 arm64 架构
mkdir -p output/ios/aarch64-apple-ios-sim
cargo build --release --target aarch64-apple-ios-sim
cp -f target/aarch64-apple-ios-sim/release/libmy_lib.a output/ios/aarch64-apple-ios-sim

# 编译模拟器 x86 架构
mkdir -p output/ios/x86_64-apple-ios
cargo build --release --target x86_64-apple-ios
cp -f target/x86_64-apple-ios/release/libmy_lib.a output/ios/x86_64-apple-ios

# 复制头文件
mkdir -p output/ios/includes
cp -f examples/c_demo/ffi_client.h output/ios/includes