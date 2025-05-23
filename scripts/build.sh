#!/bin/bash
set -e

# Build React frontend
cd dashboard
bun install
bun run build

cd ../daemon
cargo build --release

cp target/release/daemon ../packages/

echo "Build complete!"