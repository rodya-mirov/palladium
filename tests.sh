#!/usr/bin/env bash

touch src/main.rs && \
    cargo fmt -- --check && \
    cargo clippy -- -D warnings && \
    cargo test