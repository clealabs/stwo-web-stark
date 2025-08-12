#![cfg(not(target_arch = "wasm32"))]

use cairo_air::{verifier::verify_cairo, PreProcessedTraceVariant};
use cairo_lang_runner::Arg;
use cairo_vm::Felt252;
use stwo_cairo_prover::stwo_prover::core::{
    pcs::PcsConfig, vcs::blake2_merkle::Blake2sMerkleChannel,
};
use stwo_web_stark::{prove, trace_gen, verify};
// use stwo_web_stark::{execute_and_prove, verify};

#[test]
fn test_e2e() {
    let executable_json = include_str!("example.executable.json");
    let args = vec![Arg::Value(Felt252::from(100))];
    let pcs_config = PcsConfig::default();
    // let cairo_proof = execute_and_prove(executable_json, args, pcs_config);
    let prover_input = trace_gen(executable_json, args);
    let cairo_proof = prove(prover_input).expect("Failed to prove");
    let preprocessed_trace = PreProcessedTraceVariant::CanonicalWithoutPedersen;
    let result = verify_cairo::<Blake2sMerkleChannel>(cairo_proof, pcs_config, preprocessed_trace);
    assert!(result.is_ok());
}

#[test]
fn verify_is_prime_7() {
    let proof_json = include_str!("is_prime_proof_7.json");
    let cairo_proof = serde_json::from_str(proof_json).expect("Failed to read cairo proof");
    let verdict = verify(cairo_proof, false);
    assert!(verdict, "cairo proof verification failed");
}
