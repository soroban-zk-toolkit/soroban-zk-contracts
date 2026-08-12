#!/usr/bin/env bash
# deploy.sh — Deploy the ZK verifier contract to Soroban testnet or mainnet
set -euo pipefail

NETWORK="${1:-testnet}"
IDENTITY="${2:-mykey}"
WASM="target/wasm32-unknown-unknown/release/zk_verifier.wasm"

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

echo "==> Deploying to $NETWORK as $IDENTITY..."

if [[ ! -f "$WASM" ]]; then
  echo "WASM not found. Run ./scripts/build.sh first."
  exit 1
fi

CONTRACT_ID=$(stellar contract deploy \
  --wasm "$WASM" \
  --source "$IDENTITY" \
  --network "$NETWORK")

echo ""
echo "==> Deployment successful!"
echo "    Contract ID : $CONTRACT_ID"
echo "    Network     : $NETWORK"
echo ""
echo "Export for subsequent commands:"
echo "  export CONTRACT_ID=$CONTRACT_ID"
echo "  export STELLAR_NETWORK=$NETWORK"
