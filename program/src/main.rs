#![no_main]
sp1_zkvm::entrypoint!(main);

pub fn main() {
    // Read the 32-byte hash from STDIN.
    let hash_bytes: [u8; 32] = sp1_zkvm::io::read();

    // Commit the hash to the public values.
    sp1_zkvm::io::commit_slice(&hash_bytes);
}
