#!/usr/bin/env bash

set -Eeuo pipefail

# Env variables:
# - NO_EXIT_ON_FAIL:
#   when set to `true`, all checks will be performed regardless of failure.

NO_EXIT_ON_FAIL="${NO_EXIT_ON_FAIL:-false}"

comma_separated() {
    local IFS=","
    echo "$*"
}

all_features=(
    "default"
    "std"
    "detect-color-support"
    "custom-default-colors"
)

toolchains=(
    stable
    beta
    nightly
    "1.70.0"
)

( set -x; cargo +nightly fmt --all -- --check || $NO_EXIT_ON_FAIL )

(
    set -x; cargo +nightly rustdoc --all-features -- \
        -Z unstable-options --check  -D warnings \
        || $NO_EXIT_ON_FAIL
)

for toolchain in "${toolchains[@]}"; do
    export CARGO_TARGET_DIR="target/check-$toolchain"

    num_features=${#all_features[@]}
    num_combinations=$(echo "2^$num_features" | bc)

    # iterate over all 2^num_features features combinations if required
    # `j1` is used as a bitmask of the enabled features
    for ((j1 = 0; j1 < num_combinations; j1++)); do
        features_set=()
        for ((j2 = 0; j2 < num_features; j2++)); do
            mask=$(echo "2^$j2" | bc) # the mask of `j2`-th feature

            if (( j1 & mask )); then
                features_set+=(${all_features[$j2]})
            fi
        done

        features=$(comma_separated "${features_set[@]}")

        (
            set -x
            cargo "+$toolchain" clippy \
                --no-default-features --features "${features}" \
                -- -D warnings \
                || $NO_EXIT_ON_FAIL
            cargo "+$toolchain" clippy --all-targets \
                --no-default-features --features "${features}" \
                -- -D warnings \
                || $NO_EXIT_ON_FAIL
            cargo "+$toolchain" build \
                --no-default-features --features "${features}" \
                || $NO_EXIT_ON_FAIL
            cargo "+$toolchain" test --all-targets \
                --no-default-features --features "${features}" \
                || $NO_EXIT_ON_FAIL
            cargo "+$toolchain" build --release \
                --no-default-features --features "${features}" \
                || $NO_EXIT_ON_FAIL
            cargo "+$toolchain" test --release --all-targets \
                --no-default-features --features "${features}" \
                || $NO_EXIT_ON_FAIL
        )
    done

    (
        set -x
        cd ./tests/no-alloc
        cargo +$toolchain clippy -- -D warnings
        cargo +$toolchain build
    )
    if [[ $toolchain != "1.70.0" ]]; then
        (
            set -x
            cd ./tests/no-std
            cargo +$toolchain clippy -- -D warnings
            cargo +$toolchain build
        )
    fi
done

( set -x; cargo deny --workspace --all-features check )
