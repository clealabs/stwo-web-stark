pub mod utils;

// use cairo_air::{air::CairoProof, verifier::verify_cairo, PreProcessedTraceVariant};
// // use cairo_prove::prove::{prove as cairo_prove_prove, prover_input_from_runner};
// use cairo_vm::{
//     cairo_run::{self},
//     hint_processor::builtin_hint_processor::builtin_hint_processor_definition::BuiltinHintProcessor,
//     types::layout_name::LayoutName,
//     vm::{
//         errors::cairo_run_errors::CairoRunError,
//         // runners::{
//         //     cairo_pie::{
//         //         CairoPie, CairoPieAdditionalData, CairoPieMemory, CairoPieMetadata,
//         //         CairoPieVersion, /*     },
//         //     cairo_runner::{ExecutionResources, RunResources},
//         // }, */
//     },
// };
// // use serde::{Deserialize, Serialize};
// // use stwo_cairo_adapter::{vm_import::VmImportError, ProverInput};
// use stwo_cairo_prover::stwo_prover::core::{
//     fri::FriConfig,
//     pcs::PcsConfig,
//     vcs::blake2_merkle::{Blake2sMerkleChannel, Blake2sMerkleHasher},
// };
// use thiserror_no_std::Error;
// // use utils::{adapt_finished_runner, set_panic_hook};
// // use wasm_bindgen::prelude::*;

// use stwo_cairo_prover::{
//     cairo_air::{air::CairoProof, prove_cairo, verify_cairo, ProverConfig},
//     input::{plain::adapt_finished_runner, ProverInput},
// };
// use std::collections::HashMap;
use cairo_air::{air::CairoProof, verifier::verify_cairo, PreProcessedTraceVariant};
use cairo_vm::vm::runners::cairo_runner::CairoRunner;
// use cairo_lang_executable::executable::{EntryPointKind, Executable};
// use cairo_lang_runner::{build_hints_dict, Arg, CairoHintProcessor};
use cairo_vm::{
    cairo_run,
    hint_processor::builtin_hint_processor::builtin_hint_processor_definition::BuiltinHintProcessor,
    types::{layout_name::LayoutName, program::Program},
    vm::{errors::cairo_run_errors::CairoRunError, runners::cairo_runner::ExecutionResources},
};
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use stwo_cairo_adapter::{
    adapter::adapter,
    builtins::MemorySegmentAddresses,
    memory::{MemoryBuilder, MemoryConfig, MemoryEntry},
    vm_import::{adapt_to_stwo_input, RelocatedTraceEntry, VmImportError},
    ProverInput, PublicSegmentContext,
};
// use stwo_cairo_utils::vm_utils::VmError;
use stwo_cairo_prover::{prover::prove_cairo, stwo_prover::core::prover::ProvingError};
use stwo_cairo_prover::{
    // prover::default_prod_prover_parameters,
    stwo_prover::core::{
        fri::FriConfig,
        pcs::PcsConfig,
        vcs::blake2_merkle::{Blake2sMerkleChannel, Blake2sMerkleHasher},
    },
};
// use utils::set_panic_hook;
use thiserror_no_std::Error;
use wasm_bindgen::prelude::*;

use crate::utils::set_panic_hook;

///////////////////
///
// extern crate alloc;

// #[cfg(target_arch = "wasm32")]
// use lol_alloc::{FreeListAllocator, LockedAllocator};

// #[cfg(target_arch = "wasm32")]
// #[global_allocator]
// static ALLOCATOR: LockedAllocator<FreeListAllocator> =
//     LockedAllocator::new(FreeListAllocator::new());

///////////////////////

// pub struct TraceGenOutput {
//     pub execution_resources: ExecutionResources,
//     pub prover_input: ProverInput,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct TraceGenOutputJS {
//     execution_resources: String,
//     prover_input: String,
// }

// pub fn from_zip_archive<R: std::io::Read + std::io::Seek>(
//     mut zip_reader: zip::ZipArchive<R>,
// ) -> Result<CairoPie, std::io::Error> {
//     use std::io::Read;

//     let version = match zip_reader.by_name("version.json") {
//         Ok(version_buffer) => {
//             let reader = std::io::BufReader::new(version_buffer);
//             serde_json::from_reader(reader)?
//         }
//         Err(_) => CairoPieVersion { cairo_pie: () },
//     };

//     let reader = std::io::BufReader::new(zip_reader.by_name("metadata.json")?);
//     let metadata: CairoPieMetadata = serde_json::from_reader(reader)?;

//     let mut memory = vec![];
//     zip_reader.by_name("memory.bin")?.read_to_end(&mut memory)?;
//     let memory = CairoPieMemory::from_bytes(&memory)
//         .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::InvalidData))?;

//     let reader = std::io::BufReader::new(zip_reader.by_name("execution_resources.json")?);
//     let execution_resources: ExecutionResources = serde_json::from_reader(reader)?;

//     let reader = std::io::BufReader::new(zip_reader.by_name("additional_data.json")?);
//     let additional_data: CairoPieAdditionalData = serde_json::from_reader(reader)?;

