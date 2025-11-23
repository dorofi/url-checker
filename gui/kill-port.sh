#!/bin/bash

# Script to kill processes using port 1420

echo "Killing processes on port 1420..."

# Kill processes on port 1420
lsof -ti:1420 | xargs kill -9 2>/dev/null

# Also kill vite and tauri processes
pkill -f "vite" 2>/dev/null
pkill -f "tauri" 2>/dev/null

sleep 2

# Check if port is free
if lsof -ti:1420 > /dev/null 2>&1; then
    echo "Warning: Port 1420 is still in use"
    lsof -ti:1420
else
    echo "Port 1420 is now free"
fi

