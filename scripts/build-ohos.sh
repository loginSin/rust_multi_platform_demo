#!/usr/bin/env bash

# bash scripts/build-ohos.sh

# 配置环境变量，保证能找到 aarch64-unknown-linux-ohos-clang
if [ -z "${RC_OPEN_OHOS_PATH}" ]; then
    echo "error: RC_OPEN_OHOS_PATH not found"
    echo "eg."
    echo "export RC_OPEN_OHOS_PATH=\"/Applications/DevEco-Studio.app/Contents/sdk/HarmonyOS-NEXT-DB1/openharmony\""
    echo "export RC_OPEN_OHOS_PATH=\"/root/.harmony/commandline-Linux-5.0.3.401/command-line-tools/sdk/HarmonyOS-NEXT-DB1/openharmony\""
    exit 1
fi

# 编译真机 arm64 架构，需要指明 arm64 clang 路径和鸿蒙系统头文件路径
mkdir -p output/ohos/aarch64-unknown-linux-ohos
export CC=${RC_OPEN_OHOS_PATH}/native/llvm/bin/aarch64-unknown-linux-ohos-clang
export C_INCLUDE_PATH=${RC_OPEN_OHOS_PATH}/native/sysroot/usr/include:${RC_OPEN_OHOS_PATH}/native/sysroot/usr/include/aarch64-linux-ohos
cargo build -Zbuild-std -p my_lib --release --target aarch64-unknown-linux-ohos
cp -f target/aarch64-unknown-linux-ohos/release/libmy_lib.a output/ohos/aarch64-unknown-linux-ohos

# 编译模拟器 x86 架构，需要指明 x86 clang 路径和鸿蒙系统头文件路径
mkdir -p output/ohos/x86_64-unknown-linux-ohos
export CC=${RC_OPEN_OHOS_PATH}/native/llvm/bin/x86_64-unknown-linux-ohos-clang
export C_INCLUDE_PATH=${RC_OPEN_OHOS_PATH}/native/sysroot/usr/include:${RC_OPEN_OHOS_PATH}/native/sysroot/usr/include/x86_64-linux-ohos
cargo build -Zbuild-std -p my_lib --release --target x86_64-unknown-linux-ohos
cp -f target/x86_64-unknown-linux-ohos/release/libmy_lib.a output/ohos/x86_64-unknown-linux-ohos

# 复制头文件
mkdir -p output/ohos/includes
cp -f examples/c_demo/ffi_client.h output/ohos/includes