#!/bin/bash
set -euxo pipefail

export RGAPI_KEY="$(cat apikey.txt)"

cd riven

# Ensure builds with tracing, metrics.
wasm-pack build -- --features nightly,tracing,metrics

# Run tests.
wasm-pack test --node -- --features nightly,deny-unknown
