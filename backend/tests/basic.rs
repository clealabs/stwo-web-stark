#![cfg(not(target_arch = "wasm32"))]

use cairo_lang_runner::Arg;
use cairo_vm::Felt252;
use stwo_web_stark::{execute_and_prove, verify};

#[test]
fn trace_gen_prove_verify() {
    let executable_json = include_str!("is_prime_executable.json");
    let args = vec![Arg::Value(Felt252::from(7))];
    let cairo_proof = execute_and_prove(executable_json, args);
    let verdict = verify(cairo_proof, false);
    assert!(verdict, "cairo proof verification failed");
}

#[test]
fn verify_is_prime_7() {
    let proof_json = include_str!("is_prime_proof_7.json");
    let cairo_proof = serde_json::from_str(proof_json).expect("Failed to read cairo proof");
    let verdict = verify(cairo_proof, false);
    assert!(verdict, "cairo proof verification failed");
}
