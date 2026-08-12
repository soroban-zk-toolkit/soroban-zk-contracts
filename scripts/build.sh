#!/usr/bin/env bash
# build.sh — Build the ZK verifier contract to WASM
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="$REPO_ROOT/target/wasm32-unknown-unknown/release"
WASM_NAME="zk_verifier.wasm"

echo "==> Building soroban-zk-contracts (release profile)..."
cd "$REPO_ROOT"

cargo build \
  --target wasm32-unknown-unknown \
  --release \
  --package zk-verifier

echo ""
echo "==> Build complete."
echo "    Output: $OUT_DIR/$WASM_NAME"

if command -v wasm-opt &>/dev/null; then
  echo "==> Running wasm-opt optimisation pass..."
  wasm-opt -Oz \
    "$OUT_DIR/$WASM_NAME" \
    -o "$OUT_DIR/${WASM_NAME%.wasm}.optimized.wasm"
  echo "    Optimised: $OUT_DIR/${WASM_NAME%.wasm}.optimized.wasm"
  ls -lh "$OUT_DIR/$WASM_NAME" "$OUT_DIR/${WASM_NAME%.wasm}.optimized.wasm"
else
  echo "    (wasm-opt not found — skipping optimisation pass)"
  ls -lh "$OUT_DIR/$WASM_NAME"
fi
