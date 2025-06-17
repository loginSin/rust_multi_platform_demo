#!/usr/bin/env bash

# bash scripts/build-android.sh

#| Android ABI   | Rust Target Triple        | 架构类型       |
#| ------------- | ------------------------- | ---------- |
#| `arm64-v8a`   | `aarch64-linux-android`   | 64-bit ARM |
#| `armeabi-v7a` | `armv7-linux-androideabi` | 32-bit ARM |
#| `x86_64`      | `x86_64-linux-android`    | 64-bit x86 |
#| `x86`         | `i686-linux-android`      | 32-bit x86 |

# cargo build 不通过 -p 指明 sdk ，会报错  ld: unknown option: --as-needed

# 编译 arm64 架构。需要指明 arm64 clang 路径，需要根据实际情况调整
export CC_aarch64_linux_android="$HOME/Library/Android/sdk/ndk/25.1.8937393/toolchains/llvm/prebuilt/darwin-x86_64/bin/aarch64-linux-android21-clang"
mkdir -p output/android/aarch64-linux-android
cargo build -p my_lib --release --target aarch64-linux-android
cp -f target/aarch64-linux-android/release/libmy_lib.a output/android/aarch64-linux-android

# 编译 armv7 架构。需要指明 armv7 clang 路径，需要根据实际情况调整
export CC_armv7_linux_androideabi="$HOME/Library/Android/sdk/ndk/25.1.8937393/toolchains/llvm/prebuilt/darwin-x86_64/bin/armv7a-linux-androideabi21-clang"
mkdir -p output/android/armv7-linux-androideabi
cargo build -p my_lib --release --target armv7-linux-androideabi
cp -f target/armv7-linux-androideabi/release/libmy_lib.a output/android/armv7-linux-androideabi

# 编译 x86_64 架构。需要指明 x86_64 clang 路径，需要根据实际情况调整
export CC_x86_64_linux_android="$HOME/Library/Android/sdk/ndk/25.1.8937393/toolchains/llvm/prebuilt/darwin-x86_64/bin/x86_64-linux-android21-clang"
mkdir -p output/android/x86_64-linux-android
cargo build -p my_lib --release --target x86_64-linux-android
cp -f target/x86_64-linux-android/release/libmy_lib.a output/android/x86_64-linux-android

# 编译 x86 架构。需要指明 x86 clang 路径，需要根据实际情况调整
export CC_i686_linux_android="$HOME/Library/Android/sdk/ndk/25.1.8937393/toolchains/llvm/prebuilt/darwin-x86_64/bin/i686-linux-android21-clang"
mkdir -p output/android/i686-linux-android
cargo build -p my_lib --release --target i686-linux-android
cp -f target/i686-linux-android/release/libmy_lib.a output/android/i686-linux-android

# 复制头文件
mkdir -p output/android/includes
cp -f examples/c_demo/ffi_client.h output/android/includes