#!/bin/zsh
wasm-pack build --target web --release
python fix_package_file.py
cd pkg
npm link