#![cfg(not(target_arch = "wasm32"))]

// use cairo_vm::types::program::Program;
use stwo_web_stark::{prove, trace_gen, verify};

#[test]
fn trace_gen_prove_verify() {
    // let program = Program::from_bytes(include_bytes!("is_prime_executable.json"), None)
    //     .expect("failed to load Program from bytes");
    let executable_json = include_str!("is_prime_executable.json");
    let executable = serde_json::from_str(executable_json).unwrap();
    let (program, _string_to_hint) =
        stwo_web_stark::utils::program_and_hints_from_executable(&executable);

    let trace_gen_output = trace_gen(program).expect("trace_gen failed");
    let cairo_proof = prove(trace_gen_output.prover_input).expect("prove failed");
    let verdict = verify(cairo_proof);

    assert!(verdict, "cairo proof verification failed");
}
