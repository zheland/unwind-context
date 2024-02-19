#!/usr/bin/env bash

CARGO_TARGET_DIR=target/llvm-cov cargo +beta llvm-cov --all-features --html
