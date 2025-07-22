#!/bin/bash

# Build script for Incredible Rust static site generator
set -e

echo "🦀 Building Incredible Rust Site Generator"

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo not found. Please install Rust from https://rustup.rs/"
    exit 1
fi

# Build the generator
echo "🔨 Building the site generator..."
cargo build --release

# Run the generator
echo "🚀 Generating the site..."
cargo run --release

echo "✅ Site generated successfully in _site/"
echo "📁 To preview the site:"
echo "   cd _site && python3 -m http.server 8000"
echo "   Then open http://localhost:8000"