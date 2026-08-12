#!/usr/bin/env bash
# verify-proof.sh — Example: submit a Groth16 proof to the ZK verifier contract
#
# Usage:
#   export CONTRACT_ID=<your contract id>
#   export STELLAR_NETWORK=testnet
#   ./examples/verify-proof.sh
set -euo pipefail

CONTRACT_ID="${CONTRACT_ID:?Set CONTRACT_ID}"
NETWORK="${STELLAR_NETWORK:-testnet}"
IDENTITY="${STELLAR_IDENTITY:-mykey}"

# Example proof bytes (placeholder — replace with a real proof)
PROOF_HEX="0000000000000000000000000000000000000000000000000000000000000001\
0000000000000000000000000000000000000000000000000000000000000002\
0000000000000000000000000000000000000000000000000000000000000003"

# Example public inputs (placeholder)
INPUTS_HEX="0000000000000000000000000000000000000000000000000000000000000007"

# Example verification key bytes (placeholder — use the one you registered)
VK_HEX="$(cat verification_key.bin 2>/dev/null | xxd -p | tr -d '\n' || echo "00")"

echo "==> Invoking verify_groth16 on contract $CONTRACT_ID ($NETWORK)..."

stellar contract invoke \
  --id "$CONTRACT_ID" \
  --source "$IDENTITY" \
  --network "$NETWORK" \
  -- verify_groth16 \
  --proof "$PROOF_HEX" \
  --inputs "$INPUTS_HEX" \
  --vk "$VK_HEX"

echo "==> Done."
