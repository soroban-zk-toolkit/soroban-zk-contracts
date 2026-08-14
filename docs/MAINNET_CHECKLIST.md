# Mainnet Deployment Checklist — Soroban ZK Verifier Contract

## Purpose

Use this checklist before and after every mainnet deployment of the `soroban-zk-contracts` verifier. All items must be signed off by at least two team members.

---

## Pre-Deployment

### Code & Audit

- [ ] All open issues labeled `mainnet-blocker` are resolved and closed
- [ ] The WASM binary to be deployed matches the git tag in the release (verify SHA-256 hash)
- [ ] An independent security audit has been completed for this version
- [ ] Audit findings are documented and addressed (see `audits/` directory)
- [ ] No `todo!()`, `unimplemented!()`, or `panic!()` calls remain in production paths
- [ ] Spending limits are configured to sane defaults (see `SPENDING_LIMITS.md`)
- [ ] Admin address is confirmed as a multisig wallet — **never a single hot key**

### Testnet Validation

- [ ] Full deployment performed on Stellar Testnet with the exact WASM binary
- [ ] All entry points exercised on testnet (register VK, verify proof, batch verify, pause/unpause, admin transfer)
- [ ] Batch verification tested at expected maximum batch size without hitting resource limits
- [ ] Epoch spending limit tested: confirm rejection after limit is reached
- [ ] Two-step admin transfer tested end-to-end on testnet
- [ ] Contract pause tested: all write entry points return an error while paused; reads succeed

### Infrastructure

- [ ] Stellar account funding confirmed — deployer account has sufficient XLM for deployment fees
- [ ] RPC endpoint for mainnet selected and load-tested
- [ ] Off-chain event indexer configured and pointed at mainnet contract address (TBD post-deploy)
- [ ] Monitoring alerts set up for anomalous verification rates (>10× baseline per epoch)
- [ ] Incident response contacts list is current and reachable

### Deployment Scripts

- [ ] Deployment script uses the correct network passphrase for mainnet (`Public Global Stellar Network ; September 2015`)
- [ ] Admin address in deploy script matches the approved multisig address (double-check on-chain)
- [ ] Deployment script is committed and reviewed — no ad-hoc CLI commands
- [ ] Rollback procedure documented (see **Post-Deployment: Rollback** below)

---

## Deployment Steps

1. Build the release WASM:
   ```bash
   stellar contract build --release
   ```
2. Verify the WASM hash matches the expected value from the release tag:
   ```bash
   sha256sum target/wasm32-unknown-unknown/release/soroban_zk_contracts.wasm
   ```
3. Upload the WASM to mainnet:
   ```bash
   stellar contract upload --wasm target/wasm32-unknown-unknown/release/soroban_zk_contracts.wasm \
     --source DEPLOYER_KEY --network mainnet
   ```
4. Deploy the contract:
   ```bash
   stellar contract deploy --wasm-hash <WASM_HASH> \
     --source DEPLOYER_KEY --network mainnet
   ```
5. Record the **Contract ID** from the output.
6. Initialise the contract (set admin, spending limit):
   ```bash
   stellar contract invoke --id <CONTRACT_ID> --source ADMIN_KEY --network mainnet \
     -- initialize --admin <MULTISIG_ADDRESS> --spending-limit 100
   ```
7. Verify initial state reads correctly:
   ```bash
   stellar contract invoke --id <CONTRACT_ID> --network mainnet -- get_admin
   ```

---

## Post-Deployment

### Smoke Tests

- [ ] `get_admin` returns the expected multisig address
- [ ] `is_paused` returns `false`
- [ ] Register a test verification key and confirm `vk_registered` event appears on-chain
- [ ] Submit a valid proof against the test VK and confirm `proof_verified` event
- [ ] Submit an invalid proof and confirm `proof_rejected` event with `reason: invalid_proof`
- [ ] Revoke the test VK and confirm `vk_revoked` event

### Documentation & Communication

- [ ] Contract ID recorded in `docs/DEPLOYMENTS.md` (create if not present)
- [ ] Release notes published describing changes from previous version
- [ ] Downstream integrators notified of new contract address (if changed)
- [ ] Event indexer confirmed receiving events from the new contract

### Monitoring (First 24 Hours)

- [ ] Watch verification volume for unexpected spikes
- [ ] Confirm spending limits are triggering correctly under load
- [ ] No `proof_rejected` events with unexpected `reason` codes

---

## Rollback Procedure

If a critical bug is discovered post-deployment:

1. **Immediately call `pause()`** on the deployed contract via the admin multisig.
2. Assess impact — review all on-chain events since deployment.
3. If a fix is available: deploy corrected contract, update admin to new address via two-step transfer on old contract if needed.
4. If no fix yet: contract remains paused; communicate status to integrators.
5. Document the incident in `docs/INCIDENTS.md`.

---

## Sign-Off

| Role | Name | Date | Signature |
|------|------|------|-----------|
| Lead Engineer | | | |
| Security Reviewer | | | |
| DevOps | | | |
