#!/bin/bash
# Simple development server without cargo-leptos
# Uses cargo build --features ssr and runs the binary directly

cd /home/red/Clarry

echo "=== Building SSR binary ==="
cargo build --features ssr 2>&1

if [ $? -ne 0 ]; then
    echo "Build failed!"
    exit 1
fi

echo ""
echo "=== Starting server ==="
echo "Server will be available at http://localhost:3000"
echo "Press Ctrl+C to stop"
echo ""

# Run the server
./target/debug/l8-loader
