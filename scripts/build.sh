#!/bin/bash
set -e

#frontend
cd dashboard
bun install
bun run build

#backend
cd ../daemon
cargo build --release

cp target/release/daemon ../packages/

echo "Build complete!"