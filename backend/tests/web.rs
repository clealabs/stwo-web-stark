//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
// use cairo_vm::types::program::Program;
// use stwo_web_stark::{prove, trace_gen, verify};
use cairo_lang_runner::Arg;
use cairo_vm::Felt252;
use stwo_web_stark::{execute_and_prove, verify};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn trace_gen_prove_verify() {
    let executable_json = include_str!("is_prime_executable.json");
    let args = vec![Arg::Value(Felt252::from(7))];
    let cairo_proof = execute_and_prove(executable_json, args);
    let verdict = verify(cairo_proof, false);
    assert!(verdict, "cairo proof verification failed");
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
