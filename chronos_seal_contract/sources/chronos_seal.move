module chronos_seal_contract::chronos_seal {
    use sui::tx_context::{Self, TxContext};
    use sui::object::{Self, UID};
    use sui::transfer;
    use sui::clock;
    use sui::groth16;

    /// A record of a sealed file.
    public struct SealRecord has key, store {
        id: UID,
        file_hash: vector<u8>,
        sealed_by: address,
        sealed_at: u64,
    }

    /// Creates a new SealRecord if the provided proof is valid.
    public entry fun create_seal(
        groth16_vk: vector<u8>,
        public_inputs: vector<u8>,
        proof_points: vector<u8>,
        clock: &clock::Clock,
        ctx: &mut TxContext,
    ) {
        // Prepare the verifying key and public inputs
        let pvk = groth16::prepare_verifying_key(&groth16::bn254(), &groth16_vk);
        let public_inputs_parsed = groth16::public_proof_inputs_from_bytes(public_inputs);
        let proof_points_parsed = groth16::proof_points_from_bytes(proof_points);

        // Verify the Groth16 proof.
        assert!(groth16::verify_groth16_proof(
            &groth16::bn254(),
            &pvk,
            &public_inputs_parsed,
            &proof_points_parsed
        ), 0); // Abort if the proof is invalid.

        // Extract the file hash from public_inputs.
        // Assuming public_inputs directly contains the 32-byte hash.
        let file_hash = public_inputs;

        // Get the current timestamp.
        let current_timestamp = clock::timestamp_ms(clock);

        // Create a new SealRecord object.
        let seal_record = SealRecord {
            id: object::new(ctx),
            file_hash,
            sealed_by: tx_context::sender(ctx),
            sealed_at: current_timestamp,
        };

        // Transfer the SealRecord object to the sender.
        transfer::public_transfer(seal_record, tx_context::sender(ctx));
    }
}
