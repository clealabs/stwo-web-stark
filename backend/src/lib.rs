// this file is analogous to https://github.com/clealabs/stwo-cairo/blob/fix-wasm-standalone/cairo-prove/src/main.rs

use cairo_air::{verifier::verify_cairo, CairoProof, PreProcessedTraceVariant};
use cairo_lang_runner::Arg;
use cairo_prove::{
    execute::execute,
    prove::{prove as cairo_prove, prover_input_from_runner},
};
use cairo_vm::Felt252;
use stwo_cairo_adapter::ProverInput;
use stwo_cairo_prover::{
    prover::prove_cairo,
    stwo_prover::core::{
        fri::FriConfig,
        pcs::PcsConfig,
        prover::ProvingError,
        vcs::blake2_merkle::{Blake2sMerkleChannel, Blake2sMerkleHasher},
    },
};
// use wasm_bindgen::prelude::*;

#[cfg(feature = "console_error_panic_hook")]
pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

// https://docs.rs/getrandom/0.3.3/getrandom/#custom-backend
use getrandom::Error;
#[no_mangle]
unsafe extern "Rust" fn __getrandom_v03_custom(dest: *mut u8, len: usize) -> Result<(), Error> {
    if len == 0 {
        return Ok(());
    }
    if dest.is_null() {
        return Err(Error::UNSUPPORTED);
    }

    let buf = core::slice::from_raw_parts_mut(dest, len);
    let mut s: u64 = 0x1234_5678_90ab_cdef; // fixed seed for deterministic output

    for b in buf.iter_mut() {
        // tiny xorshift; update state and take low byte
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        *b = (s & 0xFF) as u8;
    }
    Ok(())
}

pub fn secure_pcs_config() -> PcsConfig {
    PcsConfig {
        pow_bits: 26,
        fri_config: FriConfig {
            log_last_layer_degree_bound: 0,
            log_blowup_factor: 1,
            n_queries: 70,
        },
    }
}

// /// WARNING: this uses too much memory for wasm
// #[wasm_bindgen]
// pub fn run_execute_and_prove(
//     executable_json_js: JsValue,
//     args_js: JsValue,
// ) -> Result<JsValue, JsValue> {
//     set_panic_hook();

//     let executable_json: &str = &serde_wasm_bindgen::from_value::<String>(executable_json_js)?;
//     let args_raw: Vec<i128> = serde_wasm_bindgen::from_value(args_js)?;

//     let args = args_raw
//         .into_iter()
//         .map(|arg| Arg::Value(Felt252::from(arg)))
//         .collect();

//     // let proof = execute_and_prove(executable_json, args, secure_pcs_config());
//     let proof = execute_and_prove(executable_json, args, PcsConfig::default());
//     Ok(serde_wasm_bindgen::to_value(&proof)?)
// }

// #[wasm_bindgen]
// pub fn run_trace_gen(executable_json_js: JsValue, args_js: JsValue) -> Result<JsValue, JsValue> {
//     set_panic_hook();

//     let executable_json: &str = &serde_wasm_bindgen::from_value::<String>(executable_json_js)?;
//     let args_raw: Vec<i128> = serde_wasm_bindgen::from_value(args_js)?;

//     let args = args_raw
//         .into_iter()
//         .map(|arg| Arg::Value(Felt252::from(arg)))
//         .collect();

//     let prover_input = trace_gen(executable_json, args);
//     let prover_input_json = serde_json::to_string(&prover_input)
//         .map_err(|e| JsValue::from(format!("Failed to serialize prover input: {e}")))?;
//     Ok(serde_wasm_bindgen::to_value(&prover_input_json)?)
// }

// #[wasm_bindgen]
// pub fn run_prove(prover_input_js: JsValue) -> Result<JsValue, JsValue> {
//     set_panic_hook();

//     let prover_input: ProverInput =
//         serde_json::from_str(&serde_wasm_bindgen::from_value::<String>(prover_input_js)?)
//             .map_err(|e| JsValue::from(format!("Failed to deserialize prover input: {e}")))?;
//     let proof =
//         prove(prover_input).map_err(|e| JsValue::from(format!("Failed to generate proof:
// {e}")))?;     Ok(serde_wasm_bindgen::to_value(
//         &serde_json::to_string(&proof)
//             .map_err(|e| JsValue::from(format!("Failed to serialize proof: {e}")))?,
//     )?)
// }

// #[wasm_bindgen]
// pub fn run_verify(proof_js: JsValue, with_pedersen_js: JsValue) -> Result<JsValue, JsValue> {
//     set_panic_hook();

//     let proof: CairoProof<Blake2sMerkleHasher> =
//         serde_json::from_str(&serde_wasm_bindgen::from_value::<String>(proof_js)?)
//             .map_err(|e| JsValue::from(format!("Failed to deserialize proof: {e}")))?;
//     let with_pedersen: bool = serde_wasm_bindgen::from_value::<bool>(with_pedersen_js)?;
//     let verdict = verify(proof, with_pedersen);
//     Ok(serde_wasm_bindgen::to_value(&verdict)?)
// }

/// WARNING: this uses too much memory for wasm
pub fn execute_and_prove(
    executable_json: &str,
    args: Vec<Arg>,
    pcs_config: PcsConfig,
) -> CairoProof<Blake2sMerkleHasher> {
    // Execute.
    let executable = serde_json::from_str(executable_json).expect("Failed to read executable");
    let runner = execute(executable, args);
    // Prove.
    let prover_input = prover_input_from_runner(&runner);
    cairo_prove(prover_input, pcs_config)
}

pub fn trace_gen(executable_json: &str, args: Vec<Arg>) -> ProverInput {
    let executable = serde_json::from_str(executable_json).expect("Failed to read executable");
    let runner = execute(executable, args);
    prover_input_from_runner(&runner)
}

pub fn prove(prover_input: ProverInput) -> Result<CairoProof<Blake2sMerkleHasher>, ProvingError> {
    prove_cairo::<Blake2sMerkleChannel>(
        prover_input,
        PcsConfig::default(),
        PreProcessedTraceVariant::CanonicalWithoutPedersen,
    )
}

pub fn verify(cairo_proof: CairoProof<Blake2sMerkleHasher>, with_pedersen: bool) -> bool {
    let preprocessed_trace = match with_pedersen {
        true => PreProcessedTraceVariant::Canonical,
        false => PreProcessedTraceVariant::CanonicalWithoutPedersen,
    };
    verify_cairo::<Blake2sMerkleChannel>(cairo_proof, secure_pcs_config(), preprocessed_trace)
        .is_ok()
}
