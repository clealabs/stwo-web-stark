use cairo_air::{verifier::verify_cairo, CairoProof, PreProcessedTraceVariant};
use cairo_lang_runner::Arg;
use cairo_prove::{
    execute::execute,
    prove::{prove as cairo_prove, prover_input_from_runner},
};
use stwo_cairo_prover::stwo_prover::core::{
    fri::FriConfig,
    pcs::PcsConfig,
    vcs::blake2_merkle::{Blake2sMerkleChannel, Blake2sMerkleHasher},
};
use wasm_bindgen::prelude::*;

fn secure_pcs_config() -> PcsConfig {
    PcsConfig {
        pow_bits: 26,
        fri_config: FriConfig {
            log_last_layer_degree_bound: 0,
            log_blowup_factor: 1,
            n_queries: 70,
        },
    }
}

// #[wasm_bindgen]
// pub fn run_trace_gen(program_content_js: JsValue) -> Result<JsValue, JsValue> {
//     set_panic_hook();

//     let program = Program::from_bytes(
//         serde_wasm_bindgen::from_value::<String>(program_content_js)?.as_bytes(),
//         None,
//     )
//     .map_err(|e| JsValue::from(format!("Failed to deserialize program: {e}")))?;
//     let trace_gen_output =
//         trace_gen(program).map_err(|e| JsValue::from(format!("Failed to generate trace: {e}")))?;
//     Ok(serde_wasm_bindgen::to_value(&TraceGenOutputJS {
//         prover_input: serde_json::to_string(&trace_gen_output.prover_input)
//             .map_err(|e| JsValue::from(format!("Failed to serialize prover input: {e}")))?,
//         execution_resources: serde_json::to_string(&trace_gen_output.execution_resources)
//             .map_err(|e| JsValue::from(format!("Failed to serialize execution resources:
// {e}")))?,     })?)
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

#[wasm_bindgen]
pub fn run_verify(proof_js: JsValue, with_pedersen_js: JsValue) -> Result<JsValue, JsValue> {
    // set_panic_hook();

    let proof: CairoProof<Blake2sMerkleHasher> =
        serde_json::from_str(&serde_wasm_bindgen::from_value::<String>(proof_js)?)
            .map_err(|e| JsValue::from(format!("Failed to deserialize proof: {e}")))?;
    let with_pedersen: bool = serde_wasm_bindgen::from_value(with_pedersen_js)?;
    let verdict = verify(proof, with_pedersen);
    Ok(serde_wasm_bindgen::to_value(&verdict)?)
}

pub fn execute_and_prove(executable_json: &str, args: Vec<Arg>) -> CairoProof<Blake2sMerkleHasher> {
    // Execute.
    let executable = serde_json::from_str(executable_json).expect("Failed to read executable");
    let runner = execute(executable, args);

    // Prove.
    let prover_input = prover_input_from_runner(&runner);
    cairo_prove(prover_input, secure_pcs_config())
}

pub fn verify(cairo_proof: CairoProof<Blake2sMerkleHasher>, with_pedersen: bool) -> bool {
    let preprocessed_trace = match with_pedersen {
        true => PreProcessedTraceVariant::Canonical,
        false => PreProcessedTraceVariant::CanonicalWithoutPedersen,
    };
    verify_cairo::<Blake2sMerkleChannel>(cairo_proof, secure_pcs_config(), preprocessed_trace)
        .is_ok()
}
