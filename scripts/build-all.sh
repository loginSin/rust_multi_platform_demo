#!/usr/bin/env bash

start_time=$(date +%s)

rm -rf output/

bash scripts/build-mac.sh
bash scripts/build-ios.sh
bash scripts/build-android.sh
bash scripts/build-ohos.sh


python scripts/check-build.py

end_time=$(date +%s)
build_time=$((end_time - start_time))
echo "build-all.sh build time cost: ${build_time} seconds"