#!/bin/bash
# Watch for file changes and rebuild/restart server

cd /home/red/Clarry

# Function to build and restart server
rebuild() {
    echo ""
    echo "=== File changed, rebuilding... ==="
    cargo build --features ssr 2>&1
    
    if [ $? -eq 0 ]; then
        echo "Build successful, restarting server..."
        # Kill old server
        pkill -f l8-loader 2>/dev/null
        sleep 1
        # Start new server
        ./target/debug/l8-loader &
        echo "Server restarted at http://localhost:3000"
    else
        echo "Build failed!"
    fi
}

# Initial build and start
echo "=== Initial build ==="
cargo build --features ssr 2>&1
if [ $? -eq 0 ]; then
    echo "Starting server..."
    ./target/debug/l8-loader &
    echo "Server running at http://localhost:3000"
else
    echo "Initial build failed!"
    exit 1
fi

# Watch for changes using find (polling)
echo ""
echo "=== Watching for file changes (Ctrl+C to stop) ==="
LAST_CHECK=$(find src -type f -name "*.rs" -printf '%T@\n' 2>/dev/null | sort -n | tail -1)

while true; do
    sleep 2
    CURRENT_CHECK=$(find src -type f -name "*.rs" -printf '%T@\n' 2>/dev/null | sort -n | tail -1)
    
    if [ "$CURRENT_CHECK" != "$LAST_CHECK" ]; then
        LAST_CHECK=$CURRENT_CHECK
        rebuild
    fi
done
