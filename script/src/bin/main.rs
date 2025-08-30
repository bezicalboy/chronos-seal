use clap::Parser;
use sp1_sdk::{ProverClient, SP1Stdin};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::PathBuf;

/// A command-line tool to generate an SP1 proof for a file's SHA-256 hash.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the file to be hashed and proven.
    #[clap(long)]
    file_path: PathBuf,
}

// This path might be incorrect, will verify later.
const ELF: &[u8] = include_bytes!("../../../target/elf-compilation/riscv32im-succinct-zkvm-elf/release/program");

fn main() {
    let args = Args::parse();

    // 1. Read the file from the given path.
    let file_content = fs::read(&args.file_path)
        .unwrap_or_else(|e| panic!("Failed to read file {:?}: {}", args.file_path, e));

    // 2. Calculate its SHA-256 hash.
    let mut hasher = Sha256::new();
    hasher.update(&file_content);
    let hash_bytes: [u8; 32] = hasher.finalize().into();

    println!("Calculated SHA-256 hash: {}", hex::encode(hash_bytes));

    // 3. Generate an SP1 proof of the zkVM program's execution with the hash as input.
    let client = ProverClient::from_env();
    let mut stdin = SP1Stdin::new();
    stdin.write(&hash_bytes);

    // Setup the proving key
    println!("Setting up proving key...");
    let (proving_key, _verifying_key) = client.setup(ELF); // Destructure the tuple

    println!("Generating SP1 proof...");
    let proof = client.prove(&proving_key, &stdin).run().expect("Failed to generate proof"); // Changed: prove() takes &SP1ProvingKey
    println!("Proof generated successfully.");

    // 4. Serialize and save the final proof to a binary file named proof.bin.
    let proof_bytes = bincode::serialize(&proof).expect("Failed to serialize proof");
    fs::write("proof.bin", proof_bytes).expect("Failed to save proof.bin");

    println!("Proof saved to proof.bin");
}
