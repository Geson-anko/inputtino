#!/bin/bash
set -e

# Set up XDG runtime directory
mkdir -p /tmp/runtime-root
chmod 700 /tmp/runtime-root
export XDG_RUNTIME_DIR=/tmp/runtime-root

# Start Xvfb
Xvfb :99 -screen 0 1024x768x24 &

# Execute the provided command
exec "$@"
