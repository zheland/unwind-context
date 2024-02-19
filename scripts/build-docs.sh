#!/usr/bin/env bash

cargo +nightly rustdoc --all-features -- --cfg docsrs