//     Ok(CairoPie {
//         metadata,
//         memory,
//         execution_resources,
//         additional_data,
//         version,
//     })
// }

// #[wasm_bindgen]
// pub fn run_trace_gen(pie_zip_js: JsValue) -> Result<JsValue, JsValue> {
//     set_panic_hook();

//     let input: Vec<u8> = serde_wasm_bindgen::from_value(pie_zip_js)?;
//     let reader = std::io::Cursor::new(input);
//     let zip_archive = zip::ZipArchive::new(reader).unwrap();

//     let pie = from_zip_archive(zip_archive)
//         .map_err(|e| JsValue::from(format!("Failed to deserialize pie: {e}")))?;
//     let trace_gen_output =
//         trace_gen(pie).map_err(|e| JsValue::from(format!("Failed to generate trace: {e}")))?;
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

// #[wasm_bindgen]
// pub fn run_verify(proof_js: JsValue) -> Result<JsValue, JsValue> {
//     set_panic_hook();

//     let proof: CairoProof<Blake2sMerkleHasher> =
//         serde_json::from_str(&serde_wasm_bindgen::from_value::<String>(proof_js)?)
//             .map_err(|e| JsValue::from(format!("Failed to deserialize proof: {e}")))?;
//     let verdict = verify(proof);
//     Ok(serde_wasm_bindgen::to_value(&verdict)?)
// }

// pub fn trace_gen(pie: CairoPie) -> Result<TraceGenOutput, VmError> {
//     let cairo_run_config = cairo_run::CairoRunConfig {
//         trace_enabled: true,
//         relocate_mem: true,
//         layout: LayoutName::all_cairo_stwo,
//         ..Default::default()
//     };

//     let mut hint_processor = BuiltinHintProcessor::new(
//         Default::default(),
//         RunResources::new(pie.execution_resources.n_steps),
//     );
//     let cairo_runner_result =
//         cairo_run::cairo_run_pie(&pie, &cairo_run_config, &mut hint_processor);

//     let cairo_runner = match cairo_runner_result {
//         Ok(runner) => runner,
//         Err(error) => {
//             return Err(VmError::Runner(error));
//         }
//     };

//     Ok(TraceGenOutput {
//         execution_resources: cairo_runner
//             .get_execution_resources()
//             .map_err(|e| VmError::Runner(CairoRunError::Runner(e)))?,
//         prover_input: adapt_finished_runner(cairo_runner)?,
//     })
// }

// pub fn prove(prover_input: ProverInput) -> Result<CairoProof<Blake2sMerkleHasher>, ProvingError>
// {     prove_cairo::<Blake2sMerkleChannel>(
//         prover_input,
//         PcsConfig::default(),
//         PreProcessedTraceVariant::CanonicalWithoutPedersen,
//     )
// }

/////////////////////////
///
// // https://github.com/starkware-libs/cairo/blob/5cc466a6c7ca3e78a053e58911d567c2889444d2/crates/cairo-lang-runner/src/lib.rs#L148
// /// Builds hints_dict required in cairo_vm::types::program::Program from instructions.
// pub fn build_hints_dict(
//     hints: &[(usize, Vec<Hint>)],
// ) -> (HashMap<usize, Vec<HintParams>>, HashMap<String, Hint>) {
//     let mut hints_dict: HashMap<usize, Vec<HintParams>> = HashMap::new();
//     let mut string_to_hint: HashMap<String, Hint> = HashMap::new();

//     for (offset, offset_hints) in hints {
//         // Register hint with string for the hint processor.
//         for hint in offset_hints {
//             string_to_hint.insert(hint.representing_string(), hint.clone());
//         }
//         // Add hint, associated with the instruction offset.
//         hints_dict.insert(
//             *offset,
//             offset_hints.iter().map(hint_to_hint_params).collect(),
//         );
//     }
//     (hints_dict, string_to_hint)
// }

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

// // pub fn prove(cairo_prove_prove: String) -> CairoProof<Blake2sMerkleHasher> {
// //     let prover_input = prover_input_from_runner(executable_json);
// //     prove::<Blake2sMerkleChannel>(
// //         prover_input,
// //         secure_pcs_config(),
// //         PreProcessedTraceVariant::CanonicalWithoutPedersen,
// //     )
// // }

// pub fn verify(cairo_proof: CairoProof<Blake2sMerkleHasher>) -> bool {
//     verify_cairo::<Blake2sMerkleChannel>(
//         cairo_proof,
//         secure_pcs_config(),
//         PreProcessedTraceVariant::CanonicalWithoutPedersen,
//     )
//     .is_ok()
// }

// #[derive(Debug, Error)]
// pub enum VmError {
//     #[error("Failed to interact with the file system")]
//     IO(#[from] std::io::Error),
//     // #[error("The cairo program execution failed")]
//     // Runner(#[from] CairoRunError),
//     // #[error("The adapter execution failed")]
//     // Adapter(#[from] VmImportError),
// }

