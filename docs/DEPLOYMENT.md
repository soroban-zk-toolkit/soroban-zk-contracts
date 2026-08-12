# Deployment Guide

## Prerequisites

- Rust toolchain with `wasm32-unknown-unknown` target
- Stellar CLI (`stellar`) ≥ 21.0
- A funded Stellar account on testnet or mainnet
- `jq` for JSON manipulation

## 1. Install the Stellar CLI

```bash
cargo install --locked stellar-cli --features opt
```

## 2. Build the Contract

```bash
./scripts/build.sh
# Output: target/wasm32-unknown-unknown/release/zk_verifier.wasm
```

## 3. Configure your identity

```bash
stellar keys generate --global mykey --network testnet
stellar keys address mykey
```

Fund it via the friendbot (testnet only):

```bash
curl "https://friendbot.stellar.org?addr=$(stellar keys address mykey)"
```

## 4. Deploy to Testnet

```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/zk_verifier.wasm \
  --source mykey \
  --network testnet
```

Save the returned contract ID:

```bash
export CONTRACT_ID=<output from above>
```

## 5. Register a Verification Key

```bash
VK_BYTES=$(cat verification_key.bin | xxd -p | tr -d '\n')

stellar contract invoke \
  --id $CONTRACT_ID \
  --source mykey \
  --network testnet \
  -- register_vk \
  --key_id "groth16-v1" \
  --vk "$VK_BYTES"
```

## 6. Deploy to Mainnet

Replace `--network testnet` with `--network mainnet` and ensure your account
has enough XLM for the deployment transaction and storage rent.

```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/zk_verifier.wasm \
  --source mykey \
  --network mainnet
```

## 7. Verify Deployment

```bash
stellar contract info --id $CONTRACT_ID --network testnet
```

## Upgrading

To upgrade the contract WASM without changing the contract ID, use the
`update_current_contract_wasm` host function (requires an admin entry point
in the contract).

## Contract Storage Rent

Soroban charges rent for persistent storage. The contract auto-extends TTL on
every write. Monitor your balance to ensure sufficient XLM for ongoing rent.
