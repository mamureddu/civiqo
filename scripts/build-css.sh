#!/bin/bash
# Build Tailwind CSS
# Run from project root: ./scripts/build-css.sh

set -e

# Check if tailwindcss binary exists
if [ ! -f "./tailwindcss" ]; then
    echo "Downloading Tailwind CSS standalone CLI..."
    curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-macos-arm64
    chmod +x tailwindcss-macos-arm64
    mv tailwindcss-macos-arm64 tailwindcss
fi

# Build CSS
echo "Building Tailwind CSS..."
./tailwindcss -i ./src/server/static/css/input.css -o ./src/server/static/css/tailwind.css --minify

echo "Done! CSS built to src/server/static/css/tailwind.css"
