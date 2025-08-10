use std::collections::HashMap;

use cairo_lang_casm::{assembler::AssembledCairoProgram, hints::Hint};
use cairo_vm::{
    cairo_run::{cairo_run_program, CairoRunConfig},
    serde::deserialize_program::{ApTracking, FlowTrackingData, HintParams},
    types::{
        builtin_name::BuiltinName, layout_name::LayoutName, program::Program,
        relocatable::MaybeRelocatable,
    },
    vm::runners::cairo_runner::CairoRunner,
};
// use cairo_lang_executable::executable::{EntryPointKind, Executable};
// use cairo_lang_runner::{Arg, CairoHintProcessor, build_hints_dict};
use cairo_vm::{hint_processor, Felt252};
use serde::{Deserialize, Serialize};
// source: https://github.com/starkware-libs/cairo/blob/5cc466a6c7ca3e78a053e58911d567c2889444d2/crates/cairo-lang-executable/src/executable.rs

/// Structure to hold the executable representation of a program.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Executable {
    /// The bytecode of the program.
    pub program: AssembledCairoProgram,
    /// The available entrypoints for the program.
    pub entrypoints: Vec<ExecutableEntryPoint>,
}

/// Information about an executable entrypoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutableEntryPoint {
    /// The used builtins of the function.
    pub builtins: Vec<BuiltinName>,
    /// The offset of the entrypoint in the bytecode.
    pub offset: usize,
    /// The kind of the entrypoint.
    pub kind: EntryPointKind,
}

/// The kind of an entrypoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntryPointKind {
    /// Entrypoint is for running it using a bootloader.
    ///
    /// The entrypoint is a function, ending with a `ret`, expecting the builtins as its
    /// parameters.
    Bootloader,
    /// Entrypoint is for running this executable as a standalone program.
    ///
    /// The entrypoint starts with `ap += <builtins.len()>` and expected the builtins to be
    /// injected there, and ends with an infinite loop.
    Standalone,
}

// source: https://github.com/starkware-libs/cairo/blob/5cc466a6c7ca3e78a053e58911d567c2889444d2/crates/cairo-lang-runner/src/casm_run/mod.rs#L58C1-L68C2
/// Convert a Hint to the cairo-vm class HintParams by canonically serializing it to a string.
pub fn hint_to_hint_params(hint: &Hint) -> HintParams {
    HintParams {
        code: hint.representing_string(),
        accessible_scopes: vec![],
        flow_tracking_data: FlowTrackingData {
            ap_tracking: ApTracking::new(),
            reference_ids: Default::default(),
        },
    }
}

// source: https://github.com/starkware-libs/cairo/blob/5cc466a6c7ca3e78a053e58911d567c2889444d2/crates/cairo-lang-runner/src/lib.rs#L149
/// Builds hints_dict required in cairo_vm::types::program::Program from instructions.
pub fn build_hints_dict(
    hints: &[(usize, Vec<Hint>)],
) -> (HashMap<usize, Vec<HintParams>>, HashMap<String, Hint>) {
    let mut hints_dict: HashMap<usize, Vec<HintParams>> = HashMap::new();
    let mut string_to_hint: HashMap<String, Hint> = HashMap::new();

    for (offset, offset_hints) in hints {
        // Register hint with string for the hint processor.
        for hint in offset_hints {
            string_to_hint.insert(hint.representing_string(), hint.clone());
        }
        // Add hint, associated with the instruction offset.
        hints_dict.insert(
            *offset,
            offset_hints.iter().map(hint_to_hint_params).collect(),
        );
    }
    (hints_dict, string_to_hint)
}

// source: https://github.com/clealabs/stwo-cairo/blob/73b578a38ee2f0dd3835881144a67f55ddd5e747/cairo-prove/src/execute.rs#L51
pub fn program_and_hints_from_executable(
    executable: &Executable,
) -> (Program, HashMap<String, Hint>) {
    let data: Vec<MaybeRelocatable> = executable
        .program
        .bytecode
        .iter()
        .map(Felt252::from)
        .map(MaybeRelocatable::from)
        .collect();
    let (hints, string_to_hint) = build_hints_dict(&executable.program.hints);
    let entrypoint = executable
        .entrypoints
        .iter()
        .find(|e| matches!(e.kind, EntryPointKind::Standalone))
        .expect("Failed to find entrypoint");
    let program = Program::new_for_proof(
        entrypoint.builtins.clone(),
        data,
        entrypoint.offset,
        entrypoint.offset + 4,
        hints.into_iter().collect(),
        Default::default(),
        Default::default(),
        vec![],
        None,
    )
    .unwrap();
    (program, string_to_hint)
}

// source: https://github.com/starkware-libs/cairo/blob/5cc466a6c7ca3e78a053e58911d567c2889444d2/crates/cairo-lang-runner/src/lib.rs#L127
/// An argument to a sierra function run,
#[derive(Debug)]
pub enum Arg {
    Value(Felt252),
    Array(Vec<Arg>),
}
impl Arg {
    /// Returns the size of the argument in the vm.
    pub fn size(&self) -> usize {
        match self {
            Self::Value(_) => 1,
            Self::Array(_) => 2,
        }
    }
}
impl From<Felt252> for Arg {
    fn from(value: Felt252) -> Self {
        Self::Value(value)
    }
}

