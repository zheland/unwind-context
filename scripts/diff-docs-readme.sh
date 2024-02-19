#!/usr/bin/env bash

diff --color=always -u \
    <(
        cat README.md \
            | rg --passthru '^#(#*) ' -r '$1 '
    ) \
    <(
        cat src/lib.rs \
            | rg --passthru '^//! ?' -r '' \
            | rg --passthru '^#!\[doc = "(.*)"\]$' -r '$1'
    )
