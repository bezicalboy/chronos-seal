# Chronos Seal dApp

This repository contains a Proof of Concept (PoC) for the Chronos Seal dApp, demonstrating an off-chain SP1 Prover and an on-chain Sui Smart Contract for verifying file hashes. This PoC is intended for contribution to SoundnessLabs for testnet validation and further development.

## Project Overview

The Chronos Seal dApp allows users to "seal" a file by generating a zero-knowledge proof of its SHA-256 hash off-chain and then verifying this proof on the Sui blockchain. Upon successful verification, a `SealRecord` is created on-chain, linking the file's hash to the sealer's address and a timestamp.

## Components

This dApp consists of two main components:

1.  **Off-Chain SP1 Prover (`chronos-seal-prover`):** A Rust command-line tool that calculates the SHA-256 hash of a given file and generates an SP1 proof of the hash calculation.
2.  **On-Chain Sui Smart Contract (`chronos-seal-contract`):** A Sui Move smart contract that verifies the SP1 proof using Sui's native Groth16 verifier and records the sealed file's hash on-chain.

## Setup Instructions

### Prerequisites

*   [Rust](https://www.rust-lang.org/tools/install)
*   [Sui Client CLI](https://docs.sui.io/guides/developer/getting-started/sui-install)
*   [SP1 zkVM](https://github.com/succinctlabs/sp1) (ensure `cargo prove` is installed)

### Clone the Repository

```bash
git clone https://github.com/your-repo/chronos-seal-prover.git # Replace with actual repo URL
cd chronos-seal-prover
```

### Build the Prover

The prover is a Rust project within the workspace. Its build process is automated.

```bash
cargo build --release -p script
```

### Build and Deploy the Sui Smart Contract

Navigate to the contract directory and build the Move package:

```bash
cd chronos_seal_contract
sui move build
```

To deploy the contract to the Sui Testnet:

```bash
sui client publish --gas-budget 100000000 chronos_seal_contract
```

Note the `Package ID` from the deployment output. You will need this later.

## How to Run the Prover

1.  Create a file you wish to seal (e.g., `my_document.txt`).
2.  Run the prover from the project root, providing the path to your file:

```bash
cargo run --release --bin script -- --file-path test_file.txt
```

This will generate a `proof.bin` file in the project root directory.

## Known Issues and Limitations

### Proof Size

The generated `proof.bin` file can be very large. When attempting to submit such large proofs to a server (e.g., a relayer service), you might encounter a "413 Payload Too Large" error. Reducing the proof size typically requires advanced Zero-Knowledge Proof (ZKP) techniques such as recursive proofs or proof aggregation. These techniques are beyond the scope of this Proof of Concept and would require significant architectural changes and cryptographic engineering.

### Client-Side Submitter

The client-side submitter (`chronos-seal-client`) component, intended to send the `proof.bin` to the deployed Sui contract, was not completed. This was due to unresolvable dependency conflicts between `sp1-sui` and `sp1-sdk` versions, and their transitive dependencies. The rapid evolution of the `sui-sdk` API also contributed to these challenges.

## Contribution to SoundnessLabs

This PoC is intended as a contribution to SoundnessLabs for testing and further development on the Sui Testnet. We believe it provides a solid foundation for integrating SP1 proofs with Sui. We welcome feedback and collaboration to address the known limitations and expand its capabilities.

For inquiries or collaboration, please refer to the official SoundnessLabs contact channels or GitHub repository.
