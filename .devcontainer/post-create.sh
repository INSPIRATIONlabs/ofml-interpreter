#!/bin/bash
set -e

echo "=== OFML Interpreter Post-Create Setup ==="

# Install cargo tools
echo "Installing cargo tools..."
cargo install cargo-watch cargo-nextest 2>/dev/null || true

# Build the project
echo "Building project..."
cd /workspace
cargo build --features tui

echo "=== Setup Complete ==="
echo "OFML data available at: /reference/ofmldata"
echo "ConceptOffice7 reference at: /reference/ConceptOffice7"
