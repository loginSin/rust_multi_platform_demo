#!/usr/bin/env bash

rm -rf output/

bash scripts/build-mac.sh
bash scripts/build-ios.sh


python scripts/check-build.py