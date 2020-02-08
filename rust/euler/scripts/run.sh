#!/bin/bash
set -e
cd /build
cargo install --path .
cargo build
cargo run