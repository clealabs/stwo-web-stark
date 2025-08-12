//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

use cairo_air::{verifier::verify_cairo, PreProcessedTraceVariant};
use cairo_lang_runner::Arg;
use cairo_vm::Felt252;
use stwo_cairo_prover::stwo_prover::core::{
    fri::FriConfig, pcs::PcsConfig, vcs::blake2_merkle::Blake2sMerkleChannel,
};
use stwo_web_stark::{prove, trace_gen, verify};
// use stwo_web_stark::{execute_and_prove, verify};
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

wasm_bindgen_test_configure!(run_in_browser);

pub fn tiny_pcs_config() -> PcsConfig {
    PcsConfig {
        pow_bits: 0,
        fri_config: FriConfig {
            log_last_layer_degree_bound: 0,
            log_blowup_factor: 1,
            n_queries: 1,
        },
    }
}

#[wasm_bindgen_test]
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

#[wasm_bindgen_test]
fn verify_is_prime_7() {
    let proof_json = include_str!("is_prime_proof_7.json");
    let cairo_proof = serde_json::from_str(proof_json).expect("Failed to read cairo proof");
    let verdict = verify(cairo_proof, false);
    assert!(verdict, "cairo proof verification failed");
}

// #[wasm_bindgen_test]
// fn trace_gen_prove_verify_cairo0() {
//     let reader = std::io::Cursor::new(include_bytes!("fibonacci_cairo0_pie.zip"));
//     let zip_archive = zip::ZipArchive::new(reader).unwrap();
//     let pie = from_zip_archive(zip_archive).unwrap();

//     let trace_gen_output = trace_gen(pie).unwrap();
//     let cairo_proof = prove(trace_gen_output.prover_input).unwrap();
//     let verdict = verify(cairo_proof);

//     assert!(verdict);
// }

// #[wasm_bindgen_test]
// fn trace_gen_prove_verify_cairo1() {
//     let reader = std::io::Cursor::new(include_bytes!("fibonacci_cairo1_pie.zip"));
//     let zip_archive = zip::ZipArchive::new(reader).unwrap();
//     let pie = from_zip_archive(zip_archive).unwrap();

//     let trace_gen_output = trace_gen(pie).unwrap();
//     let cairo_proof = prove(trace_gen_output.prover_input).unwrap();
//     let verdict = verify(cairo_proof);

//     assert!(verdict);
// }

// #[wasm_bindgen_test]
// fn trace_gen_prove_verify() {
//     let program = Program::from_bytes(include_bytes!("is_prime_executable.json"), None).unwrap();
//     let trace_gen_output = trace_gen(program).unwrap();
//     let cairo_proof = prove(trace_gen_output.prover_input).unwrap();
//     let verdict = verify(cairo_proof);
//     assert!(verdict);
// }
