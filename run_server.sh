#!/bin/bash
set -e

echo "=== Installing cargo-leptos ==="
cargo install cargo-leptos --version 0.2.29 --force 2>&1 | tee /tmp/cargo-leptos-install.log

echo ""
echo "=== Starting development server ==="
cd /home/red/Clarry
cargo leptos watch 2>&1 | tee /tmp/leptos-server.log