///////////////////////////
///
// pub struct TraceGenOutput {
//     pub execution_resources: ExecutionResources,
//     pub prover_input: ProverInput,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct TraceGenOutputJS {
//     execution_resources: String,
//     prover_input: String,
// }

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
pub fn run_verify(proof_js: JsValue) -> Result<JsValue, JsValue> {
    set_panic_hook();

    let proof: CairoProof<Blake2sMerkleHasher> =
        serde_json::from_str(&serde_wasm_bindgen::from_value::<String>(proof_js)?)
            .map_err(|e| JsValue::from(format!("Failed to deserialize proof: {e}")))?;
    let verdict = verify(proof);
    Ok(serde_wasm_bindgen::to_value(&verdict)?)
}

// pub fn trace_gen(program: Program) -> Result<TraceGenOutput, VmError> {
//     let cairo_run_config = cairo_run::CairoRunConfig {
//         trace_enabled: true,
//         relocate_mem: true,
//         layout: LayoutName::all_cairo,
//         proof_mode: true,
//         ..Default::default()
//     };

//     let mut hint_processor = BuiltinHintProcessor::new_empty();
//     let cairo_runner_result =
//         cairo_run::cairo_run_program(&program, &cairo_run_config, &mut hint_processor);

//     let cairo_runner = match cairo_runner_result {
//         Ok(runner) => runner,
//         Err(error) => {
//             return Err(VmError::Runner(error));
//         }
//     };

//     // Ok(TraceGenOutput {
//     //     execution_resources: cairo_runner
//     //         .get_execution_resources()
//     //         .map_err(|e| VmError::Runner(CairoRunError::Runner(e)))?,
//     //     prover_input: adapt_finished_runner(cairo_runner, false),
//     // })
//     Ok(TraceGenOutput {
//         execution_resources: cairo_runner
//             .get_execution_resources()
//             .map_err(|e| VmError::Runner(CairoRunError::Runner(e)))?,
//         prover_input: adapter(
//             &mut cairo_runner
//                 .get_prover_input_info()
//                 .expect("Failed to get prover input info from finished runner"),
//         )
//         .map_err(|e| VmError::Adapter(e))?,
//     })

//     // run_program_and_adapter(program)
// }

/// Exctracts artifacts from a finished cairo runner, to later be used for proving.
pub fn prover_input_from_runner(runner: &CairoRunner) -> ProverInput {
    let public_input = runner.get_air_public_input().unwrap();
    let addresses = public_input
        .public_memory
        .iter()
        .map(|entry| entry.address as u32)
        .collect::<Vec<_>>();
    let segments = public_input
        .memory_segments
        .iter()
        .map(|(&k, v)| {
            (
                k,
                MemorySegmentAddresses {
                    begin_addr: v.begin_addr,
                    stop_ptr: v.stop_ptr,
                },
            )
        })
        .collect::<HashMap<_, _>>();
    let trace = runner
        .relocated_trace
        .as_ref()
        .unwrap()
        .iter()
        .map(|x| RelocatedTraceEntry {
            ap: x.ap,
            fp: x.fp,
            pc: x.pc,
        })
        .collect::<Vec<_>>();
    let mem = runner
        .relocated_memory
        .iter()
        .enumerate()
        .filter_map(|(i, x)| {
            x.as_ref().map(|value| MemoryEntry {
                address: i as u64,
                value: unsafe { std::mem::transmute::<[u8; 32], [u32; 8]>(value.to_bytes_le()) },
            })
        });
    let mem = MemoryBuilder::from_iter(MemoryConfig::default(), mem);
    let main_args = runner
        .get_program()
        .iter_builtins()
        .copied()
        .collect::<Vec<_>>();
    let public_segment_context = PublicSegmentContext::new(&main_args);

    println!("Generating input for the prover...");
    let input =
        adapt_to_stwo_input(&trace, mem, addresses, &segments, public_segment_context).unwrap();
    println!("Input for the prover generated successfully.");
    // debug!(
    //     "State transitions: {}",
    //     input.state_transitions.casm_states_by_opcode
    // );
    // debug!("Builtins: {:#?}", input.builtins_segments.get_counts());
    input
}

pub fn prove(prover_input: ProverInput) -> Result<CairoProof<Blake2sMerkleHasher>, ProvingError> {
    prove_cairo::<Blake2sMerkleChannel>(
        prover_input,
        secure_pcs_config(),
        PreProcessedTraceVariant::CanonicalWithoutPedersen,
        // default_prod_prover_parameters().pcs_config,
        // default_prod_prover_parameters().preprocessed_trace,
    )
}

pub fn verify(cairo_proof: CairoProof<Blake2sMerkleHasher>) -> bool {
    verify_cairo::<Blake2sMerkleChannel>(
        cairo_proof,
        secure_pcs_config(),
        PreProcessedTraceVariant::CanonicalWithoutPedersen,
    )
    .is_ok()
}

#[derive(Debug, Error)]
pub enum VmError {
    #[error("Failed to interact with the file system")]
    IO(#[from] std::io::Error),
    #[error("The cairo program execution failed")]
    Runner(#[from] CairoRunError),
    #[error("The adapter execution failed")]
    Adapter(#[from] VmImportError),
}
