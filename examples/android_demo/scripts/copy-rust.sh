#!/usr/bin/env bash

#| Android ABI   | Rust Target Triple        | 架构类型       |
#| ------------- | ------------------------- | ---------- |
#| `arm64-v8a`   | `aarch64-linux-android`   | 64-bit ARM |
#| `armeabi-v7a` | `armv7-linux-androideabi` | 32-bit ARM |
#| `x86_64`      | `x86_64-linux-android`    | 64-bit x86 |
#| `x86`         | `i686-linux-android`      | 32-bit x86 |

mkdir -p mynativelib/src/main/cpp/libs/arm64-v8a
cp -f ../../output/android/aarch64-linux-android/libmy_lib.a mynativelib/src/main/cpp/libs/arm64-v8a

mkdir -p mynativelib/src/main/cpp/libs/armeabi-v7a
cp -f ../../output/android/armv7-linux-androideabi/libmy_lib.a mynativelib/src/main/cpp/libs/armeabi-v7a

mkdir -p mynativelib/src/main/cpp/libs/x86_64
cp -f ../../output/android/x86_64-linux-android/libmy_lib.a mynativelib/src/main/cpp/libs/x86_64

mkdir -p mynativelib/src/main/cpp/libs/x86
cp -f ../../output/android/i686-linux-android/libmy_lib.a mynativelib/src/main/cpp/libs/x86

cp -af ../../output/android/includes mynativelib/src/main/cpp/libs