// source: https://github.com/starkware-libs/cairo/blob/5cc466a6c7ca3e78a053e58911d567c2889444d2/crates/cairo-lang-runner/src/casm_run/mod.rs#L87
/// HintProcessor for Cairo compiler hints. // TODO: use this
// pub struct CairoHintProcessor<'a> {
//     /// The Cairo runner.
//     pub runner: Option<&'a SierraCasmRunner>,
//     /// The user arguments for the run.
//     ///
//     /// We have a vector of the arguments per parameter, as a parameter type may be composed of
//     /// several user args.
//     pub user_args: Vec<Vec<Arg>>,
//     /// A mapping from a string that represents a hint to the hint object.
//     pub string_to_hint: HashMap<String, Hint>,
//     /// The starknet state.
//     pub starknet_state: StarknetState,
//     /// Maintains the resources of the run.
//     pub run_resources: RunResources,
//     /// Resources used during syscalls - does not include resources used during the current VM
// run.     /// At the end of the run - adding both would result in the actual expected resource
// usage.     pub syscalls_used_resources: StarknetExecutionResources,
//     /// Avoid allocating memory segments so finalization of segment arena may not occur.
//     pub no_temporary_segments: bool,
//     /// A set of markers created by the run.
//     pub markers: Vec<Vec<Felt252>>,
//     /// The traceback set by a panic trace hint call.
//     pub panic_traceback: Vec<(Relocatable, Relocatable)>,
// }

/// Executes a Cairo program and returns a `CairoRunner` that can be used to generate artifacts for
/// the prover.
pub fn execute(executable: Executable, _args: Vec<Arg>) -> CairoRunner {
    let (program, _string_to_hint) = program_and_hints_from_executable(&executable);

    // let mut dyn hint_processor = HintProcessor::new();
    // {
    //     runner: None,
    //     user_args: vec![vec![Arg::Array(args)]],
    //     string_to_hint,
    //     starknet_state: Default::default(),
    //     run_resources: Default::default(),
    //     syscalls_used_resources: Default::default(),
    //     no_temporary_segments: false,
    //     markers: Default::default(),
    //     panic_traceback: Default::default(),
    // };

    // let mut hint_processor =
    // hint_processor::builtin_hint_processor::builtin_hint_processor_definition::BuiltinHintProcessor::new(string_to_hint,
    // Default::default());
    let mut hint_processor = hint_processor::builtin_hint_processor::builtin_hint_processor_definition::BuiltinHintProcessor::new_empty();
    // let mut hint_processor = hint_processor::hint_processor_definition::HintProcessor::new()

    let cairo_run_config = CairoRunConfig {
        trace_enabled: true,
        relocate_mem: true,
        layout: LayoutName::all_cairo_stwo,
        secure_run: None,
        allow_missing_builtins: None,
        dynamic_layout_params: None,
        disable_trace_padding: true,
        proof_mode: true,
        ..Default::default()
    };

    // info!("Executing program...");
    let runner = cairo_run_program(&program, &cairo_run_config, &mut hint_processor)
        .expect("Failed to execute program");
    // info!("Program executed successfully.");
    runner
}

// use cairo_vm::vm::runners::cairo_runner::CairoRunner;
// use itertools::Itertools;
// use stwo_cairo_adapter::{
//     builtins::MemorySegmentAddresses,
//     memory::{MemoryBuilder, MemoryConfig, MemoryEntry},
//     vm_import::{adapt_to_stwo_input, RelocatedTraceEntry, VmImportError},
//     HashMap, ProverInput, PublicSegmentContext,
// };

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

// pub fn adapt_finished_runner(runner: CairoRunner) -> Result<ProverInput, VmImportError> {
//     let memory_iter = runner
//         .relocated_memory
//         .iter()
//         .enumerate()
//         .filter_map(|(i, v)| {
//             v.map(|v| MemoryEntry {
//                 address: i as u64,
//                 value: bytemuck::cast(v.to_bytes_le()),
//             })
//         });

//     let public_input = runner.get_air_public_input()?;

//     let trace_iter = match runner.relocated_trace {
//         Some(ref trace) => trace.iter().map(|t| RelocatedTraceEntry {
//             ap: t.ap,
//             pc: t.pc,
//             fp: t.fp,
//         }),
//         None => return Err(VmImportError::TraceNotRelocated),
//     };

//     let memory_segments: &HashMap<&str, MemorySegmentAddresses> = &public_input
//         .memory_segments
//         .into_iter()
//         .map(|(k, v)| {
//             (
//                 k,
//                 MemorySegmentAddresses {
//                     begin_addr: v.begin_addr,
//                     stop_ptr: v.stop_ptr,
//                 },
//             )
//         })
//         .collect();

//     let public_memory_addresses = public_input
//         .public_memory
//         .iter()
//         .map(|s| s.address as u32)
//         .collect_vec();

//     // TODO(spapini): Add output builtin to public memory.
//     adapt_to_stwo_input(
//         &trace_iter.collect_vec(),
//         MemoryBuilder::from_iter(MemoryConfig::default(), memory_iter),
//         public_memory_addresses,
//         memory_segments,
//         PublicSegmentContext::bootloader_context(),
//     )
// }
