#!/usr/bin/env bash

touch loader/src/lib.rs && \
    touch main/src/main.rs && \
    cargo fmt -- --check && \
    cargo clippy -- -D warnings && \
    cargo test && \
    cargo web check -p palladium