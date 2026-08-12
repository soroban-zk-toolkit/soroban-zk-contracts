# Integration Guide for dApps

This guide shows how to integrate the ZK Verifier contract into your Stellar
dApp, from generating a proof off-chain to submitting it on-chain.

## Overview

```
Off-chain                            On-chain
─────────────────────────────────    ──────────────────────────
User input → circom/snarkjs circuit  → invoke verify_groth16
           → proof.json              → contract checks proof
           → public_inputs.json      → emits proof_verified event
```

## Step 1: Generate a Proof

Using snarkjs (Node.js):

```js
import { groth16 } from "snarkjs";

const { proof, publicSignals } = await groth16.fullProve(
  { a: 3, b: 11 }, // private inputs
  "circuit.wasm",
  "circuit_final.zkey"
);

// Serialise for on-chain submission
const proofBytes = serializeGroth16Proof(proof);       // custom serialiser
const publicInputBytes = serializePublicInputs(publicSignals);
```

## Step 2: Submit On-Chain

Using the Stellar JavaScript SDK:

```js
import { Contract, SorobanRpc, TransactionBuilder, Networks } from "@stellar/stellar-sdk";

const server = new SorobanRpc.Server("https://soroban-testnet.stellar.org");
const contract = new Contract(CONTRACT_ID);

const tx = new TransactionBuilder(sourceAccount, { fee: "100000" })
  .setNetworkPassphrase(Networks.TESTNET)
  .setTimeout(30)
  .addOperation(
    contract.call(
      "verify_groth16",
      xdr.ScVal.scvBytes(proofBytes),
      xdr.ScVal.scvBytes(publicInputBytes),
      xdr.ScVal.scvBytes(vkBytes)
    )
  )
  .build();

const sim = await server.simulateTransaction(tx);
const preparedTx = SorobanRpc.assembleTransaction(tx, sim).build();
preparedTx.sign(sourceKeypair);

const result = await server.sendTransaction(preparedTx);
```

## Step 3: Listen for Events

```js
const events = await server.getEvents({
  startLedger: deployLedger,
  filters: [{ type: "contract", contractIds: [CONTRACT_ID] }]
});

for (const event of events.events) {
  const [topic, scheme] = event.topic;
  if (topic === "verified") {
    console.log(`Proof accepted for scheme: ${scheme}`);
  }
}
```

## Error Handling

| Error Code | Meaning |
|---|---|
| 1 | `InvalidProof` — malformed proof bytes |
| 2 | `VerificationFailed` — pairing check failed |
| 3 | `InvalidPublicInputs` — malformed public inputs |
| 4 | `UnknownVerificationKey` — VK not registered |
| 5 | `Unauthorized` — caller is not admin |
| 6 | `NullifierSpent` — proof already used |
| 99 | `NotImplemented` — scheme not yet available |

## SDK Helper (Coming Soon)

A TypeScript SDK helper library is planned for `v0.2.0` that will handle
serialisation, simulation, and event parsing automatically.
