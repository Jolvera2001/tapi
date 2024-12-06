#!/bin/bash

# Run frontend coverage
cargo tarpaulin --manifest-path ./Cargo.toml --out Xml --output-dir ./coverage

# Run backend coverage
cd src-tauri && cargo tarpaulin --out Xml --output-dir ../coverage

# Combine reports using grcov
grcov ./coverage -t html --llvm --branch --ignore-not-existing -o ./coverage/combined

# Clean up intermediate files
rm ./coverage/*.xml